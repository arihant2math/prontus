use std::collections::HashMap;
use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use log::{error, info};

use slint::{EventLoopError, Image, Model, ModelRc, VecModel, Weak};

use thiserror::Error;

use crate::{AppWindow, Channel, ChannelGroup, Message};
use crate::client::{ProntoClient, ReactionType};
use crate::image_service::ImageService;
use crate::settings::Settings;
use crate::websocket_worker::WebsocketTasks;

#[derive(Clone, Debug)]
pub enum WorkerTasks {
    ChangeChannel(Channel),
    ScrollChannel(u64, u64),
    AddMessage(u64, Option<u64>, String),
    RemoveMessage(u64),
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

pub fn lazy_get_pfp<F>(ui: Weak<AppWindow>, image_service: Arc<Mutex<ImageService>>, message: crate::client::Message, _width: u32, _height: u32, set_image: F)
    -> Image
    where F: Fn(AppWindow, Image) + Send + Copy + 'static {
    // TODO: don't spawn thread if cached
    // TODO: Return image
    // TODO: Respect width and height
    // TODO: Threadpool
    info!("Loading pfp");
    let message = message.clone();
    let ui = ui.clone();
    let thread_image_service = Arc::clone(&image_service);
    thread::spawn(move || {
        let mut unlocked_service = thread_image_service.lock().unwrap();
        let pfp_image_loaded = unlocked_service.block_get(&message.user.profilepicurl);
        drop(unlocked_service);
        if let Ok(pfp_image) = pfp_image_loaded {
            ui.upgrade_in_event_loop(move |ui| {
                let image = Image::from_rgba8(pfp_image);
                set_image(ui, image);
            }).unwrap();
        } else {
            error!("Failed to load pfp");
        }
    });
    let unlocked_service = image_service.lock().unwrap();
    let loading_image = unlocked_service.loading_image();
    drop(unlocked_service);
    Image::from_rgba8(loading_image)
}

pub fn lazy_get_image<F>(ui: Weak<AppWindow>, image_service: Arc<Mutex<ImageService>>, media: crate::client::MessageMedia, _width: u32, _height: u32, set_image: F)
    where F: Fn(AppWindow, Image) + Send + Copy + 'static {
    // TODO: don't spawn thread if cached
    // TODO: Return image
    // TODO: Respect width and height
    // TODO: Threadpool
    info!("Loading image");
    let message = media.clone();
    let ui = ui.clone();
    thread::spawn(move || {
        let mut unlocked_service = image_service.lock().unwrap();
        let media_image_loaded = unlocked_service.block_get(&message.url);
        drop(unlocked_service);
        if let Ok(media_image) = media_image_loaded {
            ui.upgrade_in_event_loop(move |ui| {
                let image = Image::from_rgba8(media_image);
                set_image(ui, image);
            }).unwrap();
        } else {
            error!("Failed to load image");
        }
    });
}

