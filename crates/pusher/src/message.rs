use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

use client::ProntoClient;

macro_rules! create_event {
    ($raw:expr, $event_type:ident, $data_type:ty) => {{
        let data: $data_type = serde_json::from_str($raw.data.as_str().unwrap()).unwrap();
        Self::Event(PusherServerEvent {
            channel: $raw.channel.unwrap(),
            event: PusherServerEventType::$event_type(data),
        })
    }};
}

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
pub struct PusherServerMembershipUpdatedEvent {
    pub membership: client::Membership,
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
pub struct PusherServerMarkUpdatedEvent {
    pub user_id: u64,
    pub mark: u64,
    // TODO: is datetime (YYYY-MM-DD HH-mm-SS)
    pub markupdated: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PusherServerReactionAddedEvent {
    pub message_id: u64,
    pub reactiontype_id: u64,
    pub user_id: u64,
    pub count: u64,
    pub emoji: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PusherServerReactionRemovedEvent {
    pub message_id: u64,
    pub reactiontype_id: u64,
    pub user_id: u64,
    pub count: u64,
    pub emoji: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PusherServerUserUpdatedEvent {
    pub user: client::UserInfo,
}

// Received unknown message: RawPusherMessage { event: "App\\Events\\AnnouncementRemoved", data: String("{\"announcement_id\":31608}"), channel: Some("private-organization.2245") }

//Received unknown message: RawPusherMessage { event: "App\\Events\\AnnouncementUpdated", data: String("{\"announcement\":{\"id\":32222,\"organ
// ization_id\":2245,\"senderuser_id\":5279806,\"targets\":{\"bubble_ids\":[2828820]},\"announcement\":\"\\u2b50\\ufe0f Hey Sophomores! \\u2b50\\ufe0f\\n\\nJust a quick reminder that our first movie night is s
// tarting at 5pm pst today!! Hope to see you there!\\n\\nZoom link: https:\\/\\/stanford.zoom.us\\/j\\/91400975734?pwd=2lA6b9bRi0VvzEKngRijdEgMyN75nv.1\\n\\n-Izzy and Neel\",\"created_at\":\"2024-09-28 23:05:
// 50\",\"updated_at\":\"2024-09-28 23:05:51\",\"deleted_at\":null,\"sent\":\"2024-09-28 23:05:51\",\"scheduled\":null,\"read\":\"2024-09-30 01:22:02\",\"lang\":\"en\",\"sender\":{\"id\":5279806,\"firstname\":
// \"Izzy\",\"lastname\":\"Nguyen\",\"username\":null,\"locale\":\"\",\"lastseen\":\"2024-09-29 04:11:38\",\"profilepic\":true,\"status\":0,\"created_at\":\"2023-07-28 18:17:03\",\"updated_at\":\"2024-09-29 04
// :11:50\",\"deactivated_at\":null,\"email_verified_at\":\"2024-09-20 22:48:01\",\"phone_verified_at\":null,\"isverified\":false,\"dropinorder\":0,\"maxstreams\":10,\"autotranslate\":true,\"isonline\":false,\
// "lastpresencetime\":\"2024-09-29 04:11:50\",\"acceptedtos\":\"2024-09-20 22:48:01\",\"sentwelcomemsg\":null,\"role\":\"user\",\"mute\":true,\"muteuntil\":null,\"isbot\":0,\"fullname\":\"Izzy Nguyen\",\"hasa
// ctivity\":true,\"inactive\":false,\"language\":\"en\",\"permissions\":{\"change_name\":\"system\",\"change_email\":\"system\",\"change_phone\":\"system\",\"remove_user\":\"system\",\"change_title\":\"admin\
// ",\"change_pronouns\":\"admin\",\"change_own_name\":false,\"change_own_email\":false,\"change_own_phone\":false,\"change_own_title\":true,\"change_own_pronouns\":true},\"profilepicpath\":\"\\/files\\/users\
// \/5279806\\/profilepic?pronto_time=1700314741\",\"profilepicurl\":\"https:\\/\\/files.chat.trypronto.com\\/files\\/users\\/5279806\\/profilepic?pronto_time=1700314741\"},\"announcementmedia\":[{\"id\":8696,
// \"announcement_id\":32222,\"title\":\"Screenshot 2024-09-28 at 4.05.06\\u202fPM.png\",\"url\":\"https:\\/\\/files.chat.trypronto.com\\/files\\/orgs\\/2245\\/announcements\\/32222\\/3422de00-7dee-11ef-9549-d
// bc45047dbab\",\"thumbnail\":\"\",\"width\":1268,\"height\":950,\"filesize\":640894,\"duration\":null,\"created_at\":\"2024-09-28 23:05:50\",\"updated_at\":\"2024-09-28 23:05:50\",\"uuid\":\"4e407183-10d9-48
// 8d-b5be-91dbe0d1b685\",\"mediatype\":\"PHOTO\",\"urlmimetype\":\"image\\/png\",\"thumbnailmimetype\":null,\"path\":\"\\/files\\/orgs\\/2245\\/announcements\\/32222\\/3422de00-7dee-11ef-9549-dbc45047dbab\",\"thumbnailpath\":\"\",\"external\":false}],\"announcementtrans\":[]}}"), channel: Some("private-user.5302428") }


#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PusherServerEventType {
    PusherServerUserPresenceEvent(PusherServerUserPresenceEvent),
    PusherServerBubbleStatsEvent(PusherServerBubbleStatsEvent),
    PusherServerMembershipUpdatedEvent(PusherServerMembershipUpdatedEvent),
    PusherServerMessageUpdatedEvent(PusherServerMessageUpdatedEvent),
    PusherServerMessageAddedEvent(PusherServerMessageAddedEvent),
    PusherServerMessageRemovedEvent(PusherServerMessageRemovedEvent),
    PusherServerUserTypingEvent(PusherServerUserTypingEvent),
    PusherServerUserStoppedTypingEvent(PusherServerUserStoppedTypingEvent),
    PusherMarkUpdatedEvent(PusherServerMarkUpdatedEvent),
    PusherServerReactionAddedEvent(PusherServerReactionAddedEvent),
    PusherServerReactionRemovedEvent(PusherServerReactionRemovedEvent),
    PusherServerUserUpdatedEvent(PusherServerUserUpdatedEvent),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PusherServerEvent {
    pub channel: String,
    pub event: PusherServerEventType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PusherServerError {
    pub code: Option<i64>,
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
                create_event!(raw, PusherServerUserPresenceEvent, PusherServerUserPresenceEvent)
            }
            "App\\Events\\BubbleStats" => {
                create_event!(raw, PusherServerBubbleStatsEvent, PusherServerBubbleStatsEvent)
            },
            "App\\Events\\MembershipUpdated" => {
                create_event!(raw, PusherServerMembershipUpdatedEvent, PusherServerMembershipUpdatedEvent)
            },
            "App\\Events\\MessageUpdated" => {
                create_event!(raw, PusherServerMessageUpdatedEvent, PusherServerMessageUpdatedEvent)
            }
            "App\\Events\\MessageAdded" => {
                create_event!(raw, PusherServerMessageAddedEvent, PusherServerMessageAddedEvent)
            }
            "App\\Events\\MessageRemoved" => {
                create_event!(raw, PusherServerMessageRemovedEvent, PusherServerMessageRemovedEvent)
            }
            "client-App\\Events\\UserTyping" => {
                create_event!(raw, PusherServerUserTypingEvent, PusherServerUserTypingEvent)
            }
            "client-App\\Events\\UserStoppedTyping" => {
                create_event!(raw, PusherServerUserStoppedTypingEvent, PusherServerUserStoppedTypingEvent)
            }
            "App\\Events\\MarkUpdated" => {
                create_event!(raw, PusherMarkUpdatedEvent, PusherServerMarkUpdatedEvent)
            }
            "App\\Events\\ReactionAdded" => {
                create_event!(raw, PusherServerReactionAddedEvent, PusherServerReactionAddedEvent)
            }
            "App\\Events\\ReactionRemoved" => {
                create_event!(raw, PusherServerReactionRemovedEvent, PusherServerReactionRemovedEvent)
            }
            "App\\Events\\UserUpdated" => {
                create_event!(raw, PusherServerUserUpdatedEvent, PusherServerUserUpdatedEvent)
            }
            _ => Self::Other(raw),
        }
    }
}
