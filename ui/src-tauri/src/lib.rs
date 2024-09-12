use client::routes::user_login::{DeviceInfo, UserLoginRequest};
use client::{Bubble, BubbleStats, GetBubbleMembershipSearchRequest, Message, ProntoClient, ReactionType, UserInfo};
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;
use tauri::{command, Manager, State};

mod error;
mod pusher_thread;
mod state;

pub use error::BackendError;
use pusher_thread::run_pusher_thread;
use settings::Settings;
pub use state::{AppData, AppState, InnerAppState};
use crate::state::ChannelUsers;

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
    let mut settings = Settings::load().await?;
    settings.auth.api_key = Some(token.clone());
    settings.save().await?;
    // TODO: This is the part where we can switch base urls
    let client =
        ProntoClient::new("https://stanfordohs.pronto.io/api/".to_string(), token).unwrap();
    // TODO: Standardize device info
    let response = client
        .user_token_login(
            token,
        )
        .await?;

    let mut settings = Settings::load().await?;
    settings.auth.api_key = Some(response.users[0].access_token.clone());
    settings.save().await?;
    // TODO: Error handling as usual
    Ok(())
}

#[command]
async fn load(state: State<'_, AppState>) -> Result<(), BackendError> {
    if state.is_loaded().await {
        return Ok(());
    }
    let settings = Settings::load().await?;
    let client = ProntoClient::new(
        "https://stanfordohs.pronto.io/api/".to_string(),
        &settings.auth.api_key.ok_or(BackendError::NotAuthenticated)?,
    )
        .unwrap();
    let user_info_future = client.get_current_user_info();
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
        channel_users: HashMap::new(),
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
    // let bubble_info = state.client.get_bubble_info(id).await?;
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
async fn get_user(state: State<'_, AppState>, id: u64) -> Result<UserInfo, BackendError> {
    let unlocked_state = state.inner().inner();
    let unlocked_state_guard = unlocked_state.read().await;
    let unlocked_state = unlocked_state_guard.try_inner()?;

    if let Some(user) = unlocked_state.users.get(&id) {
        return Ok(user.clone());
    }
    let user_info = unlocked_state.client.get_user_info(Some(id)).await?;
    drop(unlocked_state_guard);
    let state = state.inner().inner();
    let mut state = state.write().await;
    let state = state.try_inner_mut()?;
    state.users.insert(user_info.user.id, user_info.user.clone());

    Ok(user_info.user)
}

#[command]
async fn get_channel_list(
    state: State<'_, AppState>,
) -> Result<Vec<(Bubble, BubbleStats)>, BackendError> {
    let state = state.inner().inner();
    let state = state.read().await;
    let state = state.try_inner()?;

    Ok(state.channel_list.clone())
}

#[command]
async fn get_channel_info(
    state: State<'_, AppState>,
) -> Result<Option<(Bubble, BubbleStats)>, BackendError> {
    let state = state.inner().inner();
    let state = state.read().await;
    let state = state.try_inner()?;

    let id = state.current_channel;
    let bubble = state
        .channel_list
        .iter()
        .find(|(bubble, _)| bubble.id == id);
    Ok(bubble.cloned())
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
async fn get_message(state: State<'_, AppState>, id: u64) -> Result<Option<Message>, BackendError> {
    let state = state.inner().inner();
    let state = state.read().await;
    let state = state.try_inner()?;

    Ok(state.message_list.iter().find(|message| message.id == id).cloned())
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
        let messages = state.message_list.iter_mut().find(|message| message.id == message_id).unwrap();
        let o = messages.reactions.iter_mut().find(|reaction| reaction.id == reaction_id);
        if o.is_none() {
            messages.reactions.push(client::Reactions {
                id: reaction_id,
                count: 1,
                users: vec![state.user_info.id],
            });
        } else {
            // TODO: can be more rustlike
            o.unwrap().count += 1;
        }
    } else {
        state
            .client
            .remove_reaction(message_id, ReactionType::from(reaction_id as i32))
            .await?;
        let messages = state.message_list.iter_mut().find(|message| message.id == message_id).unwrap();
        messages.reactions.iter_mut().find(|reaction| reaction.id == reaction_id).unwrap().count -= 1;
    }
    Ok(())
}

#[command]
async fn delete_message(
    state: State<'_, AppState>,
    message_id: u64,
) -> Result<(), BackendError> {
    let state = state.inner().inner();
    let state = state.read().await;
    let state = state.try_inner()?;

    state.client.delete_message(message_id).await?;
    Ok(())
}

#[command]
async fn get_channel_users(state: State<'_, AppState>, id: u64) -> Result<Vec<UserInfo>, BackendError> {
    let state = state.inner().inner();
    let state = state.read().await;
    let state = state.try_inner()?;

    let users = state.channel_users.get(&id).map(|u| {
        let u = u.clone();
        u.users.into_iter().map(|u| {state.users.get(&u).unwrap().clone()}).collect::<Vec<UserInfo>>()
    }).unwrap_or(vec![]);

    Ok(users)
}

#[command]
async fn load_channel_users(state: State<'_, AppState>, id: u64) -> Result<(), BackendError> {
    let state = state.inner().inner();
    let mut state = state.write().await;
    let state = state.try_inner_mut()?;

    let page = state.channel_users.get(&id).map(|u| u.pages).unwrap_or(0);
    let membership = state.client.get_bubble_membership(GetBubbleMembershipSearchRequest {
        bubble_id: id,
        page,
        ..Default::default()
    }).await?;
    let users: Vec<u64> = membership.membership.iter().map(|m| m.user_id).collect();
    let o = state.channel_users.get_mut(&id).map(|u| u.users.extend(users.clone()));
    if o.is_none() {
        state.channel_users.insert(id, ChannelUsers {
            pages: membership.page_size,
            users,
        });
    }
    for user in membership.membership {
        if !state.users.contains_key(&user.user_id) {
            state.users.insert(user.user_id, user.user);
        }
    }
    Ok(())
}

#[command]
async fn get_settings() -> Result<Settings, BackendError> {
    Ok(Settings::load().await?)
}

#[command]
async fn set_settings(settings: Settings) -> Result<(), BackendError> {
    settings.save().await?;
    Ok(())
}

#[command]
async fn rich(
    _state: State<'_, AppState>,
    message: String,
) -> Result<serde_json::Value, BackendError> {
    // let state = state.inner().inner();
    // let state = state.read().await;
    // let state = state.try_inner()?;

    let message = richtext::parse(&message);
    Ok(message)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let context = AppState::unloaded();
            thread::spawn({
                let context = context.clone();
                move || {
                    let _ = run_pusher_thread(context);
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
            get_channel_info,
            get_current_user,
            get_user,
            get_channel_list,
            get_message,
            get_messages,
            get_more_messages,
            load_messages,
            send_message,
            set_reaction_state,
            delete_message,
            get_channel_users,
            load_channel_users,
            get_settings,
            set_settings,
            rich
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
