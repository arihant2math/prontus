use ::settings::Settings;
use client::{
    Announcement, Bubble, BubbleStats, Membership, Message, ProntoClient, ReactionType, Task,
    UserInfo,
};
use dashmap::DashMap;
use search::milli::score_details::ScoringStrategy;
use search::milli::{GeoSortStrategy, TermsMatchingStrategy, TimeBudget};
use search::SearchResults;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::sync::atomic::AtomicBool;
use tauri::{command, Emitter, State};
use ui_lib::{AppData, AppState, BackendError};
use updater::Version;

#[command]
pub async fn load(state: State<'_, AppState>) -> Result<(), BackendError> {
    if state.is_loaded() {
        return Ok(());
    }
    let settings = Settings::load().await?;

    let client = ProntoClient::new(
        settings
            .auth
            .as_ref()
            .ok_or(BackendError::NotAuthenticated)?
            .base_url
            .to_string(),
        &settings
            .auth
            .as_ref()
            .ok_or(BackendError::NotAuthenticated)?
            .api_key
            .clone(),
    )
    .unwrap();
    let user_info_future = client.current_user_info();
    let channel_list_future = client.bubble_list();
    let (user_info, channel_list) = futures::join!(user_info_future, channel_list_future);
    let user_info = user_info?.user;
    let users = DashMap::new();
    users.insert(user_info.id, user_info.clone());
    let channel_list = channel_list?;
    let mut state_channel_list: Vec<(Bubble, Option<BubbleStats>, Option<Membership>)> = vec![];
    for bubble in channel_list.bubbles.clone() {
        let stats = channel_list
            .stats
            .iter()
            .find(|s| s.bubble_id == bubble.id)
            .cloned();
        let membership = channel_list
            .memberships
            .iter()
            .find(|m| m.bubble_id == bubble.id)
            .cloned();
        state_channel_list.push((bubble, stats, membership));
    }
    let tasks_list_incomplete = client.task_list(user_info.organizations[0].id, false);
    let tasks_list_complete = client.task_list(user_info.organizations[0].id, true);
    let announcements_list = client.announcement_list("RECEIVED".to_string());
    let (tasks_list_incomplete, tasks_list_complete, announcements_list) = futures::join!(
        tasks_list_incomplete,
        tasks_list_complete,
        announcements_list
    );
    let data = AppData {
        user_info,
        users,
        client: Arc::new(client),
        current_channel: RwLock::new(state_channel_list[0].clone().0),
        channel_list: RwLock::new(state_channel_list),
        message_list: RwLock::new(vec![]),
        parent_messages: RwLock::new(vec![]),
        channel_users: DashMap::new(),
        announcements: RwLock::new(announcements_list?.announcements),
        tasks: RwLock::new(tasks_list_incomplete?
            .tasks
            .iter()
            .chain(tasks_list_complete?.tasks.iter())
            .cloned()
            .collect()),
        is_typing: AtomicBool::new(false),
        typing_users: DashMap::new(),
        settings: RwLock::new(settings),
    };
    state.load(data);
    Ok(())
}

#[command]
pub async fn set_reaction_state(
    state: State<'_, AppState>,
    message_id: u64,
    reaction_id: u64,
    active: bool,
) -> Result<(), BackendError> {
    async fn send_state_change(
        state: State<'_, AppState>,
        message_id: u64,
        reaction_type: ReactionType,
        active: bool,
    ) -> Result<Message, BackendError> {
        let state = state.try_inner()?;

        let message = if active {
            state.client.add_reaction(message_id, reaction_type).await?
        } else {
            state
                .client
                .remove_reaction(message_id, reaction_type)
                .await?
        };
        Ok(message.message)
    }
    let message = send_state_change(
        state.clone(),
        message_id,
        ReactionType::from(reaction_id as i32),
        active,
    );
    {
        let state = state.try_inner()?;

        if active {
            state
                .client
                .add_reaction(message_id, ReactionType::from(reaction_id as i32))
                .await?;
            let mut message_list = state.message_list.write().map_err(|_| BackendError::RwLockWriteError)?;
            let messages = message_list
                .iter_mut()
                .find(|message| message.id == message_id)
                .unwrap();
            let o = messages
                .reactions
                .iter_mut()
                .find(|reaction| reaction.id == reaction_id);
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
            let mut message_list = state.message_list.write().map_err(|_| BackendError::RwLockWriteError)?;
            let messages = message_list
                .iter_mut()
                .find(|message| message.id == message_id)
                .unwrap();
            messages
                .reactions
                .iter_mut()
                .find(|reaction| reaction.id == reaction_id)
                .unwrap()
                .count -= 1;
        }
    }
    let message = message.await?;

    let state = state.try_inner()?;
    let mut message_list = state.message_list.write().map_err(|_| BackendError::RwLockWriteError)?;

    *message_list
        .iter_mut()
        .find(|m| m.id == message_id)
        .unwrap() = message;

    Ok(())
}

