use futures::future::join_all;
use notify_rust::{Notification, Timeout};
use pusher::{PusherClient, PusherServerEventType, PusherServerMessage, PusherServerMessageWrapper};
use settings::Settings;
use crate::{AppState, BackendError};

// TODO: Should not be backend error result
#[tokio::main]
pub async fn run_pusher_thread(context: AppState) -> Result<(), BackendError> {
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

        pusher_client
            .subscribe(format!("private-organization.{}", state.user_info.organizations[0].id))
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
        println!("Subscribed to pusher channels");
    }

    // TODO: this object doesn't update instantly when a user changes a setting
    let settings = Settings::load().await?;

    loop {
        let message = pusher_client.server_messages().await.recv().await;
        match message {
            Ok(PusherServerMessageWrapper::PusherServerMessage(message)) => {
                match message {
                    PusherServerMessage::Event(event) => {
                        match event.event {
                            PusherServerEventType::PusherServerMessageAddedEvent(event) => {
                                // TODO: check the setting
                                // TODO: Also make sure app in not in foreground
                                if settings.options.notifications {
                                    Notification::new()
                                        .summary(&format!("New message from {user}",
                                                          user = event.message.user.fullname))
                                        .body(&event.message.message)
                                        .icon("thunderbird")
                                        .timeout(Timeout::Milliseconds(6000))
                                        .show().unwrap();
                                }
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
