use ui_lib::{state::UnlockError, AppState};
use client::Reactions;
use futures::future::join_all;
use log::{error, info, warn};
use notify_rust::{Notification, Timeout};
use pusher::{
    PusherClient, PusherServerEventType, PusherServerMessage, PusherServerMessageWrapper,
};
use settings::{Settings, SettingsError};
use tauri::{AppHandle, Emitter};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PusherThreadError {
    #[error("Settings error: {0}")]
    SettingsError(#[from] SettingsError),
    #[error("Unlock error: {0}")]
    UnlockError(#[from] UnlockError),
}

pub async fn run(
    handle: AppHandle,
    context: AppState,
) -> Result<(), PusherThreadError> {
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
    info!("Pusher client initialized");
    {
        let state = context.inner();
        let mut state_ = state.write().await;
        let state = state_.try_inner_mut()?;

        pusher_client
            .subscribe(format!(
                "private-organization.{}",
                state.user_info.organizations[0].id
            ))
            .await;
        pusher_client
            .subscribe(format!("private-user.{}", state.user_info.id))
            .await;
        let mut tasks = vec![];
        for channel in state.channel_list.iter() {
            tasks.push(pusher_client.subscribe(format!(
                "private-bubble.{}.{}",
                channel.0.id, channel.0.channel_code
            )))
        }
        drop(state_);
        join_all(tasks).await;
        info!("Subscribed to pusher channels");
    }

    // TODO: this object doesn't update instantly when a user changes a setting
    let settings = Settings::load().await?;
    let direct_mention = {
        let state = context.inner();
        let state = state.read().await;
        let state = state.try_inner()?;
        format!("<@{}>", state.user_info.id)
    };

    loop {
        let message = pusher_client.server_messages().await.recv().await;
        match message {
            Ok(PusherServerMessageWrapper::PusherServerMessage(message)) => {
                match message {
                    PusherServerMessage::Event(ev) => {
                        match ev.event {
                            PusherServerEventType::PusherServerMessageAddedEvent(event) => {
                                // TODO: Make sure app in not in foreground
                                if settings.options.notifications {
                                    let state = context.inner();
                                    let state = state.read().await;
                                    let state = state.try_inner()?;
                                    let channel = state
                                        .channel_list
                                        .iter()
                                        .find(|c| c.0.id == event.message.bubble_id);
                                    let mut show_notification = true;
                                    if let Some((_, _, membership)) = channel {
                                        if let Some(membership) = membership {
                                            if membership.notification_preference != "ALL" {
                                                show_notification = false;
                                            }
                                            if event.message.message.contains(&direct_mention) && (membership.notification_preference == "MENTIONS" || membership.notification_preference == "MENTIONS_EXCLUDE_ALL") {
                                                show_notification = true;
                                            }
                                            if event.message.message.contains("<@everyone>") && membership.notification_preference == "MENTIONS" {
                                                show_notification = true;
                                            }

                                            if membership.mute {
                                                show_notification = false;
                                            }
                                        }
                                    }
                                    if show_notification {
                                        Notification::new()
                                            .summary(&format!(
                                                "New message from {user}",
                                                user = event.message.user.fullname
                                            ))
                                            .body(&event.message.message)
                                            .appname("Prontus")
                                            .icon("thunderbird")
                                            .timeout(Timeout::Milliseconds(6000))
                                            .show()
                                            .unwrap();
                                    }
                                }
                                let state = context.inner();
                                let mut state = state.write().await;
                                let state = state.try_inner_mut()?;
                                if event.message.bubble_id == state.current_channel.id {
                                    if !state.message_list.iter().any(|m| m.id == event.message.id)
                                    {
                                        state.message_list.insert(0, event.message);
                                    }
                                }
                                let _ = handle.emit("messageListUpdate", ());
                            }
                            PusherServerEventType::PusherServerMessageUpdatedEvent(event) => {
                                let state = context.inner();
                                let mut state = state.write().await;
                                let state = state.try_inner_mut()?;
                                if event.message.bubble_id == state.current_channel.id {
                                    let message = state
                                        .message_list
                                        .iter_mut()
                                        .find(|m| m.id == event.message.id);
                                    if let Some(message) = message {
                                        *message = event.message;
                                    }

                                    let _ = handle.emit("messageListUpdate", ());
                                }
                            }
                            PusherServerEventType::PusherServerMessageRemovedEvent(event) => {
                                let state = context.inner();
                                let mut state = state.write().await;
                                let state = state.try_inner_mut()?;
                                state.message_list.retain(|m| m.id != event.message.id);

                                let _ = handle.emit("messageListUpdate", ());
                            }
                            PusherServerEventType::PusherServerBubbleStatsEvent(event) => {
                                let state = context.inner();
                                let mut state = state.write().await;
                                let state = state.try_inner_mut()?;
                                // double for loop (I can't think of a better way to do this)
                                // time complexity is O(b*n) in all cases
                                // Iterating through the event stats first would lead to a better average/best case complexity
                                for (bubble, stats, _) in state.channel_list.iter_mut() {
                                    for stat in event.stats.iter() {
                                        if bubble.id == stat.bubble_id {
                                            *stats = Some(stat.clone());
                                        }
                                    }
                                }

                                let _ = handle.emit("channelListUpdate", ());
                            }
                            PusherServerEventType::PusherServerUserPresenceEvent(event) => {
                                let state = context.inner();
                                let mut state = state.write().await;
                                let state = state.try_inner_mut()?;
                                for (id, user) in state.users.iter_mut() {
                                    if id == &event.user_id {
                                        user.online = event.is_online;
                                    }
                                }

                                let _ = handle.emit("messageListUpdate", ());
                                let _ = handle.emit("channelListUpdate", ());
                            }
                            PusherServerEventType::PusherServerUserUpdatedEvent(event) => {
                                let state = context.inner();
                                let mut state = state.write().await;
                                let state = state.try_inner_mut()?;
                                let user = state.users.get_mut(&event.user.id);
                                if let Some(user) = user {
                                    *user = event.user;
                                } else {
                                    state.users.insert(event.user.id, event.user);
                                }

                                let _ = handle.emit("messageListUpdate", ());
                                let _ = handle.emit("channelListUpdate", ());
                            }
                            PusherServerEventType::PusherServerReactionAddedEvent(event) => {
                                let state = context.inner();
                                let mut state = state.write().await;
                                let state = state.try_inner_mut()?;
                                let message = state
                                    .message_list
                                    .iter_mut()
                                    .find(|m| m.id == event.message_id);
                                if let Some(message) = message {
                                    if message
                                        .reactions
                                        .iter_mut()
                                        .find(|r| r.id == event.reactiontype_id)
                                        .map(|r| {
                                            r.users.push(event.user_id);
                                            r.count = event.count;
                                        })
                                        .is_none()
                                    {
                                        message.reactions.push(Reactions {
                                            id: event.reactiontype_id,
                                            count: event.count,
                                            users: vec![event.user_id],
                                        });
                                    }
                                }
                                let _ = handle.emit("messageListUpdate", ());
                            }
                            PusherServerEventType::PusherServerReactionRemovedEvent(event) => {
                                let state = context.inner();
                                let mut state = state.write().await;
                                let state = state.try_inner_mut()?;
                                let message = state
                                    .message_list
                                    .iter_mut()
                                    .find(|m| m.id == event.message_id);
                                if let Some(message) = message {
                                    if let Some(reaction) = message
                                        .reactions
                                        .iter_mut()
                                        .find(|r| r.id == event.reactiontype_id)
                                    {
                                        reaction.users.retain(|u| u != &event.user_id);
                                        reaction.count = event.count;
                                    }
                                }

                                let _ = handle.emit("messageListUpdate", ());
                            }
                            PusherServerEventType::PusherServerMembershipUpdatedEvent(event) => {
                                let state = context.inner();
                                let mut state = state.write().await;
                                let state = state.try_inner_mut()?;
                                for (bubble, _, membership) in state.channel_list.iter_mut() {
                                    if bubble.id == event.membership.bubble_id {
                                        if let Some(membership) = membership {
                                            *membership = event.membership.clone();
                                            break;
                                        }
                                    }
                                }

                                let _ = handle.emit("channelListUpdate", ());
                            }
                            PusherServerEventType::PusherServerAnnouncementAddedEvent(event) => {
                                let state = context.inner();
                                let mut state = state.write().await;
                                let state = state.try_inner_mut()?;
                                state.announcements.insert(0, event.announcement.clone());

                                let _ = handle.emit("announcementListUpdate", ());
                            }
                            PusherServerEventType::PusherServerAnnouncementRemovedEvent(event) => {
                                let state = context.inner();
                                let mut state = state.write().await;
                                let state = state.try_inner_mut()?;
                                state
                                    .announcements
                                    .retain(|a| a.id != event.announcement_id);

                                let _ = handle.emit("announcementListUpdate", ());
                            }
                            PusherServerEventType::PusherServerAnnouncementUpdatedEvent(event) => {
                                let state = context.inner();
                                let mut state = state.write().await;
                                let state = state.try_inner_mut()?;
                                let announcement = state
                                    .announcements
                                    .iter_mut()
                                    .find(|a| a.id == event.announcement.id);
                                if let Some(announcement) = announcement {
                                    *announcement = event.announcement.clone();
                                }

                                let _ = handle.emit("announcementListUpdate", ());
                            }
                            PusherServerEventType::PusherServerUserTypingEvent(event) => {
                                let state = context.inner();
                                let mut state = state.write().await;
                                let state = state.try_inner_mut()?;
                                let channel_id = ev.channel.split(".").nth(1).unwrap().parse().unwrap();
                                let users = state.typing_users.entry(channel_id).or_default();
                                if !users.contains(&event.user_id) {
                                    users.push(event.user_id);
                                }

                                let _ = handle.emit("typingListUpdate", ());
                            }
                            PusherServerEventType::PusherServerUserStoppedTypingEvent(event) => {
                                let state = context.inner();
                                let mut state = state.write().await;
                                let state = state.try_inner_mut()?;
                                let channel_id = ev.channel.split(".").nth(1).unwrap().parse().unwrap();
                                let users = state.typing_users.entry(channel_id).or_default();
                                users.retain(|u| u != &event.user_id);

                                let _ = handle.emit("typingListUpdate", ());
                            }
                            // TODO: handle other
                            // _ => {}
                        }
                    }
                    PusherServerMessage::Error(e) => {
                        error!("Received error: {:?}", e);
                    }
                    PusherServerMessage::Other(raw) => {
                        warn!("Received unknown message: {:?}", raw);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