#[command]
pub async fn create_dm(handle: tauri::AppHandle, state: State<'_, AppState>, user_id: u64) -> Result<(), BackendError> {
    let channel_list = {
        let state = state.try_inner()?;
        state
            .client
            .create_dm(state.user_info.organizations[0].id, user_id)
            .await?;
        let channel_list = state.client.bubble_list().await?;
        let mut state_channel_list: Vec<(Bubble, Option<BubbleStats>, Option<Membership>)> = vec![];
        for bubble in channel_list.bubbles.clone() {
            let stats = channel_list
                .stats
                .iter()
                .find(|s| s.bubble_id == bubble.id)
                .cloned();
            let membership = channel_list
                .memberships
                .iter()
                .find(|m| m.bubble_id == bubble.id)
                .cloned();
            state_channel_list.push((bubble, stats, membership));
        }
        state_channel_list
    };

    let state = state.try_inner()?;
    let mut state_channel_list = state.channel_list.write().map_err(|_| BackendError::RwLockWriteError)?;
    *state_channel_list = channel_list;
    // TODO: Return new channel (id at the minimum)
    let _ = handle.emit("channelListUpdate", ());

    Ok(())
}

#[command]
pub async fn create_bubble(handle: tauri::AppHandle, state: State<'_, AppState>, name: String) -> Result<(), BackendError> {
    let channel_list = {
        let state = state.try_inner()?;
        state
            .client
            .create_bubble(state.user_info.organizations[0].id, name)
            .await?;
        let channel_list = state.client.bubble_list().await?;
        let mut state_channel_list: Vec<(Bubble, Option<BubbleStats>, Option<Membership>)> = vec![];
        for bubble in channel_list.bubbles.clone() {
            let stats = channel_list
                .stats
                .iter()
                .find(|s| s.bubble_id == bubble.id)
                .cloned();
            let membership = channel_list
                .memberships
                .iter()
                .find(|m| m.bubble_id == bubble.id)
                .cloned();
            state_channel_list.push((bubble, stats, membership));
        }
        state_channel_list
    };

    let state = state.try_inner()?;
    let mut state_channel_list = state.channel_list.write().map_err(|_| BackendError::RwLockWriteError)?;
    *state_channel_list = channel_list;
    // TODO: Return new channel (id at the minimum)
    let _ = handle.emit("channelListUpdate", ());

    Ok(())
}

#[command]
pub async fn user_search(
    state: State<'_, AppState>,
    query: String,
) -> Result<Vec<UserInfo>, BackendError> {
    let state = state.try_inner()?;

    let response = state
        .client
        .user_search(client::routes::user_search::GetUserSearchRequest {
            query,
            ..Default::default()
        })
        .await?;

    Ok(response.data)
}

#[command]
pub async fn get_announcements(
    state: State<'_, AppState>,
) -> Result<Vec<Announcement>, BackendError> {
    let state = state.try_inner()?;

    Ok(state.announcements.read().map_err(|_| BackendError::RwLockReadError)?.clone())
}

#[command]
pub async fn mark_announcement_read(
    state: State<'_, AppState>,
    id: u64,
) -> Result<(), BackendError> {
    let state = state.try_inner()?;
    let new_announcement = state.client.mark_read_announcement(id).await?;
    let mut state_announcements = state.announcements.write().map_err(|_| BackendError::RwLockWriteError)?;
    state_announcements.iter_mut().for_each(|announcement| {
        if announcement.id == new_announcement.announcement.id {
            *announcement = new_announcement.announcement.clone();
        }
    });
    Ok(())
}

