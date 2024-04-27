use std::collections::HashMap;
use std::sync::{Arc, mpsc};

use slint::{Model, ModelRc, VecModel, Weak};

use thiserror::Error;

use crate::{AppWindow, Channel, ChannelGroup, Message, PRONTO_BASE_URL};
use crate::client::ProntoClient;
use crate::settings::Settings;

#[derive(Clone, Debug)]
pub enum WorkerTasks {
    ChangeChannel(u64),
    ScrollChannel(u64, u64),
    AddMessage(u64, Option<u64>, String)
}

#[derive(Debug, Error)]
pub enum NetWorkerError {
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
}

pub async fn worker(app: Weak<AppWindow>, rx: mpsc::Receiver<WorkerTasks>) -> Result<(), NetWorkerError> {
    let settings = Settings::load("settings.json").unwrap();
    let client = if let Some(pronto_api_token) = settings.pronto_api_token {
        Arc::new(ProntoClient::new(PRONTO_BASE_URL.to_string(), &settings.pronto_session.clone().unwrap_or("".to_string()), &pronto_api_token.clone(), &settings.pacct.clone().unwrap_or("".to_string())))
    } else {
        panic!("No Pronto API token provided");
    };

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
                    let messages = ui.get_messages();
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