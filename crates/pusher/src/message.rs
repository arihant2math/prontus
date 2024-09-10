use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

use client::ProntoClient;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PusherClientSubscribe {
    pub auth: String,
    pub channel: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PusherClientMessage {
    Subscribe(PusherClientSubscribe),
}

impl PusherClientMessage {
    pub async fn subscribe(client: Arc<ProntoClient>, socket_id: &str, channel: &str) -> Self {
        let auth = client.pusher_auth(socket_id, channel).await.unwrap();
        Self::Subscribe(PusherClientSubscribe {
            auth: auth.auth,
            channel: channel.to_string(),
        })
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Subscribe(sub) => serde_json::to_string(&RawPusherMessage {
                event: "pusher:subscribe".to_string(),
                data: serde_json::to_value(&sub).unwrap(),
                channel: None,
            })
            .unwrap(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PusherServerConnectionEstablished {
    pub socket_id: String,
    pub activity_timeout: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PusherServerSubscriptionSucceeded {
    pub channel: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PusherServerUserPresenceEvent {
    pub user_id: u64,
    #[serde(rename = "isonline")]
    pub is_online: bool,
    // TODO: the presence time is actually a date (UTC or smth it seems)
    #[serde(rename = "lastpresencetime")]
    pub last_presence_time: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PusherServerBubbleStatsEvent {
    pub stats: Vec<client::BubbleStats>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PusherServerMessageUpdatedEvent {
    pub message: client::Message,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PusherServerMessageAddedEvent {
    pub message: client::Message,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct MessageId {
    pub id: u64,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct PusherServerMessageRemovedEvent {
    pub message: MessageId,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct PusherServerUserTypingEvent {
    pub user_id: u64,
    pub thread_id: Option<u64>,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct PusherServerUserStoppedTypingEvent {
    pub user_id: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PusherMarkUpdatedEvent {
    pub user_id: u64,
    pub mark: u64,
    // TODO: is datetime (YYYY-MM-DD HH-mm-SS)
    pub markupdated: String,
}

// Received unknown message: RawPusherMessage { event: "App\\Events\\UserUpdated", data: String("{\"user\":{\"id\":5279672,\"firstname\":\"Ally\",\"lastname\":\"Aggarwal\",\"username\":null,\"locale\":\"\",\"lastseen\":\"
// 2024-09-10 20:36:32\",\"profilepic\":true,\"status\":0,\"created_at\":\"2023-07-28 18:15:32\",\"updated_at\":\"2024-09-10 20:36:50\",\"deactivated_at\":null,\"email_verified_at\":\"2023-11-05 04:53:50\",\"phone_verifie
// d_at\":null,\"isverified\":false,\"dropinorder\":0,\"maxstreams\":10,\"autotranslate\":false,\"isonline\":false,\"lastpresencetime\":\"2024-09-10 00:08:35\",\"acceptedtos\":\"2023-11-05 04:53:50\",\"sentwelcomemsg\":nu
// ll,\"role\":\"user\",\"mute\":false,\"muteuntil\":null,\"isbot\":0,\"pronouns\":null,\"fullname\":\"Ally Aggarwal\",\"hasactivity\":true,\"inactive\":false,\"language\":\"en\",\"permissions\":{\"change_name\":\"system\
// ",\"change_email\":\"system\",\"change_phone\":\"system\",\"remove_user\":\"system\",\"change_title\":\"admin\",\"change_pronouns\":\"admin\",\"change_own_name\":false,\"change_own_email\":false,\"change_own_phone\":fa
// lse,\"change_own_title\":true,\"change_own_pronouns\":true},\"profilepicpath\":\"\\/files\\/users\\/5279672\\/profilepic?pronto_time=1698399524\",\"profilepicurl\":\"https:\\/\\/files.chat.trypronto.com\\/files\\/users\\/5279672\\/profilepic?pronto_time=1698399524\"}}"), channel: Some("private-user.5302428") }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PusherServerEventType {
    PusherServerUserPresenceEvent(PusherServerUserPresenceEvent),
    PusherServerBubbleStatsEvent(PusherServerBubbleStatsEvent),
    PusherServerMessageUpdatedEvent(PusherServerMessageUpdatedEvent),
    PusherServerMessageAddedEvent(PusherServerMessageAddedEvent),
    PusherServerMessageRemovedEvent(PusherServerMessageRemovedEvent),
    PusherServerUserTypingEvent(PusherServerUserTypingEvent),
    PusherServerUserStoppedTypingEvent(PusherServerUserStoppedTypingEvent),
    PusherMarkUpdatedEvent(PusherMarkUpdatedEvent),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PusherServerEvent {
    pub channel: String,
    pub event: PusherServerEventType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PusherServerError {
    // TODO: check spec
    pub code: Option<()>,
    pub message: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PusherServerMessage {
    ConnectionEstablished(PusherServerConnectionEstablished),
    SubscriptionSucceeded(PusherServerSubscriptionSucceeded),
    Error(PusherServerError),
    Event(PusherServerEvent),
    Other(RawPusherMessage),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RawPusherMessage {
    pub event: String,
    pub data: Value,
    pub channel: Option<String>,
}

impl From<String> for PusherServerMessage {
    fn from(s: String) -> Self {
        let raw: RawPusherMessage = serde_json::from_str(&s).unwrap();
        match raw.event.as_str() {
            "pusher:connection_established" => {
                let data: PusherServerConnectionEstablished =
                    serde_json::from_str(raw.data.as_str().unwrap()).unwrap();
                Self::ConnectionEstablished(data)
            }
            "pusher_internal:subscription_succeeded" => {
                Self::SubscriptionSucceeded(PusherServerSubscriptionSucceeded {
                    channel: raw.channel.unwrap(),
                })
            }
            "pusher:error" => {
                let data: PusherServerError = serde_json::from_value(raw.data).unwrap();
                Self::Error(data)
            }
            "App\\Events\\UserPresence" => {
                let data: PusherServerUserPresenceEvent =
                    serde_json::from_str(raw.data.as_str().unwrap()).unwrap();
                Self::Event(PusherServerEvent {
                    channel: raw.channel.unwrap(),
                    event: PusherServerEventType::PusherServerUserPresenceEvent(data),
                })
            }
            "App\\Events\\BubbleStats" => {
                let data: PusherServerBubbleStatsEvent =
                    serde_json::from_str(raw.data.as_str().unwrap()).unwrap();
                Self::Event(PusherServerEvent {
                    channel: raw.channel.unwrap(),
                    event: PusherServerEventType::PusherServerBubbleStatsEvent(data),
                })
            }
            "App\\Events\\MessageUpdated" => {
                let data: PusherServerMessageUpdatedEvent =
                    serde_json::from_str(raw.data.as_str().unwrap()).unwrap();
                Self::Event(PusherServerEvent {
                    channel: raw.channel.unwrap(),
                    event: PusherServerEventType::PusherServerMessageUpdatedEvent(data),
                })
            }
            "App\\Events\\MessageAdded" => {
                let data: PusherServerMessageAddedEvent =
                    serde_json::from_str(raw.data.as_str().unwrap()).unwrap();
                Self::Event(PusherServerEvent {
                    channel: raw.channel.unwrap(),
                    event: PusherServerEventType::PusherServerMessageAddedEvent(data),
                })
            }
            "App\\Events\\MessageRemoved" => {
                let data: PusherServerMessageRemovedEvent =
                    serde_json::from_str(raw.data.as_str().unwrap()).unwrap();
                Self::Event(PusherServerEvent {
                    channel: raw.channel.unwrap(),
                    event: PusherServerEventType::PusherServerMessageRemovedEvent(data),
                })
            }
            "App\\Events\\UserTyping" => {
                let data: PusherServerUserTypingEvent =
                    serde_json::from_str(raw.data.as_str().unwrap()).unwrap();
                Self::Event(PusherServerEvent {
                    channel: raw.channel.unwrap(),
                    event: PusherServerEventType::PusherServerUserTypingEvent(data),
                })
            }
            "App\\Events\\UserStoppedTyping" => {
                let data: PusherServerUserStoppedTypingEvent =
                    serde_json::from_str(raw.data.as_str().unwrap()).unwrap();
                Self::Event(PusherServerEvent {
                    channel: raw.channel.unwrap(),
                    event: PusherServerEventType::PusherServerUserStoppedTypingEvent(data),
                })
            }
            "App\\Events\\MarkUpdated" => {
                let data: PusherMarkUpdatedEvent =
                    serde_json::from_str(raw.data.as_str().unwrap()).unwrap();
                Self::Event(PusherServerEvent {
                    channel: raw.channel.unwrap(),
                    event: PusherServerEventType::PusherMarkUpdatedEvent(data),
                })
            }
            _ => Self::Other(raw),
        }
    }
}
