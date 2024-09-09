use client::routes::user_login::{DeviceInfo, UserLoginRequest};
use client::{Bubble, BubbleStatsInfo, Message, ProntoClient, ReactionType, UserInfo};
use futures::future::join_all;
use pusher::{
    PusherClient, PusherServerEvent, PusherServerEventType, PusherServerMessage,
    PusherServerMessageWrapper,
};
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;
use tauri::{command, Manager, State};

mod error;
pub use error::BackendError;
mod state;
pub use state::{AppData, AppState, InnerAppState};

// TODO: Should not be backend error result
#[tokio::main]
async fn pusher_thread(context: AppState) -> Result<(), BackendError> {
    loop {
        if context.is_loaded().await {
            break;
        }
    }

    let pusher_client = {
        let state = context.inner();
        let mut state = state.write().await;
        let state = state.try_inner_mut()?;
        PusherClient::new(state.client.clone()).await
    };
    pusher_client.init().await;
    println!("Pusher client initialized");
    {
        let state = context.inner();
        let mut state_ = state.write().await;
        let state = state_.try_inner_mut()?;
        // TODO: make this portable (don't have constant org id)
        pusher_client
            .subscribe("private-organization.2245".to_string())
            .await;
        pusher_client
            .subscribe(format!("private-user.{}", state.user_info.id))
            .await;
        let mut tasks = vec![];
        for channel in state.channel_list.iter() {
            tasks.push(pusher_client.subscribe(format!(
                "private-bubble.{}.{}",
                channel.0.id, channel.0.channelcode
            )))
        }
        drop(state_);
        join_all(tasks).await;
        println!("Subscribed to pusher channels");
    }
    loop {
        let message = pusher_client.server_messages().await.recv().await;
        match message {
            Ok(PusherServerMessageWrapper::PusherServerMessage(message)) => {
                match message {
                    PusherServerMessage::Event(event) => {
                        match event.event {
                            PusherServerEventType::PusherServerMessageAddedEvent(event) => {
                                // TODO: create setting or smth
                                // TODO: Also make sure app in not in foreground
                                // Notification::new()
                                //     .summary("New Message")
                                //     .body(event.message.message.clone())
                                //     .icon("thunderbird")
                                //     .timeout(Timeout::Milliseconds(6000))
                                //     .show().unwrap();
                                let state = context.inner();
                                let mut state = state.write().await;
                                let state = state.try_inner_mut()?;
                                if event.message.bubble_id == state.current_channel {
                                    state.message_list.insert(0, event.message);
                                }
                            }
                            PusherServerEventType::PusherServerMessageUpdatedEvent(event) => {
                                let state = context.inner();
                                let mut state = state.write().await;
                                let state = state.try_inner_mut()?;
                                if event.message.bubble_id == state.current_channel {
                                    let message = state
                                        .message_list
                                        .iter_mut()
                                        .find(|m| m.id == event.message.id);
                                    if let Some(message) = message {
                                        *message = event.message;
                                    }
                                }
                            }
                            PusherServerEventType::PusherServerMessageRemovedEvent(event) => {
                                let state = context.inner();
                                let mut state = state.write().await;
                                let state = state.try_inner_mut()?;
                                state.message_list.retain(|m| m.id != event.message.id);
                            }
                            PusherServerEventType::PusherServerBubbleStatsEvent(event) => {
                                let state = context.inner();
                                let mut state = state.write().await;
                                let state = state.try_inner_mut()?;
                                // TODO: double for loop, ew
                                for (bubble, stats) in state.channel_list.iter_mut() {
                                    for stat in event.stats.iter() {
                                        if bubble.id == stat.bubble_id {
                                            *stats = stat.clone();
                                        }
                                    }
                                }
                            }
                            // TODO: handle other
                            _ => {}
                        }
                    }
                    PusherServerMessage::Other(raw) => {
                        println!("Received unknown message: {:?}", raw);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

#[command]
async fn get_code(email: String) -> Result<(), BackendError> {
    let _response = client::routes::user_verify::post(client::routes::user_verify::UserVerifyRequest::Email(email))
        .await
        .unwrap()
        .to_result();
    // TODO: Error handling
    Ok(())
}

#[command]
async fn send_code(email: String, code: String) -> Result<(), BackendError> {
    let response = client::routes::user_login::post(UserLoginRequest {
        email,
        code,
        // TODO: Fix
        device: DeviceInfo {
            browsername: "".to_string(),
            browserversion: "".to_string(),
            osname: "".to_string(),
            r#type: "".to_string(),
        },
    })
        .await
        .unwrap()
        .to_result()
        .unwrap();
    let token = &response.users[0].login_token;
    let mut settings = settings::Settings::load()?;
    settings.api_key = Some(token.clone());
    settings.save()?;
    // TODO: This is the part where we can switch base urls
    let client =
        ProntoClient::new("https://stanfordohs.pronto.io/api/".to_string(), token).unwrap();
    // TODO: Standardize device info
    let response = client
        .user_token_login(
            token,
            DeviceInfo {
                browsername: "".to_string(),
                browserversion: "".to_string(),
                osname: "".to_string(),
                r#type: "".to_string(),
            },
        )
        .await?;

    let mut settings = settings::Settings::load()?;
    settings.api_key = Some(response.users[0].access_token.clone());
    settings.save()?;
    // TODO: Error handling as usual
    Ok(())
}

#[command]
async fn load(state: State<'_, AppState>) -> Result<(), BackendError> {
    if state.is_loaded().await {
        return Ok(());
    }
    let settings = settings::Settings::load()?;
    let client = ProntoClient::new(
        "https://stanfordohs.pronto.io/api/".to_string(),
        &settings.api_key.ok_or(BackendError::NotAuthenticated)?,
    )
    .unwrap();
    let user_info_future = client.get_user_info();
    let channel_list_future = client.get_bubble_list();
    let (user_info, channel_list) = futures::join!(user_info_future, channel_list_future);
    let user_info = user_info?.user;
    let mut users = HashMap::new();
    users.insert(user_info.id, user_info.clone());
    let channel_list = channel_list?;
    let channel_list = channel_list
        .bubbles
        .clone()
        .into_iter()
        .zip(channel_list.stats.clone().into_iter())
        .collect();
    let data = AppData {
        user_info,
        users,
        client: Arc::new(client),
        channel_list,
        current_channel: 0,
        message_list: vec![],
    };
    *state.inner().inner().write().await = InnerAppState::Loaded(data);
    Ok(())
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[command]
async fn load_channel(state: State<'_, AppState>, id: u64) -> Result<(), BackendError> {
    let state = state.inner().inner();
    let mut state = state.write().await;
    let state = state.try_inner_mut()?;

    // FIXME: This commented statement panics
    // let bubble_info = state.client.get_bubble_info(id).await.unwrap();
    state.current_channel = id;
    Ok(())
}

#[command]
async fn get_current_user(state: State<'_, AppState>) -> Result<UserInfo, BackendError> {
    let state = state.inner().inner();
    let state = state.read().await;
    let state = state.try_inner()?;

    Ok(state.user_info.clone())
}

#[command]
async fn get_channel_list(
    state: State<'_, AppState>,
) -> Result<Vec<(Bubble, BubbleStatsInfo)>, BackendError> {
    let state = state.inner().inner();
    let state = state.read().await;
    let state = state.try_inner()?;

    Ok(state.channel_list.clone())
}

#[command]
async fn load_messages(state: State<'_, AppState>) -> Result<(), BackendError> {
    let state = state.inner().inner();
    let mut state = state.write().await;
    let state = state.try_inner_mut()?;

    let id = state.current_channel;
    let messages = state.client.get_bubble_history(id, None).await?;
    state.message_list = messages.messages;
    Ok(())
}

#[command]
async fn get_messages(state: State<'_, AppState>) -> Result<Vec<Message>, BackendError> {
    let state = state.inner().inner();
    let state = state.read().await;
    let state = state.try_inner()?;

    Ok(state.message_list.clone())
}

#[command]
async fn get_more_messages(
    state: State<'_, AppState>,
    last_message_id: u64,
) -> Result<Vec<Message>, BackendError> {
    let state = state.inner().inner();
    let mut state = state.write().await;
    let state = state.try_inner_mut()?;

    let id = state.current_channel;
    let messages = state
        .client
        .get_bubble_history(id, Some(last_message_id))
        .await?;
    let messages = messages.messages;
    state.message_list.extend_from_slice(&mut messages.clone());
    Ok(messages)
}

#[command]
async fn send_message(
    state: State<'_, AppState>,
    message: String,
) -> Result<Message, BackendError> {
    let state = state.inner().inner();
    let mut state = state.write().await;
    let state = state.try_inner_mut()?;

    let user_id = state.user_info.id;
    let id = state.current_channel;
    let response = state
        .client
        .post_message(user_id, id, message, None)
        .await?;
    Ok(response.message)
}

#[command]
async fn set_reaction_state(
    state: State<'_, AppState>,
    message_id: u64,
    reaction_id: u64,
    active: bool,
) -> Result<(), BackendError> {
    let state = state.inner().inner();
    let mut state = state.write().await;
    let state = state.try_inner_mut()?;

    if active {
        state
            .client
            .add_reaction(message_id, ReactionType::from(reaction_id as i32))
            .await?;
    } else {
        state
            .client
            .remove_reaction(message_id, ReactionType::from(reaction_id as i32))
            .await?;
    }
    Ok(())
}

#[command]
async fn delete_message(
    state: State<'_, AppState>,
    message_id: u64,
) -> Result<(), BackendError> {
    let state = state.inner().inner();
    let mut state = state.read().await;
    let state = state.try_inner()?;

    state.client.delete_message(message_id).await?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let context = AppState::unloaded();
            thread::spawn({
                let context = context.clone();
                move || {
                    let _ = pusher_thread(context);
                }
            });

            app.manage(context);
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_code,
            send_code,
            load,
            load_channel,
            get_current_user,
            get_channel_list,
            get_messages,
            get_more_messages,
            load_messages,
            send_message,
            set_reaction_state,
            delete_message
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
