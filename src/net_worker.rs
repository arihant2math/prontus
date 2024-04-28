use std::collections::HashMap;
use std::sync::{Arc, mpsc};
use log::{error, info};

use slint::{EventLoopError, Model, ModelRc, VecModel, Weak};

use thiserror::Error;

use crate::{AppWindow, Channel, ChannelGroup, Message, PRONTO_BASE_URL};
use crate::client::{ProntoClient, ReactionType};
use crate::settings::Settings;

#[derive(Clone, Debug)]
pub enum WorkerTasks {
    ChangeChannel(Channel),
    ScrollChannel(u64, u64),
    AddMessage(u64, Option<u64>, String),
    Reaction(u64, ReactionType, bool),
}

#[derive(Debug, Error)]
pub enum NetWorkerError {
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("Response error: {0}")]
    APIError(#[from] crate::client::ResponseError),
    #[error("Event loop error: {0}")]
    EventLoopError(EventLoopError),
}

impl From<EventLoopError> for NetWorkerError {
    fn from(e: EventLoopError) -> Self {
        NetWorkerError::EventLoopError(e)
    }
}

pub async fn worker(app: Weak<AppWindow>, rx: mpsc::Receiver<WorkerTasks>) -> Result<(), NetWorkerError> {
    let settings = Settings::load("settings.json").unwrap();
    let client = if let Some(pronto_api_token) = settings.pronto_api_token {
        Arc::new(ProntoClient::new(PRONTO_BASE_URL.to_string(), &settings.pronto_session.clone().unwrap_or("".to_string()), &pronto_api_token.clone(), &settings.pacct.clone().unwrap_or("".to_string())).unwrap())
    } else {
        panic!("No Pronto API token provided");
    };
    info!("Created Client");

    let channels = client.get_bubble_list().await?;
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
    })?;


    let user_info = Arc::new(client.get_user_info().await?.user);
    loop {
        let loop_client = Arc::clone(&client);
        let loop_user_info = Arc::clone(&user_info);
        let message = rx.recv();
        match message {
            // TODO: have a centralized message storage variable for time saving
            // TODO: store users in memory and share that with the websocket worker and ui thread.
            Ok(WorkerTasks::ChangeChannel(channel)) => {
                info!("Change channel to {}", channel.title);
                let history_result = loop_client.get_bubble_history(channel.id as u64, None).await;
                match history_result {
                    Ok(history) => {
                        app.upgrade_in_event_loop(move |ui| {
                            let messages = ui.get_messages();
                            let messages = messages.as_any().downcast_ref::<VecModel<Message>>().unwrap();
                            let mut ui_messages = Vec::new();
                            for message in history.messages.clone().into_iter().rev() {
                                ui_messages.push(message.to_slint(&loop_user_info, &history.parentmessages));
                            }
                            messages.set_vec(ui_messages);
                            if history.messages.len() > 0 {
                                ui.set_top_msg_id(history.messages.last().unwrap().id as i32);
                            } else {
                                ui.set_top_msg_id(0);
                            }
                            ui.set_message("".to_string().into());
                            ui.set_current_channel(channel);
                            ui.set_viewport_y(ui.get_visible_height() - ui.get_viewport_height());
                        })?;
                    }
                    Err(e) => {
                        info!("{:?}", channel);
                        error!("Failed to get history for channel {channel_id}: {e}", channel_id = channel.id);
                    }
                }
            },
            Ok(WorkerTasks::ScrollChannel(channel_id, top_msg_id)) => {
                info!("Scroll channel {} to {}", channel_id, top_msg_id);
                let history_response = loop_client.get_bubble_history(channel_id, Some(top_msg_id)).await;
                if let Ok(history) = history_response {
                    app.upgrade_in_event_loop(move |ui| {
                        let messages = ui.get_messages();
                        let messages = messages.as_any().downcast_ref::<VecModel<Message>>().unwrap();
                        for message in history.messages.clone().into_iter() {
                            messages.insert(0, message.to_slint(&loop_user_info, &history.parentmessages));
                        }
                        ui.set_top_msg_id(history.messages.last().map(|msg| msg.id).unwrap_or(0) as i32);
                    })?;
                } else {
                    error!("Failed to get history for channel {}", channel_id); // TODO: log actual error
                }
            }
            Ok(WorkerTasks::Reaction(message_id, reaction_type, add)) => {
                info!("Reaction to {}: {:?} is {}", message_id, reaction_type, add);
                if add {
                    loop_client.add_reaction(message_id, reaction_type).await?;
                } else {
                    loop_client.remove_reaction(message_id, reaction_type).await?;
                }
                app.upgrade_in_event_loop(move |ui| {
                    let messages = ui.get_messages();
                    let messages = messages.as_any().downcast_ref::<VecModel<Message>>().unwrap();
                    for message in messages.iter() {
                        if message.id as u64 == message_id {
                            for reaction in message.reactions.iter() {
                                if reaction.id == reaction_type as i32 {
                                    let user_ids = reaction.user_ids.as_any().downcast_ref::<VecModel<i32>>().unwrap();
                                    if add {
                                        user_ids.push(loop_user_info.id as i32);
                                    } else {
                                        user_ids.remove(user_ids.iter().position(|x| x == loop_user_info.id as i32).unwrap());
                                    }
                                }
                            }
                        }
                    }
                })?;
            }
            Ok(WorkerTasks::AddMessage(channel_id, parent_id, message)) => {
                info!("Add message to {}: {}", channel_id, message);
                let message = loop_client.post_message(loop_user_info.id, channel_id, message, parent_id).await?;
                app.upgrade_in_event_loop(move |ui| {
                    let messages = ui.get_messages();
                    let messages = messages.as_any().downcast_ref::<VecModel<Message>>().unwrap();
                    messages.push(message.message.clone().to_slint(&loop_user_info, &Vec::new()));
                })?;
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }
}