#[command]
pub async fn get_tasks(state: State<'_, AppState>) -> Result<Vec<Task>, BackendError> {
    let state = state.try_inner()?;

    Ok(state.tasks.read().map_err(|_| BackendError::RwLockReadError)?.clone())
}

#[command]
pub async fn complete_task(
    handle: tauri::AppHandle,
    state: State<'_, AppState>,
    task_id: u64,
) -> Result<(), BackendError> {
    let updated_task = {
        let state = state.try_inner()?;
        state.client.task_complete(task_id).await?
    };

    let state = state.try_inner()?;
    let mut state_tasks = state.tasks.write().map_err(|_| BackendError::RwLockWriteError)?;
    if let Some(task) = state_tasks.iter_mut().find(|task| task.id == task_id) {
        *task = updated_task.task.clone();
    }
    let _ = handle.emit("taskListUpdate", ());
    Ok(())
}

#[command]
pub async fn uncomplete_task(
    handle: tauri::AppHandle,
    state: State<'_, AppState>,
    task_id: u64,
) -> Result<(), BackendError> {
    let updated_task = {
        let state = state.try_inner()?;
        state.client.task_uncomplete(task_id).await?
    };

    let state = state.try_inner()?;
    let mut state_tasks = state.tasks.write().map_err(|_| BackendError::RwLockWriteError)?;
    if let Some(task) = state_tasks.iter_mut().find(|task| task.id == task_id) {
        *task = updated_task.task.clone();
    }
    let _ = handle.emit("taskListUpdate", ());
    Ok(())
}

#[command]
pub async fn delete_task(
    handle: tauri::AppHandle,
    state: State<'_, AppState>,
    task_id: u64,
) -> Result<(), BackendError> {
    // TODO: Fix
    let state = state.try_inner()?;

    // state.client.task_delete(task_id).await?;
    let mut tasks = state.tasks.write().map_err(|_| BackendError::RwLockWriteError)?;
    *tasks = tasks
        .iter()
        .filter(|task| task.id != task_id)
        .cloned()
        .collect();
    let _ = handle.emit("taskListUpdate", ());
    Ok(())
}

#[command]
pub async fn set_typing(state: State<'_, AppState>, typing: bool) -> Result<(), BackendError> {
    let state = state.try_inner()?;
    state.is_typing.store(typing, std::sync::atomic::Ordering::Relaxed);
    // TODO: send pusher message
    Ok(())
}

#[command]
pub async fn get_typing_users(
    state: State<'_, AppState>,
) -> Result<DashMap<u64, Vec<u64>>, BackendError> {
    let state = state.try_inner()?;
    Ok(state.typing_users.clone())
}

#[command]
pub async fn search_local(
    state: State<'_, AppState>,
    query: String,
) -> Result<Option<SearchResults>, BackendError> {
    let state = state.try_inner()?;
    let settings = &state.settings.read().map_err(|_| BackendError::RwLockReadError)?;
    if let Some(msg) = settings.search.messages.as_ref() {
        let loc = PathBuf::from(&msg.path);
        let mut search = search::Search::new(&loc);
        let results = search
            .search(
                (!query.trim().is_empty()).then(|| query.trim()),
                TermsMatchingStrategy::Last,
                ScoringStrategy::Skip,
                false,
                &None,
                &None,
                GeoSortStrategy::default(),
                0,
                20,
                None,
                TimeBudget::max(),
                None,
                None,
            )
            .unwrap();
        // TODO: no unwrap (see above)
        Ok(Some(results))
    } else {
        Ok(None)
    }
}

#[command]
pub async fn version() -> Result<String, BackendError> {
    Ok(version::VERSION.to_string())
}

#[command]
pub async fn check_update(state: State<'_, AppState>) -> Result<Option<Version>, BackendError> {
    let state = state.try_inner()?;
    let update_channel = state.settings.read().map_err(|_| BackendError::RwLockReadError)?.update.channel.clone();
    let file = updater::UpdateFile::update_file(updater::UpdateChannel::from(
        &*update_channel,
    ))
    .await?;
    if file.update_available() {
        Ok(file.latest_version_details()?)
    } else {
        Ok(None)
    }
}
