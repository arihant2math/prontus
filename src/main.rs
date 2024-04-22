use std::sync::{Arc, mpsc};
use slint::{Model, ModelRc, VecModel, Weak};

use crate::client::ProntoClient;

mod client;
mod secret;

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
                let mut ui_messages = Vec::new();
                for message in history.messages.iter().rev() {
                    ui_messages.push(Message {
                        id: message.id as i32,
                        content: message.message.clone().into(),
                        user: message.user.fullname.clone().into()
                    })
                }
                app.upgrade_in_event_loop(move |ui| {
                    ui.set_messages(ModelRc::new(VecModel::from(ui_messages)));
                    if history.messages.len() > 0 {
                        ui.set_top_msg_id(history.messages.last().unwrap().id as i32); // TODO: this is confusing
                    } else {
                        ui.set_top_msg_id(0);
                    }
                    ui.set_channel_id(channel_id as i32);
                }).unwrap();
            },
            Ok(WorkerTasks::ScrollChannel(channel_id, top_msg_id)) => {
                let history = client.get_bubble_history(channel_id as u64, Some(top_msg_id as u64));
                let mut reversed: Vec<Message> = history.messages.clone().into_iter().rev().map(|msg| {
                    Message {
                        id: msg.id as i32,
                        content: msg.message.clone().into(),
                        user: msg.user.fullname.clone().into()
                    }
                }).collect();
                app.upgrade_in_event_loop(move |ui| {
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

    let mut ui_channels = Vec::new();
    for channel in channels.bubbles {
        ui_channels.push(Channel {
            id: channel.id as i32,
            title: channel.title.into(),
            unread: false,
            notifications: 0
        });
    }
    ui.set_fullname(user_info.user.fullname.into());


    let first_channel = ui_channels.iter().nth(0).unwrap();

    tx.send(WorkerTasks::ChangeChannel(first_channel.id as u64)).unwrap();

    ui.set_channels(ModelRc::new(VecModel::from(ui_channels)));

    ui.on_setChannel({
        let ui_handle = ui.as_weak();
        let client = Arc::clone(&client);
        let tx = tx.clone();
        move || {
            let ui = ui_handle.unwrap();
            let channel_index = ui.get_current_sidebar_item_id();
            let channels = client.get_bubble_list();
            let channel = channels.bubbles.iter().nth(channel_index as usize).unwrap();
            tx.send(WorkerTasks::ChangeChannel(channel.id)).unwrap();
        }
    });

    ui.on_scrollChannel({
        let ui_handle = ui.as_weak();
        let client = Arc::clone(&client);
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
