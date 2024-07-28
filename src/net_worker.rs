use log::{error, info};
use std::collections::HashMap;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use slint::{EventLoopError, Image, Model, ModelRc, VecModel, Weak};

use crate::client::{ProntoClient, ReactionType};
use crate::image_service::ImageService;
use crate::settings::Settings;
use crate::websocket_worker::WebsocketTasks;
use crate::{AppWindow, Channel, ChannelGroup, Message};
use thiserror::Error;
use tokio::runtime;
use tokio::runtime::Runtime;
use crate::client::UserInfo;

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

pub fn lazy_get_pfp<F>(
    ui: Weak<AppWindow>,
    image_service: Arc<Mutex<ImageService>>,
    message: crate::client::Message,
    _width: u32,
    _height: u32,
    set_image: F,
) -> Image
where
    F: Fn(AppWindow, Image) + Send + Copy + 'static,
{
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
            })
            .unwrap();
        } else {
            error!("Failed to load pfp");
        }
    });
    let unlocked_service = image_service.lock().unwrap();
    let loading_image = unlocked_service.loading_image();
    drop(unlocked_service);
    Image::from_rgba8(loading_image)
}

pub fn lazy_get_image<F>(
    ui: Weak<AppWindow>,
    image_service: Arc<Mutex<ImageService>>,
    media: crate::client::MessageMedia,
    _width: u32,
    _height: u32,
    set_image: F,
) where
    F: Fn(AppWindow, Image) + Send + Copy + 'static,
{
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
            })
            .unwrap();
        } else {
            error!("Failed to load image");
        }
    });
}

#[derive(Clone)]
pub struct Context {
    pub client: Arc<ProntoClient>,
    pub user_info: Arc<UserInfo>,
    pub image_service: Arc<Mutex<ImageService>>,
}

pub struct NetWorker {
    app: Weak<AppWindow>,
    rx: mpsc::Receiver<WorkerTasks>,
    websocket_tx: tokio::sync::mpsc::Sender<WebsocketTasks>,
}

impl NetWorker {
    pub fn new(
        app: Weak<AppWindow>,
        rx: mpsc::Receiver<WorkerTasks>,
        websocket_tx: tokio::sync::mpsc::Sender<WebsocketTasks>,
    ) -> Self {
        NetWorker {
            app,
            rx,
            websocket_tx,
        }
    }

    fn create_client() -> Arc<ProntoClient> {
        let settings = Settings::load("settings.json").unwrap();
        if let Some(pronto_api_token) = settings.pronto_api_token {
            let client = Arc::new(ProntoClient::new(settings.base_url.clone(), &pronto_api_token).unwrap());
            info!("Created Client");
            return client;
        } else {
            panic!("No Pronto API token provided");
        }
    }

