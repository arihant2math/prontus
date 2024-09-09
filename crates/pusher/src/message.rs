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
    pub stats: Vec<client::BubbleStatsInfo>,
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

// Received other message: RawPusherMessage { event: "client-App\\Events\\UserTyping", data: String("{\"user_id\":5302428,\"thread_id\":null}"), channel: Some("private-bubble.3738656.bsFWoVrfRArjYaqv1CcFMaSSKs5z4DIapMMyaFGk") }
// Received other message: RawPusherMessage { event: "client-App\\Events\\UserStoppedTyping", data: String("{\"user_id\":5302428}"), channel: Some("private-bubble.3738656.bsFWoVrfRArjYaqv1CcFMaSSKs5z4DIapMMyaFGk") }
// Received other message: RawPusherMessage { event: "App\\Events\\MarkUpdated", data: String("{\"user_id\":5302428,\"mark\":88072979,\"markupdated\":\"2024-09-04 05:55:31\"}"), channel: Some("private-bubble.3738656.bsFWoVrfRArjYaqv1CcFMaSSKs5z4DIapMMyaFGk") }
// Received other message: RawPusherMessage { event: "App\\Events\\MarkUpdated", data: String("{\"user_id\":5279855,\"mark\":88072124,\"markupdated\":\"2024-09-04 05:55:56\"}"), channel: Some("private-bubble.3742021.y25TRXwZdNfCdzZKN5FzAEIev6AWdZp8edRH99ZW") }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PusherServerEventType {
    PusherServerUserPresenceEvent(PusherServerUserPresenceEvent),
    PusherServerBubbleStatsEvent(PusherServerBubbleStatsEvent),
    PusherServerMessageUpdatedEvent(PusherServerMessageUpdatedEvent),
    PusherServerMessageAddedEvent(PusherServerMessageAddedEvent),
    PusherServerMessageRemovedEvent(PusherServerMessageRemovedEvent),
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
            _ => Self::Other(raw),
        }
    }
}
