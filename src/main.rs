use std::collections::HashMap;
use std::sync::{Arc, mpsc};
use slint::{Image, Model, ModelRc, Rgba8Pixel, SharedPixelBuffer, VecModel, Weak};
use tokio_tungstenite::connect_async;
use futures_util::StreamExt;

use crate::client::ProntoClient;

pub(crate) mod client;
mod secret;
pub(crate) mod storage;
mod websocket;

use crate::secret::{*};

slint::include_modules!();

static PRONTO_BASE_URL: &str = "https://stanfordohs.pronto.io/api/";

#[derive(Clone, Debug)]
enum WorkerTasks {
    ChangeChannel(u64),
    ScrollChannel(u64, u64),
    AddMessage(u64, Option<u64>, String)
}

#[derive(Clone, Debug)]
enum WebsocketTasks {
    ChangeChannel(u64)
}

async fn websocket_worker(ui: Weak<AppWindow>, rx: mpsc::Receiver<WebsocketTasks>) {
    let url = url::Url::parse("wss://ws-mt1.pusher.com/app/f44139496d9b75f37d27?protocol=7&client=js&version=8.3.0&flash=false").unwrap();
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");

    let (write, read) = ws_stream.split();
    // TODO
}

#[tokio::main]
async fn net_worker(app: Weak<AppWindow>, rx: mpsc::Receiver<WorkerTasks>) {
    let client = Arc::new(ProntoClient::new(PRONTO_BASE_URL.to_string(), PRONTO_SESSION, PRONTO_API_TOKEN, PAACT_2245_5302428));

    let channels = client.get_bubble_list().await;
    app.upgrade_in_event_loop(move |ui| {
        // TODO: Sort by priority
        let mut ui_channels_groups = HashMap::new();
        for (count, channel) in channels.bubbles.iter().enumerate() {
            let category_name = &channel.category.clone().map(|c| c.title).unwrap_or_else(|| {
                if channel.isdm {
                    "Direct Messages".to_string()
                } else {
                    "Uncategorized".to_string()
                }
            });
            let category_id = &channel.category.clone().map(|c| c.id).unwrap_or(0);
            let key = (*category_id, category_name.to_string());

            if !ui_channels_groups.contains_key(&key) {
                ui_channels_groups.insert(key.clone(), Vec::new());
            }
            let ui_channel = Channel {
                id: channel.id as i32,
                title: channel.title.clone().into(),
                unread: false,
                notifications: channels.stats[count].unread as i32
            };
            ui_channels_groups.get_mut(&key).unwrap().push(ui_channel);
        }
        let mut ui_channels = Vec::new();
        for ((category_id, category_name), ui_channels_group) in ui_channels_groups.clone().into_iter() {
            ui_channels.push(ChannelGroup {
                id: category_id as i32,
                title: category_name.into(),
                channels: ModelRc::new(VecModel::from(ui_channels_group))
            });
        }
        ui.set_channels(ModelRc::new(VecModel::from(ui_channels)));
    }).unwrap();


    let user_info = client.get_user_info().await;
    let user_id = user_info.user.id;
    loop {
        let loop_client = Arc::clone(&client);
        match rx.recv() {
            // TODO: have a centralized message storage variable for time saving
            // TODO: store users in memory and share that with the websocket worker and ui thread.
            Ok(WorkerTasks::ChangeChannel(channel_id)) => {
                let history = loop_client.get_bubble_history(channel_id, None).await;
                app.upgrade_in_event_loop(move |ui| {
                    let mut ui_messages = Vec::new();
                    for message in history.messages.clone().into_iter().rev() {
                        ui_messages.push(message.to_slint(&history.parentmessages));
                    }
                    ui.set_messages(ModelRc::new(VecModel::from(ui_messages)));
                    if history.messages.len() > 0 {
                        ui.set_top_msg_id(history.messages.last().unwrap().id as i32);
                    } else {
                        ui.set_top_msg_id(0);
                    }
                    ui.set_message("".to_string().into());
                    ui.set_channel_id(channel_id as i32);
                    ui.set_viewport_y(ui.get_visible_height() - ui.get_viewport_height());
                }).unwrap();
            },
            Ok(WorkerTasks::ScrollChannel(channel_id, top_msg_id)) => {
                let history = loop_client.get_bubble_history(channel_id, Some(top_msg_id)).await;
                app.upgrade_in_event_loop(move |ui| {
                    let messages = ui.get_messages();
                    let messages = messages.as_any().downcast_ref::<VecModel<Message>>().unwrap();
                    for message in history.messages.clone().into_iter() {
                        messages.insert(0, message.to_slint(&history.parentmessages));
                    }
                    ui.set_top_msg_id(history.messages.last().map(|msg| msg.id).unwrap_or(0) as i32);
                }).unwrap();
            }
            Ok(WorkerTasks::AddMessage(channel_id, parent_id, message)) => {
                let message = client.post_message(user_id, channel_id, message, parent_id).await;
                app.upgrade_in_event_loop(move |ui| {
                    let mut messages = ui.get_messages();
                    let messages = messages.as_any().downcast_ref::<VecModel<Message>>().unwrap();
                    messages.push(message.message.clone().to_slint(&Vec::new()));
                }).unwrap();
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let (tx, rx) = mpsc::channel::<WorkerTasks>();

    std::thread::spawn({
        let ui_handle = ui.as_weak();
        move || {
            net_worker(ui_handle, rx);
        }
    });


    ui.on_setChannel({
        let tx = tx.clone();
        let ui_handle = ui.as_weak();
        move |channel_id| { // TODO: channel id is broken
            let ui = ui_handle.unwrap();
            tx.send(WorkerTasks::ChangeChannel(ui.get_current_sidebar_item_id() as u64)).unwrap();
        }
    });

    ui.on_scrollChannel({
        let ui_handle = ui.as_weak();
        let tx = tx.clone();
        move || {
            let ui = ui_handle.unwrap();
            println!("{} {} {}", ui.get_visible_height(), ui.get_viewport_y(), ui.get_viewport_height());
            if ui.get_viewport_y() > -100.0 { // TODO: Do not hardcode
                let top_msg_id = ui.get_top_msg_id();
                let channel_id = ui.get_channel_id();
                tx.send(WorkerTasks::ScrollChannel(channel_id as u64, top_msg_id as u64)).unwrap();
            }
        }
    });

    ui.on_sendMessage({
        let ui_handle = ui.as_weak();
        let tx = tx.clone();
        move || { // TODO: preliminary message appending
            let ui = ui_handle.unwrap();
            tx.send(WorkerTasks::AddMessage(ui.get_channel_id() as u64, None, ui.get_message().to_string())).unwrap();
            ui.set_message("".to_string().into());
        }
    });

    ui.on_openLink({
        move |link| {
            open::that(&link.to_string()).unwrap();
        }
    });


    // let image = storage::load_image(Arc::clone(&client), image.url.clone()).await;
    // let buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
    //     image.as_raw(),
    //     image.width(),
    //     image.height(),
    // );
    // let slint_image = Image::from_rgba8(buffer);

    ui.run()
}
