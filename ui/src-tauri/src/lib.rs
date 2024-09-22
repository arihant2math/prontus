use client::{Bubble, BubbleStats, GetBubbleMembershipSearchRequest, Message, ProntoClient, ReactionType, UserInfo};
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;
use tauri::{command, Manager, State};

mod handler;
pub use handler::{get_code, send_code, get_settings, set_settings};

mod error;
mod pusher_thread;
mod state;

pub use error::BackendError;
use pusher_thread::run_pusher_thread;
use settings::Settings;
pub use state::{AppData, AppState, InnerAppState};
use crate::state::ChannelUsers;

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
    let mut state_channel_list: Vec<(Bubble, Option<BubbleStats>)> = vec![];
    for bubble in channel_list.bubbles.clone() {
        let stats = channel_list.stats.iter().find(|s| s.bubble_id == bubble.id).cloned();
        state_channel_list.push((bubble, stats));
    }
    let data = AppData {
        user_info,
        users,
        client: Arc::new(client),
        channel_list: state_channel_list,
        current_channel: 0,
        message_list: vec![],
        parent_messages: vec![],
        channel_users: HashMap::new(),
    };
    *state.inner().inner().write().await = InnerAppState::Loaded(data);
    Ok(())
}

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
    let user_info = {
        let state = state.clone().inner().inner();
        let state = state.read().await;
        let state = state.try_inner()?;

        if let Some(user) = state.users.get(&id) {
            return Ok(user.clone());
        }
        println!("{:?}", state.users.keys());
        state.client.get_user_info(Some(id)).await?
    };

    let state = state.inner().inner();
    let mut state = state.write().await;
    let state = state.try_inner_mut()?;
    state.users.insert(user_info.user.id, user_info.user.clone());

    Ok(user_info.user)
}

#[command]
async fn get_channel_list(
    state: State<'_, AppState>,
) -> Result<Vec<(Bubble, Option<BubbleStats>)>, BackendError> {
    let state = state.inner().inner();
    let state = state.read().await;
    let state = state.try_inner()?;

    Ok(state.channel_list.clone())
}

#[command]
async fn get_channel_info(
    state: State<'_, AppState>,
) -> Result<Option<(Bubble, Option<BubbleStats>)>, BackendError> {
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
    let messages = {
        let state = state.clone().inner().inner();
        let state = state.read().await;
        let state = state.try_inner()?;

        let id = state.current_channel;
        state.client.get_bubble_history(id, None).await?
    };
    let state = state.inner().inner();
    let mut state = state.write().await;
    let state = state.try_inner_mut()?;

    for message in messages.messages.iter() {
        if !state.users.contains_key(&message.user.id) {
            state.users.insert(message.user.id, message.user.clone());
        }
    }

    state.message_list = messages.messages;
    state.parent_messages = messages.parent_messages;
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
async fn get_parent_messages(state: State<'_, AppState>) -> Result<Vec<Message>, BackendError> {
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
    let mut messages = {
        let state = state.clone().inner().inner();
        let state = state.read().await;
        let state = state.try_inner()?;

        let id = state.current_channel;
        state
            .client
            .get_bubble_history(id, Some(last_message_id))
            .await?
    };

    let state = state.inner().inner();
    let mut state = state.write().await;
    let state = state.try_inner_mut()?;
    for message in messages.messages.iter() {
        if !state.users.contains_key(&message.user.id) {
            state.users.insert(message.user.id, message.user.clone());
        }
    }
    state.message_list.extend_from_slice(&mut messages.messages.clone());
    state.parent_messages.extend_from_slice(&mut messages.parent_messages);
    Ok(messages.messages)
}

#[command]
async fn edit_message(
    state: State<'_, AppState>,
    message_id: u64,
    message: String,
) -> Result<(), BackendError> {
    let message = {
        let state = state.clone().inner().inner();
        let state = state.read().await;
        let state = state.try_inner()?;
        state.client.edit_message(message_id, message).await?
    };
    let state = state.inner().inner();
    let mut state = state.write().await;
    let state = state.try_inner_mut()?;
    *state.message_list.iter_mut().find(|m| m.id == message_id).unwrap() = message.message;

    Ok(())
}

#[command]
async fn send_message(
    state: State<'_, AppState>,
    message: String,
    thread: Option<u64>
) -> Result<(), BackendError> {
    let response = {
        let state = state.clone().inner().inner();
        let state = state.read().await;
        let state = state.try_inner()?;
        let user_id = state.user_info.id;
        let id = state.current_channel;
        state.client.post_message(user_id, id, message, thread).await?
    };

    let state = state.inner().inner();
    let mut state = state.write().await;
    let state = state.try_inner_mut()?;
    if state.message_list.iter().find(|m| m.id == response.message.id).is_some() {
        return Ok(());
    }
    state.message_list.insert(0, response.message);
    Ok(())
}

#[command]
async fn set_reaction_state(
    state: State<'_, AppState>,
    message_id: u64,
    reaction_id: u64,
    active: bool,
) -> Result<(), BackendError> {
    async fn send_state_change(state: State<'_, AppState>, message_id: u64, reaction_type: ReactionType, active: bool) -> Result<Message, BackendError> {
        let state = state.clone().inner().inner();
        let state = state.read().await;
        let state = state.try_inner()?;

        let message = if active {
            state
                .client
                .add_reaction(message_id, reaction_type)
                .await?
        } else {
            state
                .client
                .remove_reaction(message_id, reaction_type)
                .await?
        };
        Ok(message.message)
    }
    let message = send_state_change(state.clone(), message_id, ReactionType::from(reaction_id as i32), active);
    {
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
            if let Some(o) = o {
                o.count += 1;
            } else {
                messages.reactions.push(client::Reactions {
                    id: reaction_id,
                    count: 1,
                    users: vec![state.user_info.id],
                });
            }
        } else {
            state
                .client
                .remove_reaction(message_id, ReactionType::from(reaction_id as i32))
                .await?;
            let messages = state.message_list.iter_mut().find(|message| message.id == message_id).unwrap();
            messages.reactions.iter_mut().find(|reaction| reaction.id == reaction_id).unwrap().count -= 1;
        }
    }
    let message = message.await?;

    let state = state.inner().inner();
    let mut state = state.write().await;
    let state = state.try_inner_mut()?;

    *state.message_list.iter_mut().find(|m| m.id == message_id).unwrap() = message;

    Ok(())
}

