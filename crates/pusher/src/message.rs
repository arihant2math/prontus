use std::sync::Arc;
use serde::{Deserialize, Serialize};
use serde_json::Value;
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
            Self::Subscribe(sub) => {
                serde_json::to_string(&RawPusherMessage {
                    event: "pusher:subscribe".to_string(),
                    data: serde_json::to_value(&sub).unwrap(),
                    channel: None,
                }).unwrap()
            }
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PusherServerConnectionEstablished {
    pub socket_id: String,
    pub activity_timeout: u64
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PusherServerSubscriptionSucceeded {
    pub channel: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PusherServerUserPresenceEvent {
    pub user_id: u64,
    #[serde(rename = "isonline")]
    pub is_online: bool,
    // TODO: the presence time is actually a date (UTC or smth it seems)
    #[serde(rename = "lastpresencetime")]
    pub last_presence_time: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PusherServerEventType {
    PusherServerUserPresenceEvent(PusherServerUserPresenceEvent)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PusherServerEvent {
    pub channel: String,
    pub event: PusherServerEventType
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PusherServerError {
    // TODO: check spec
    pub code: Option<()>,
    pub message: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PusherServerMessage {
    ConnectionEstablished(PusherServerConnectionEstablished),
    SubscriptionSucceeded(PusherServerSubscriptionSucceeded),
    Error(PusherServerError),
    UserPresence(PusherServerEvent),
    Other(RawPusherMessage)
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
                let data: PusherServerConnectionEstablished = serde_json::from_str(raw.data.as_str().unwrap()).unwrap();
                Self::ConnectionEstablished(data)
            }
            "pusher_internal:subscription_succeeded" => {
                Self::SubscriptionSucceeded(PusherServerSubscriptionSucceeded {
                    channel: raw.channel.unwrap()
                })
            }
            "pusher:error" => {
                let data: PusherServerError = serde_json::from_value(raw.data).unwrap();
                Self::Error(data)
            }
            "App\\Events\\UserPresence" => {
                let data: PusherServerUserPresenceEvent = serde_json::from_str(raw.data.as_str().unwrap()).unwrap();
                Self::UserPresence(PusherServerEvent {
                    channel: raw.channel.unwrap(),
                    event: PusherServerEventType::PusherServerUserPresenceEvent(data)
                })
            }
            _ => {
                Self::Other(raw)
            }
        }
    }
}