    pub async fn run_async(&self) -> Result<(), NetWorkerError> {
        let client = Self::create_client();

        let image_service = Arc::new(Mutex::new(ImageService::new(Arc::clone(&client))));
        info!("Started Image Service");

       let channels = client.get_bubble_list().await?;
        let user_info = Arc::new(client.get_user_info().await?.user);
        let user_name = user_info.fullname.clone();
        info!("Retrieved User Info");


        self.app.upgrade_in_event_loop(move |ui| {
            ui.set_user_name(user_name.into());
            // TODO: Sort by priority
            let mut ui_channels_groups = HashMap::new();
            for (count, channel) in channels.bubbles.iter().enumerate() {
                let category_name =
                    &channel
                        .category
                        .clone()
                        .map(|c| c.title)
                        .unwrap_or_else(|| {
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
                    unread: channels.stats[count].unread as i32 > 0,
                    notifications: channels.stats[count].unread_mentions as i32,
                    can_send_message: channel.grant_create_message,
                };
                ui_channels_groups.get_mut(&key).unwrap().push(ui_channel);
            }
            let mut ui_channels = Vec::new();
            for ((category_id, category_name), ui_channels_group) in
                ui_channels_groups.clone().into_iter()
            {
                ui_channels.push(ChannelGroup {
                    id: category_id as i32,
                    title: category_name.into(),
                    channels: ModelRc::new(VecModel::from(ui_channels_group)),
                });
            }
            ui.set_channels(ModelRc::new(VecModel::from(ui_channels)));
        })?;

        self.websocket_tx
            .send(WebsocketTasks::SubscribeUser(user_info.id))
            .await;

        let context = Context {
            client: Arc::clone(&client),
            user_info: Arc::clone(&user_info),
            image_service: Arc::clone(&image_service),
        };

        loop {
            let loop_context = context.clone();

            let message = self.rx.recv();
            match message {
                // TODO: have a centralized message storage variable for time saving
                // TODO: store users in memory and share that with the websocket worker and ui thread.
                Ok(WorkerTasks::ChangeChannel(channel)) => {
                    info!(
                        "Change channel to \"{}\": \"{}\"",
                        channel.id, channel.title
                    );
                    let history_result = loop_context.client
                        .get_bubble_history(channel.id as u64, None)
                        .await;
                    match history_result {
                        Ok(history) => {
                            let weak_app = self.app.clone();
                            self.websocket_tx
                                .send(WebsocketTasks::ChangeChannel(channel.id as u64))
                                .await;
                            self.app.upgrade_in_event_loop(move |ui| {
                                let messages = ui.get_messages();
                                let messages = messages
                                    .as_any()
                                    .downcast_ref::<VecModel<Message>>()
                                    .unwrap();
                                let mut ui_messages = Vec::new();
                                for (index, message) in
                                    history.messages.clone().into_iter().enumerate().rev()
                                {
                                    let mut slint_message = message
                                        .clone()
                                        .to_slint(&loop_context.user_info, &history.parentmessages);
                                    slint_message.profile_picture = lazy_get_pfp(
                                        weak_app.clone(),
                                        Arc::clone(&loop_context.image_service),
                                        message.clone(),
                                        200,
                                        200,
                                        move |ui, image| {
                                            // This closure is the "magic" part. Once the image is lazy
                                            // loaded in the background, this closure will be called
                                            // WITHIN the UI thread. We use the ui reference to
                                            // get the existing card from model by index, set
                                            // image and then set the data back on the model.
                                            let len = ui.get_messages().iter().len() - 1;
                                            let mut message =
                                                ui.get_messages().row_data(len - index).unwrap();
                                            message.profile_picture = image;
                                            ui.get_messages().set_row_data(len - index, message);
                                        },
                                    );
                                    ui_messages.push(slint_message);
                                    for (count, media) in message.message_media.iter().enumerate() {
                                        lazy_get_image(
                                            weak_app.clone(),
                                            Arc::clone(&loop_context.image_service),
                                            media.clone(),
                                            500,
                                            200,
                                            move |ui, image| {
                                                // Same as above
                                                let len = ui.get_messages().iter().len() - 1;
                                                let message = ui
                                                    .get_messages()
                                                    .row_data(len - index)
                                                    .unwrap();
                                                message.images.set_row_data(count, image);
                                                ui.get_messages()
                                                    .set_row_data(len - index, message);
                                            },
                                        )
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
                                ui.set_viewport_y(
                                    ui.get_visible_height() - ui.get_viewport_height(),
                                );
                            })?;
                        }
                        Err(e) => {
                            info!("{:?}", channel);
                            error!(
                                "Failed to get history for channel {channel_id}: {e}",
                                channel_id = channel.id
                            );
                        }
                    }
                }
                Ok(WorkerTasks::ScrollChannel(channel_id, top_msg_id)) => {
                    info!("Scroll channel \"{}\" to {}", channel_id, top_msg_id);
                    if top_msg_id != 0 {
                        let history_response = loop_context.client
                            .get_bubble_history(channel_id, Some(top_msg_id))
                            .await;
                        if let Ok(history) = history_response {
                            self.app.upgrade_in_event_loop(move |ui| {
                                let messages = ui.get_messages();
                                let messages = messages
                                    .as_any()
                                    .downcast_ref::<VecModel<Message>>()
                                    .unwrap();
                                for message in history.messages.clone().into_iter() {
                                    messages.insert(
                                        0,
                                        message.to_slint(&loop_context.user_info, &history.parentmessages),
                                    );
                                }
                                // TODO: load images
                                ui.set_top_msg_id(
                                    history.messages.last().map(|msg| msg.id).unwrap_or(0) as i32,
                                );
                            })?;
                        } else {
                            error!("Failed to get history for channel \"{}\"", channel_id);
                            // TODO: log actual error
                        }
                    }
                }
                Ok(WorkerTasks::Reaction(message_id, reaction_type, add)) => {
                    info!("Reaction to {}: {:?} is {}", message_id, reaction_type, add);
                    if add {
                        loop_context.client.add_reaction(message_id, reaction_type).await?;
                    } else {
                        loop_context.client
                            .remove_reaction(message_id, reaction_type)
                            .await?;
                    }
                    self.app.upgrade_in_event_loop(move |ui| {
                        let messages = ui.get_messages();
                        let messages = messages
                            .as_any()
                            .downcast_ref::<VecModel<Message>>()
                            .unwrap();
                        for message in messages.iter() {
                            if message.id as u64 == message_id {
                                for reaction in message.reactions.iter() {
                                    if reaction.id == reaction_type as i32 {
                                        let user_ids = reaction
                                            .user_ids
                                            .as_any()
                                            .downcast_ref::<VecModel<i32>>()
                                            .unwrap();
                                        if add {
                                            user_ids.push(loop_context.user_info.id as i32);
                                        } else {
                                            user_ids.remove(
                                                user_ids
                                                    .iter()
                                                    .position(|x| x == loop_context.user_info.id as i32)
                                                    .unwrap(),
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    })?;
                }
                Ok(WorkerTasks::AddMessage(channel_id, parent_id, message)) => {
                    info!("Add message to {}: {}", channel_id, message);
                    let message_response = loop_context.client
                        .post_message(loop_context.user_info.id, channel_id, message, parent_id)
                        .await;
                    if let Ok(message) = message_response {
                        self.app.upgrade_in_event_loop(move |ui| {
                            let messages = ui.get_messages();
                            let messages = messages
                                .as_any()
                                .downcast_ref::<VecModel<Message>>()
                                .unwrap();
                            messages.push(
                                message
                                    .message
                                    .clone()
                                    .to_slint(&loop_context.user_info, &Vec::new()),
                            );
                        })?;
                    } else {
                        error!("Failed to add message to {}", channel_id); // TODO: log actual error
                    }
                }
                Ok(WorkerTasks::RemoveMessage(message_id)) => {
                    info!("Remove message {}", message_id);
                    loop_context.client.delete_message(message_id).await?;
                    self.app.upgrade_in_event_loop(move |ui| {
                        let messages = ui.get_messages();
                        let messages = messages
                            .as_any()
                            .downcast_ref::<VecModel<Message>>()
                            .unwrap();
                        messages.remove(
                            messages
                                .iter()
                                .position(|x| x.id as u64 == message_id)
                                .unwrap(),
                        );
                    })?;
                }
                Err(e) => {
                    panic!("{}", e);
                }
            }
        }
    }

    pub fn run(self) -> Result<(), NetWorkerError> {
        thread::spawn(move || {
            let runtime = Arc::new(
                runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build()
                    .unwrap(),
            );
            runtime.block_on(self.run_async())
        })
        .join()
        .unwrap()
    }
}