#[command]
async fn delete_message(
    state: State<'_, AppState>,
    message_id: u64,
) -> Result<(), BackendError> {
    {
        let state = state.inner().inner();
        let state = state.read().await;
        let state = state.try_inner()?;
        state.client.delete_message(message_id).await?;
    }
    let state = state.inner().inner();
    let mut state = state.write().await;
    let state = state.try_inner_mut()?;
    state.message_list.retain(|message| message.id != message_id);
    Ok(())
}

#[command]
async fn get_current_channel_id(state: State<'_, AppState>) -> Result<u64, BackendError> {
    let state = state.inner().inner();
    let state = state.read().await;
    let state = state.try_inner()?;

    Ok(state.current_channel)
}

#[command]
async fn get_channel_users(state: State<'_, AppState>, id: u64) -> Result<Vec<UserInfo>, BackendError> {
    let state = state.inner().inner();
    let state = state.read().await;
    let state = state.try_inner()?;

    let users = state.channel_users.get(&id).map(|u| {
        let u = u.clone();
        u.users.into_iter().map(|u| { state.users.get(&u).unwrap().clone() }).collect::<Vec<UserInfo>>()
    }).unwrap_or(vec![]);

    Ok(users)
}

#[command]
async fn load_channel_users(state: State<'_, AppState>, id: u64) -> Result<(), BackendError> {
    let membership = {
        let state = state.inner().inner();
        let state = state.read().await;
        let state = state.try_inner()?;
        let page = state.channel_users.get(&id).map(|u| u.pages).unwrap_or(0);
        state.client.get_bubble_membership(GetBubbleMembershipSearchRequest {
            bubble_id: id,
            page,
            ..Default::default()
        }).await?
    };

    let state = state.inner().inner();
    let mut state = state.write().await;
    let state = state.try_inner_mut()?;


    let users: Vec<u64> = membership.membership.iter().map(|m| m.user_id).collect();
    let o = state.channel_users.get_mut(&id).map(|u| {
        u.users.extend(users.clone());
        u.page += 1;
    });
    if o.is_none() {
        state.channel_users.insert(id, ChannelUsers {
            pages: membership.page_size,
            users,
            page: 1,
        });
    }
    for user in membership.membership {
        if !state.users.contains_key(&user.user_id) {
            state.users.insert(user.user_id, user.user);
        }
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let context = AppState::unloaded();
            let thread_handle = app.handle().clone();
            thread::spawn({
                let context = context.clone();
                move || {
                    let _ = run_pusher_thread(thread_handle, context);
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
            get_current_channel_id,
            get_channel_list,
            get_message,
            get_messages,
            get_more_messages,
            get_parent_messages,
            load_messages,
            edit_message,
            send_message,
            set_reaction_state,
            delete_message,
            get_channel_users,
            load_channel_users,
            get_settings,
            set_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