// TODO: Lazy load embed
// TODO: Struct
pub async fn worker(app: Weak<AppWindow>, rx: mpsc::Receiver<WorkerTasks>, websocket_tx: tokio::sync::mpsc::Sender<WebsocketTasks>) -> Result<(), NetWorkerError> {
    let settings = Settings::load("settings.json").unwrap();
    let client = if let Some(pronto_api_token) = settings.pronto_api_token {
        Arc::new(ProntoClient::new(settings.base_url.clone(), &pronto_api_token).unwrap())
    } else {
        panic!("No Pronto API token provided");
    };
    info!("Created Client");

    let image_service = Arc::new(Mutex::new(ImageService::new(Arc::clone(&client))));
    info!("Created Image Service");

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
                notifications: channels.stats[count].unread as i32,
                can_send_message: channel.grant_create_message
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
    info!("Retrieved User Info");

    websocket_tx.send(WebsocketTasks::SubscribeUser(user_info.id)).await;

    loop {
        let loop_client = Arc::clone(&client);
        let loop_user_info = Arc::clone(&user_info);
        let loop_image_service = Arc::clone(&image_service);

        let message = rx.recv();
        match message {
            // TODO: have a centralized message storage variable for time saving
            // TODO: store users in memory and share that with the websocket worker and ui thread.
            Ok(WorkerTasks::ChangeChannel(channel)) => {
                info!("Change channel to \"{}\": \"{}\"", channel.id, channel.title);
                let history_result = loop_client.get_bubble_history(channel.id as u64, None).await;
                match history_result {
                    Ok(history) => {
                        let weak_app = app.clone();
                        websocket_tx.send(WebsocketTasks::ChangeChannel(channel.id as u64)).await;
                        app.upgrade_in_event_loop(move |ui| {
                            let messages = ui.get_messages();
                            let messages = messages.as_any().downcast_ref::<VecModel<Message>>().unwrap();
                            let mut ui_messages = Vec::new();
                            for (index, message) in history.messages.clone().into_iter().enumerate().rev() {
                                let mut slint_message = message.clone().to_slint(&loop_user_info, &history.parentmessages);
                                slint_message.profile_picture = lazy_get_pfp(weak_app.clone(), Arc::clone(&loop_image_service), message.clone(), 200, 200, move |ui, image| {
                                    // This closure is the "magic" part. Once the image is lazy
                                    // loaded in the background, this closure will be called
                                    // WITHIN the UI thread. We use the ui reference to
                                    // get the existing card from model by index, set
                                    // image and then set the data back on the model.
                                    let len = ui.get_messages().iter().len() - 1;
                                    let mut message = ui.get_messages().row_data(len - index).unwrap();
                                    message.profile_picture = image;
                                    ui.get_messages().set_row_data(len - index, message);
                                });
                                ui_messages.push(slint_message);
                                for (count, media) in message.message_media.iter().enumerate() {
                                    lazy_get_image(weak_app.clone(), Arc::clone(&loop_image_service), media.clone(), 500, 200, move |ui, image| {
                                        // Same as above
                                        let len = ui.get_messages().iter().len() - 1;
                                        let message = ui.get_messages().row_data(len - index).unwrap();
                                        message.images.set_row_data(count, image);
                                        ui.get_messages().set_row_data(len - index, message);
                                    })
                                }
                                // TODO: Embed images
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
                info!("Scroll channel \"{}\" to {}", channel_id, top_msg_id);
                let history_response = loop_client.get_bubble_history(channel_id, Some(top_msg_id)).await;
                if let Ok(history) = history_response {
                    app.upgrade_in_event_loop(move |ui| {
                        let messages = ui.get_messages();
                        let messages = messages.as_any().downcast_ref::<VecModel<Message>>().unwrap();
                        for message in history.messages.clone().into_iter() {
                            messages.insert(0, message.to_slint(&loop_user_info, &history.parentmessages));
                        }
                        // TODO: load images
                        ui.set_top_msg_id(history.messages.last().map(|msg| msg.id).unwrap_or(0) as i32);
                    })?;
                } else {
                    error!("Failed to get history for channel \"{}\"", channel_id); // TODO: log actual error
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
                let message_response = loop_client.post_message(loop_user_info.id, channel_id, message, parent_id).await;
                if let Ok(message) = message_response {
                    app.upgrade_in_event_loop(move |ui| {
                        let messages = ui.get_messages();
                        let messages = messages.as_any().downcast_ref::<VecModel<Message>>().unwrap();
                        messages.push(message.message.clone().to_slint(&loop_user_info, &Vec::new()));
                    })?;
                } else {
                    error!("Failed to add message to {}", channel_id); // TODO: log actual error
                }
            }
            Ok(WorkerTasks::RemoveMessage(message_id)) => {
                info!("Remove message {}", message_id);
                loop_client.delete_message(message_id).await?;
                app.upgrade_in_event_loop(move |ui| {
                    let messages = ui.get_messages();
                    let messages = messages.as_any().downcast_ref::<VecModel<Message>>().unwrap();
                    messages.remove(messages.iter().position(|x| x.id as u64 == message_id).unwrap());
                })?;
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }
}