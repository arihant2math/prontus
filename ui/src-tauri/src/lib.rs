use client::{Announcement, Bubble, BubbleStats, Membership, Message, PostBubbleMembershipSearchRequest, ProntoClient, ReactionType, Task, UserInfo};
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;
use log::debug;
use tauri::{command, Manager, State};
use client::user_search::GetUserSearchRequest;

mod handler;
pub use handler::{get_code, get_settings, send_code, set_settings};

mod error;
mod state;
mod task;
#[cfg(desktop)]
mod tray;

pub use error::BackendError;
use task::run_pusher_thread;
use settings::Settings;
pub use state::{AppData, AppState, InnerAppState};
use task::run_proxy_thread;
use crate::state::ChannelUsers;

#[command]
async fn load(state: State<'_, AppState>) -> Result<(), BackendError> {
    if state.is_loaded().await {
        return Ok(());
    }
    let settings = Settings::load().await?;

    let client = ProntoClient::new(
        settings.auth.as_ref().ok_or(BackendError::NotAuthenticated)?.base_url.to_string(),
        &settings.auth.ok_or(BackendError::NotAuthenticated)?.api_key,
    )
        .unwrap();
    let user_info_future = client.current_user_info();
    let channel_list_future = client.bubble_list();
    let (user_info, channel_list) = futures::join!(user_info_future, channel_list_future);
    let user_info = user_info?.user;
    let mut users = HashMap::new();
    users.insert(user_info.id, user_info.clone());
    let channel_list = channel_list?;
    let mut state_channel_list: Vec<(Bubble, Option<BubbleStats>, Option<Membership>)> = vec![];
    for bubble in channel_list.bubbles.clone() {
        let stats = channel_list.stats.iter().find(|s| s.bubble_id == bubble.id).cloned();
        let membership = channel_list.memberships.iter().find(|m| m.bubble_id == bubble.id).cloned();
        state_channel_list.push((bubble, stats, membership));
    }
    let data = AppData {
        user_info,
        users,
        client: Arc::new(client),
        current_channel: state_channel_list[0].clone().0,
        channel_list: state_channel_list,
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

    let bubble_info = state.client.bubble_info(id).await?;
    state.current_channel = bubble_info.bubble;
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
        debug!("{:?}", state.users.keys());
        state.client.user_info(Some(id)).await?
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
) -> Result<Vec<(Bubble, Option<BubbleStats>, Option<Membership>)>, BackendError> {
    let state = state.inner().inner();
    let state = state.read().await;
    let state = state.try_inner()?;

    Ok(state.channel_list.clone())
}

#[command]
async fn get_channel_info(
    state: State<'_, AppState>,
) -> Result<Option<(Bubble, Option<BubbleStats>, Option<Membership>)>, BackendError> {
    let state = state.inner().inner();
    let state = state.read().await;
    let state = state.try_inner()?;

    let id = state.current_channel.id;
    let bubble = state
        .channel_list
        .iter()
        .find(|(bubble, _, _)| bubble.id == id);
    Ok(bubble.cloned())
}

#[command]
async fn load_messages(state: State<'_, AppState>) -> Result<(), BackendError> {
    let messages = {
        let state = state.clone().inner().inner();
        let state = state.read().await;
        let state = state.try_inner()?;

        let id = state.current_channel.id;
        state.client.bubble_history(id, None).await?
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

        let id = state.current_channel.id;
        state
            .client
            .bubble_history(id, Some(last_message_id))
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
        let id = state.current_channel.id;
        state.client.send_message(user_id, id, message, thread).await?
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
async fn get_current_channel_id(state: State<'_, AppState>) -> Result<Bubble, BackendError> {
    let state = state.inner().inner();
    let state = state.read().await;
    let state = state.try_inner()?;

    Ok(state.current_channel.clone())
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
        let page = state.channel_users.get(&id).map(|u| u.page).unwrap_or(1);
        state.client.bubble_membership(PostBubbleMembershipSearchRequest {
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
            page: 2,
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
async fn set_channel_mute(state: State<'_, AppState>, channel_id: u64, mute: bool) -> Result<(), BackendError> {
    let membership = {
        let state = state.inner().inner();
        let state = state.read().await;
        let state = state.try_inner()?;

        state.client.mute_bubble(channel_id, mute).await?
    };

    let state = state.inner().inner();
    let mut state = state.write().await;
    let state = state.try_inner_mut()?;
    state.channel_list.iter_mut().find(|(bubble, _, _)| bubble.id == state.current_channel.id).unwrap().2 = Some(membership.membership);
    Ok(())
}

#[command]
async fn set_channel_pin(state: State<'_, AppState>, channel_id: u64, pin: bool) -> Result<(), BackendError> {
    let membership = {
        let state = state.inner().inner();
        let state = state.read().await;
        let state = state.try_inner()?;

        state.client.pin_bubble(channel_id, pin).await?
    };

    let state = state.inner().inner();
    let mut state = state.write().await;
    let state = state.try_inner_mut()?;
    state.channel_list.iter_mut().find(|(bubble, _, _)| bubble.id == state.current_channel.id).unwrap().2 = Some(membership.membership);
    Ok(())
}

#[command]
async fn set_channel_alias(state: State<'_, AppState>, channel_id: u64, alias: Option<String>) -> Result<(), BackendError> {
    let membership = {
        let state = state.inner().inner();
        let state = state.read().await;
        let state = state.try_inner()?;

        state.client.set_bubble_alias(channel_id, alias).await?
    };

    let state = state.inner().inner();
    let mut state = state.write().await;
    let state = state.try_inner_mut()?;
    state.channel_list.iter_mut().find(|(bubble, _, _)| bubble.id == state.current_channel.id).unwrap().2 = Some(membership.membership);
    Ok(())
}

#[command]
async fn set_channel_notifications(state: State<'_, AppState>, channel_id: u64, level: String) -> Result<(), BackendError> {
    let membership = {
        let state = state.inner().inner();
        let state = state.read().await;
        let state = state.try_inner()?;

        state.client.set_bubble_notifications_preferences(channel_id, match &*level {
            "ALL" => client::NotificationsPreference::All,
            "MENTIONS" => client::NotificationsPreference::Mentions,
            "NOTHING" => client::NotificationsPreference::Nothing,
            _ => return Err(BackendError::NotLoaded),
        }).await?
    };

    let state = state.inner().inner();
    let mut state = state.write().await;
    let state = state.try_inner_mut()?;
    state.channel_list.iter_mut().find(|(bubble, _, _)| bubble.id == state.current_channel.id).unwrap().2 = Some(membership.membership);
    Ok(())
}

#[command]
async fn read_channel(state: State<'_, AppState>, channel_id: u64) -> Result<(), BackendError> {
    {
        let state = state.inner().inner();
        let state = state.read().await;
        let state = state.try_inner()?;
        let latest_bubble_id = state.channel_list.iter().find(|(info, _, _)| info.id == channel_id).cloned().unwrap().1.unwrap().latest_message_id;
        state.client.update_bubble_mark(channel_id, latest_bubble_id).await?;
    }

    // TODO: update bubble stats
    Ok(())
}

#[command]
async fn create_dm(state: State<'_, AppState>, user_id: u64) -> Result<(), BackendError> {
    let channel_list = {
        let state = state.inner().inner();
        let state = state.read().await;
        let state = state.try_inner()?;
        state.client.create_dm(state.user_info.organizations[0].id, user_id).await?;
        let channel_list = state.client.bubble_list().await?;
        let mut state_channel_list: Vec<(Bubble, Option<BubbleStats>, Option<Membership>)> = vec![];
        for bubble in channel_list.bubbles.clone() {
            let stats = channel_list.stats.iter().find(|s| s.bubble_id == bubble.id).cloned();
            let membership = channel_list.memberships.iter().find(|m| m.bubble_id == bubble.id).cloned();
            state_channel_list.push((bubble, stats, membership));
        }
        state_channel_list
    };

    let state = state.inner().inner();
    let mut state = state.write().await;
    let state = state.try_inner_mut()?;
    state.channel_list = channel_list;

    Ok(())
}

#[command]
async fn create_bubble(state: State<'_, AppState>, name: String) -> Result<(), BackendError> {
    let channel_list = {
        let state = state.inner().inner();
        let state = state.read().await;
        let state = state.try_inner()?;
        state.client.create_bubble(state.user_info.organizations[0].id as u64, name).await?;
        let channel_list = state.client.bubble_list().await?;
        let mut state_channel_list: Vec<(Bubble, Option<BubbleStats>, Option<Membership>)> = vec![];
        for bubble in channel_list.bubbles.clone() {
            let stats = channel_list.stats.iter().find(|s| s.bubble_id == bubble.id).cloned();
            let membership = channel_list.memberships.iter().find(|m| m.bubble_id == bubble.id).cloned();
            state_channel_list.push((bubble, stats, membership));
        }
        state_channel_list
    };

    let state = state.inner().inner();
    let mut state = state.write().await;
    let state = state.try_inner_mut()?;
    state.channel_list = channel_list;

    Ok(())
}

#[command]
async fn user_search(state: State<'_, AppState>, query: String) -> Result<Vec<UserInfo>, BackendError> {
    let state = state.inner().inner();
    let state = state.read().await;
    let state = state.try_inner()?;

    let response = state.client.user_search(GetUserSearchRequest {
        query,
        ..Default::default()
    }).await?;

    Ok(response.data)
}

#[command]
async fn get_announcements(state: State<'_, AppState>) -> Result<Vec<Announcement>, BackendError> {
    let state = state.inner().inner();
    let state = state.read().await;
    let state = state.try_inner()?;

    let response = state.client.announcement_list().await?;

    Ok(response.announcements)
}

#[command]
async fn get_tasks(state: State<'_, AppState>, completed: bool) -> Result<Vec<Task>, BackendError> {
    let state = state.inner().inner();
    let state = state.read().await;
    let state = state.try_inner()?;

    let response = state.client.task_list(state.user_info.organizations[0].id, completed).await?;

    Ok(response.tasks)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app = tauri::Builder::default()
        .setup(|app| {
            let context = AppState::unloaded();
            let thread_handle = app.handle().clone();
            thread::spawn({
                let context = context.clone();
                move || {
                    let _ = run_proxy_thread(context);
                }
            });
            thread::spawn({
                let context = context.clone();
                move || {
                    let _ = run_pusher_thread(thread_handle, context);
                }
            });

            app.manage(context);

            let main_window = app.get_window("main").unwrap();
            main_window.set_title("Prontus")?;

            #[cfg(all(desktop))]
            {
                let handle = app.handle();
                tray::create_tray(handle)?;
            }

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
            set_settings,
            set_channel_mute,
            set_channel_pin,
            set_channel_alias,
            set_channel_notifications,
            read_channel,
            create_dm,
            create_bubble,
            user_search,
            get_announcements,
            get_tasks
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    app.run(|_app_handle, event| match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {}
    });
}
