use std::collections::HashMap;
use std::sync::{Arc, mpsc};
use slint::{Model, ModelRc, VecModel, Weak};

use crate::client::ProntoClient;

pub(crate) mod client;
mod secret;
pub(crate) mod storage;

use crate::secret::{*};

slint::include_modules!();

static PRONTO_BASE_URL: &str = "https://stanfordohs.pronto.io/api/";

#[derive(Copy, Clone, Debug)]
enum WorkerTasks {
    ChangeChannel(u64),
    ScrollChannel(u64, u64),
}

fn net_worker(app: Weak<AppWindow>, rx: mpsc::Receiver<WorkerTasks>) {
    let client = Arc::new(ProntoClient::new(PRONTO_BASE_URL.to_string(), PRONTO_SESSION, PRONTO_API_TOKEN, PAACT_2245_5302428));
    loop {
        match rx.recv() {
            Ok(WorkerTasks::ChangeChannel(channel_id)) => {
                let history = client.get_bubble_history(channel_id, None);
                for message in history.messages.iter() {
                    for image in message.message_media.iter() {
                        println!("{:?}", image);
                        storage::load_image(image.url.clone());
                    }
                }

                app.upgrade_in_event_loop(move |ui| {
                    let mut ui_messages = Vec::new();
                    for message in history.messages.clone().into_iter().rev() {
                        ui_messages.push(message.into())
                    }
                    ui.set_messages(ModelRc::new(VecModel::from(ui_messages)));
                    if history.messages.len() > 0 {
                        ui.set_top_msg_id(history.messages.last().unwrap().id as i32); // TODO: this is confusing
                    } else {
                        ui.set_top_msg_id(0);
                    }
                    ui.set_channel_id(channel_id as i32);
                    ui.set_viewport_y(ui.get_visible_height() - ui.get_viewport_height());
                }).unwrap();
            },
            Ok(WorkerTasks::ScrollChannel(channel_id, top_msg_id)) => {
                let history = client.get_bubble_history(channel_id, Some(top_msg_id));
                for message in history.messages.iter() {
                    for image in message.message_media.iter() {
                        storage::load_image(image.url.clone());
                    }
                }

                app.upgrade_in_event_loop(move |ui| {
                    let mut reversed: Vec<Message> = history.messages.clone().into_iter().rev()
                        .map(|msg| msg.into()).collect();
                    let mut ui_messages: Vec<Message> = ui.get_messages().iter().collect();
                    reversed.append(&mut ui_messages);
                    ui.set_messages(ModelRc::new(VecModel::from(reversed.clone())));
                    ui.set_top_msg_id(reversed.first().map(|msg| msg.id).unwrap_or(0)); // TODO: this is confusing
                }).unwrap();
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let client = Arc::new(ProntoClient::new(PRONTO_BASE_URL.to_string(), PRONTO_SESSION, PRONTO_API_TOKEN, PAACT_2245_5302428));
    let ui = AppWindow::new()?;
    let (tx, rx) = mpsc::channel::<WorkerTasks>();

    std::thread::spawn({
        let ui_handle = ui.as_weak();
        move || {
            net_worker(ui_handle, rx);
        }});


    let user_info = client.get_user_info();
    let channels = client.get_bubble_list();

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
    ui.set_fullname(user_info.user.fullname.into());

    let mut ui_channels = Vec::new();
    for ((category_id, category_name), ui_channels_group) in ui_channels_groups.clone().into_iter() {
        ui_channels.push(ChannelGroup {
            id: category_id as i32,
            title: category_name.into(),
            channels: ModelRc::new(VecModel::from(ui_channels_group))
        });
    }


    let first_channel = ui_channels_groups.iter().next().unwrap().1.first().unwrap();
    tx.send(WorkerTasks::ChangeChannel(first_channel.id as u64)).unwrap();

    ui.set_channels(ModelRc::new(VecModel::from(ui_channels)));

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
            if ui.get_viewport_y() > -20.0 { // TODO: Do not hardcode
                let top_msg_id = ui.get_top_msg_id();
                let channel_id = ui.get_channel_id();
                tx.send(WorkerTasks::ScrollChannel(channel_id as u64, top_msg_id as u64)).unwrap();
            }
        }
    });

    ui.run()
}
