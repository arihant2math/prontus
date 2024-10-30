use tauri::{command, State};
use client::{Bubble, BubbleStats, Membership, UserInfo};
use ui_lib::{AppState, BackendError, ChannelUsers};

#[command]
pub async fn load_channel(state: State<'_, AppState>, id: u64) -> Result<(), BackendError> {
    let state = state.inner().inner();
    let mut state = state.write().await;
    let state = state.try_inner_mut()?;

    let bubble_info = state.client.bubble_info(id).await?;
    state.current_channel = bubble_info.bubble;
    Ok(())
}

#[command]
pub async fn get_current_channel(state: State<'_, AppState>) -> Result<Bubble, BackendError> {
    let state = state.inner().inner();
    let state = state.read().await;
    let state = state.try_inner()?;

    Ok(state.current_channel.clone())
}

#[command]
pub async fn get_channel_list(
    state: State<'_, AppState>,
) -> Result<Vec<(Bubble, Option<BubbleStats>, Option<Membership>)>, BackendError> {
    let state = state.inner().inner();
    let state = state.read().await;
    let state = state.try_inner()?;

    Ok(state.channel_list.clone())
}

#[command]
pub async fn get_channel_info(
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
pub async fn get_channel_users(
    state: State<'_, AppState>,
    id: u64,
) -> Result<Vec<UserInfo>, BackendError> {
    let state = state.inner().inner();
    let state = state.read().await;
    let state = state.try_inner()?;

    let users = state
        .channel_users
        .get(&id)
        .map(|u| {
            let u = u.clone();
            u.users
                .into_iter()
                .map(|u| state.users.get(&u).unwrap().clone())
                .collect::<Vec<UserInfo>>()
        })
        .unwrap_or(vec![]);

    Ok(users)
}

#[command]
pub async fn load_channel_users(state: State<'_, AppState>, id: u64) -> Result<(), BackendError> {
    let membership = {
        let state = state.inner().inner();
        let state = state.read().await;
        let state = state.try_inner()?;
        let page = state.channel_users.get(&id).map(|u| u.page).unwrap_or(1);
        state
            .client
            .bubble_membership(client::PostBubbleMembershipSearchRequest {
                bubble_id: id,
                page,
                ..Default::default()
            })
            .await?
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
        state.channel_users.insert(
            id,
            ChannelUsers {
                pages: membership.page_size,
                users,
                page: 2,
            },
        );
    }
    for user in membership.membership {
        if !state.users.contains_key(&user.user_id) {
            state.users.insert(user.user_id, user.user);
        }
    }
    Ok(())
}

#[command]
pub async fn set_channel_mute(
    state: State<'_, AppState>,
    channel_id: u64,
    mute: bool,
) -> Result<(), BackendError> {
    let membership = {
        let state = state.inner().inner();
        let state = state.read().await;
        let state = state.try_inner()?;

        state.client.mute_bubble(channel_id, mute).await?
    };

    let state = state.inner().inner();
    let mut state = state.write().await;
    let state = state.try_inner_mut()?;
    state
        .channel_list
        .iter_mut()
        .find(|(bubble, _, _)| bubble.id == state.current_channel.id)
        .unwrap()
        .2 = Some(membership.membership);
    Ok(())
}

#[command]
pub async fn set_channel_pin(
    state: State<'_, AppState>,
    channel_id: u64,
    pin: bool,
) -> Result<(), BackendError> {
    let membership = {
        let state = state.inner().inner();
        let state = state.read().await;
        let state = state.try_inner()?;

        state.client.pin_bubble(channel_id, pin).await?
    };

    let state = state.inner().inner();
    let mut state = state.write().await;
    let state = state.try_inner_mut()?;
    state
        .channel_list
        .iter_mut()
        .find(|(bubble, _, _)| bubble.id == state.current_channel.id)
        .unwrap()
        .2 = Some(membership.membership);
    Ok(())
}

#[command]
pub async fn set_channel_alias(
    state: State<'_, AppState>,
    channel_id: u64,
    alias: Option<String>,
) -> Result<(), BackendError> {
    let membership = {
        let state = state.inner().inner();
        let state = state.read().await;
        let state = state.try_inner()?;

        state.client.set_bubble_alias(channel_id, alias).await?
    };

    let state = state.inner().inner();
    let mut state = state.write().await;
    let state = state.try_inner_mut()?;
    state
        .channel_list
        .iter_mut()
        .find(|(bubble, _, _)| bubble.id == state.current_channel.id)
        .unwrap()
        .2 = Some(membership.membership);
    Ok(())
}

#[command]
pub async fn set_channel_notifications(
    state: State<'_, AppState>,
    channel_id: u64,
    level: String,
) -> Result<(), BackendError> {
    let membership = {
        let state = state.inner().inner();
        let state = state.read().await;
        let state = state.try_inner()?;

        state
            .client
            .set_bubble_notifications_preferences(
                channel_id,
                match &*level {
                    "ALL" => client::NotificationsPreference::All,
                    "MENTIONS" => client::NotificationsPreference::Mentions,
                    "NOTHING" => client::NotificationsPreference::Nothing,
                    _ => return Err(BackendError::NotLoaded),
                },
            )
            .await?
    };

    let state = state.inner().inner();
    let mut state = state.write().await;
    let state = state.try_inner_mut()?;
    state
        .channel_list
        .iter_mut()
        .find(|(bubble, _, _)| bubble.id == state.current_channel.id)
        .unwrap()
        .2 = Some(membership.membership);
    Ok(())
}

#[command]
pub async fn read_channel(state: State<'_, AppState>, channel_id: u64) -> Result<(), BackendError> {
    {
        let state = state.inner().inner();
        let state = state.read().await;
        let state = state.try_inner()?;
        let latest_bubble_id = state
            .channel_list
            .iter()
            .find(|(info, _, _)| info.id == channel_id)
            .cloned()
            .unwrap()
            .1
            .unwrap()
            .latest_message_id;
        state
            .client
            .update_bubble_mark(channel_id, latest_bubble_id)
            .await?;
    }

    // TODO: update bubble stats
    Ok(())
}

#[command]
pub async fn set_channel_title(state: State<'_, AppState>, channel_id: u64, title: String) -> Result<(), BackendError> {
    let state = state.inner().inner();
    let state = state.read().await;
    let state = state.try_inner()?;

    state.client.set_bubble_title(channel_id, title).await?;
    // TODO: update bubble
    Ok(())
}

#[command]
pub async fn set_channel_category(state: State<'_, AppState>, channel_id: u64, category_id: Option<u64>) -> Result<(), BackendError> {
    let state = state.inner().inner();
    let state = state.read().await;
    let state = state.try_inner()?;

    state.client.set_bubble_category(channel_id, category_id).await?;
    // TODO: update bubble
    Ok(())
}

#[command]
pub async fn delete_channel(state: State<'_, AppState>, channel_id: u64) -> Result<(), BackendError> {
    let state = state.inner().inner();
    let state = state.read().await;
    let state = state.try_inner()?;

    state.client.delete_bubble(channel_id).await?;
    // TODO: update bubble
    Ok(())
}
