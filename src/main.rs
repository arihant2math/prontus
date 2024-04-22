use std::sync::Arc;
use slint::{Model, ModelRc, VecModel};

use crate::client::ProntoClient;

mod client;
mod secret;

use crate::secret::{*};

slint::include_modules!();

static PRONTO_BASE_URL: &str = "https://stanfordohs.pronto.io/api/";

fn change_channel(ui: &AppWindow, client: &ProntoClient, channel_id: u64) {
    let history = client.get_bubble_history(channel_id, None);
    let mut ui_messages = Vec::new();
    for message in history.messages.iter().rev() {
        ui_messages.push(Message {
            id: message.id as i32,
            content: message.message.clone().into(),
            user: message.user.fullname.clone().into()
        })
    }
    ui.set_messages(ModelRc::new(VecModel::from(ui_messages)));
    if history.messages.len() > 0 {
        ui.set_top_msg_id(history.messages.last().unwrap().id as i32); // TODO: this is confusing
    } else {
        ui.set_top_msg_id(0);
    }
    ui.set_channel_id(channel_id as i32);
}

fn main() -> Result<(), slint::PlatformError> {
    let client = Arc::new(ProntoClient::new(PRONTO_BASE_URL.to_string(), PRONTO_SESSION, PRONTO_API_TOKEN, PAACT_2245_5302428));
    let ui = AppWindow::new()?;
    let resp = client.get_user_info();
    let channels = client.get_bubble_list();

    let mut ui_channels = Vec::new();
    for channel in channels.bubbles {
        ui_channels.push(Channel {
            id: channel.id as i32,
            title: channel.title.into(),
            supergroup: channel.issupergroup.unwrap_or(false)
        });
    }

    let first_channel = ui_channels.iter().nth(0).unwrap();
    change_channel(&ui, &client, first_channel.id as u64);

    ui.set_channels(ModelRc::new(VecModel::from(ui_channels)));
    ui.set_fullname(resp.user.fullname.into());

    ui.on_setChannel({
        let ui_handle = ui.as_weak();
        let client = Arc::clone(&client);
        move || {
            let ui = ui_handle.unwrap();
            let channel_index = ui.get_current_sidebar_item_id();
            let channels = client.get_bubble_list();
            let channel = channels.bubbles.iter().nth(channel_index as usize).unwrap();
            change_channel(&ui, &client, channel.id);
        }
    });

    ui.on_scrollChannel({
        let ui_handle = ui.as_weak();
        let client = Arc::clone(&client);
        move || {
            let ui = ui_handle.unwrap();
            println!("{} {} {}", ui.get_visible_height(), ui.get_viewport_y(), ui.get_viewport_height());
            if ui.get_viewport_y() > -20.0 { // TODO: Do not hardcode
                let top_msg_id = ui.get_top_msg_id();
                let channel_id = ui.get_channel_id();
                let history = client.get_bubble_history(channel_id as u64, Some(top_msg_id as u64));
                let mut ui_messages: Vec<Message> = ui.get_messages().iter().collect();
                let mut reversed: Vec<Message> = history.messages.clone().into_iter().rev().map(|msg| {
                    Message {
                        id: msg.id as i32,
                        content: msg.message.clone().into(),
                        user: msg.user.fullname.clone().into()
                    }
                }).collect();
                reversed.append(&mut ui_messages);
                ui.set_messages(ModelRc::new(VecModel::from(reversed.clone())));
                ui.set_top_msg_id(reversed.first().map(|msg| msg.id).unwrap_or(0) as i32); // TODO: this is confusing
            }
        }
    });

    ui.run()
}
