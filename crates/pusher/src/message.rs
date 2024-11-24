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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PusherServerAnnouncementAddedEvent {
    pub announcement: client::Announcement,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PusherServerAnnouncementUpdatedEvent {
    pub announcement: client::Announcement,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PusherServerAnnouncementRemovedEvent {
    pub announcement_id: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PusherServerTaskUpdatedEvent {
    pub task: client::Task,
}

// RawPusherMessage { event: "App\\Events\\CategoryUpdated", data: String("{\"category\":{\"id\":679345,\"organization_
// id\":2245,\"created_at\":\"2023-08-13 04:44:16\",\"updated_at\":\"2023-08-13 04:44:16\",\"sortorder\":149,\"title\":\"Clubs\",\"externalid\":null,\"usercategory\":{\"id\":9689,\"user_id\":5302428,\"category_id\":679345,\"alias\":\"test\",\"created_at\":\"2024-10-03 03:35:32\",\"updated_at\":\"2024-10-03 03:35:32\"}}}"), channel: Some("private-user.5302428") }

// RawPusherMessage { event: "App\\Events\\BubbleChanged", data: String("{\"bubble\":{\"id\":3832006,\"updated_at\":\"2024-10-02 18:36:12\"}}"), channel: Some("private-user.5302428") }
// RawPusherMessage { event: "App\\Events\\MessageTransAdded", data: String("{\"messagetrans\":{\"message_id\":90641609
// ,\"translation\":\"<@5301875> BAHAHAHA\",\"user_edited_version\":0,\"updated_at\":\"2024-10-03 04:47:23\",\"created_at\":\"2024-10-03 04:47:23\",\"id\":910634,\"lang\":\"en\"}}"), channel: Some("private-bubble.3832006.OAOxiNFFvXM94frhiyO7kAq4wIMNG9Zhz52nNVLW") }
// Received unknown message: RawPusherMessage { event: "App\\Events\\MessageTransAdded", data: String("{\"messagetrans\":{\"message_id\":90641758
// ,\"translation\":\"Hahahahaha\",\"user_edited_version\":0,\"updated_at\":\"2024-10-03 04:50:23\",\"created_at\":\"2024-10-03 04:50:23\",\"id\":910636,\"lang\":\"en\"}}"), channel: Some("private-bubble.3832006.OAOxiNFFvXM94frhiyO7kAq4wIMNG9Zhz52nNVLW") }
// RawPusherMessage { event: "App\\Events\\MessageTransAdded", data: String("{\"messagetrans\":{\"message_id\":90641761
// ,\"translation\":\"lol\",\"user_edited_version\":0,\"updated_at\":\"2024-10-03 04:50:25\",\"created_at\":\"2024-10-03 04:50:25\",\"id\":910637,\"lang\":\"en\"}}"), channel: Some("private-bubble.3832006.OAOxiNFFvXM94frhiyO7kAq4wIMNG9Zhz52nNVLW") }
// RawPusherMessage { event: "App\\Events\\MessageTransAdded", data: String("{\"messagetrans\":{\"message_id\":90641763
// ,\"translation\":\"lol\",\"user_edited_version\":0,\"updated_at\":\"2024-10-03 04:50:27\",\"created_at\":\"2024-10-03 04:50:27\",\"id\":910638,\"lang\":\"en\"}}"), channel: Some("private-bubble.3832006.OAOxiNFFvXM94frhiyO7kAq4wIMNG9Zhz52nNVLW") }
// 2024-10-02T21:50:42.023731400-07:00 WARN ui_lib::pusher_thread - Received unknown message: RawPusherMessage { event: "App\\Events\\MessageTransAdded", data: String("{\"messagetrans\":{\"message_id\":90641776
// ,\"translation\":\"lol\",\"user_edited_version\":0,\"updated_at\":\"2024-10-03 04:50:41\",\"created_at\":\"2024-10-03 04:50:41\",\"id\":910639,\"lang\":\"en\"}}"), channel: Some("private-bubble.3832006.OAOxiNFFvXM94frhiyO7kAq4wIMNG9Zhz52nNVLW") }
// 2024-10-02T21:50:43.842035200-07:00 WARN ui_lib::pusher_thread - Received unknown message: RawPusherMessage { event: "App\\Events\\MessageTransAdded", data: String("{\"messagetrans\":{\"message_id\":90641780
// ,\"translation\":\"lol\",\"user_edited_version\":0,\"updated_at\":\"2024-10-03 04:50:43\",\"created_at\":\"2024-10-03 04:50:43\",\"id\":910640,\"lang\":\"en\"}}"), channel: Some("private-bubble.3832006.OAOxiNFFvXM94frhiyO7kAq4wIMNG9Zhz52nNVLW") }
// 2024-10-02T21:50:44.373499500-07:00 WARN ui_lib::pusher_thread - Received unknown message: RawPusherMessage { event: "App\\Events\\MessageTransAdded", data: String("{\"messagetrans\":{\"message_id\":90641785
// ,\"translation\":\"lol\",\"user_edited_version\":0,\"updated_at\":\"2024-10-03 04:50:44\",\"created_at\":\"2024-10-03 04:50:44\",\"id\":910641,\"lang\":\"en\"}}"), channel: Some("private-bubble.3832006.OAOxiNFFvXM94frhiyO7kAq4wIMNG9Zhz52nNVLW") }
// 2024-10-02T21:50:44.435793-07:00 WARN ui_lib::pusher_thread - Received unknown message: RawPusherMessage { event: "App\\Events\\MessageTransAdded", data: String("{\"messagetrans\":{\"message_id\":90641782,\"
// translation\":\"lol\",\"user_edited_version\":0,\"updated_at\":\"2024-10-03 04:50:44\",\"created_at\":\"2024-10-03 04:50:44\",\"id\":910642,\"lang\":\"en\"}}"), channel: Some("private-bubble.3832006.OAOxiNFFvXM94frhiyO7kAq4wIMNG9Zhz52nNVLW") }
// RawPusherMessage { event: "App\\Events\\MessageTransAdded", data: String("{\"messagetrans\":{\"message_id\":90641824
// ,\"translation\":\"Lol\",\"user_edited_version\":0,\"updated_at\":\"2024-10-03 04:51:29\",\"created_at\":\"2024-10-03 04:51:29\",\"id\":910645,\"lang\":\"en\"}}"), channel: Some("private-bubble.3832006.OAOxiNFFvXM94frhiyO7kAq4wIMNG9Zhz52nNVLW") }
// RawPusherMessage { event: "App\\Events\\BubbleChanged", data: String("{\"bubble\":{\"id\":3832006,\"updated_at\":\"2024-10-03 04:45:44\"}}"), channel: Some("private-user.5302428") }
// Received unknown message: RawPusherMessage { event: "App\\Events\\BubbleRemoved", data: String("{\"bubble\":{\"id\":3872909,\"updated_at\":\"2024-10-17 17:42:30\",\"wasHidden\":false}}"), channel: Some("private-user.5302428") }
// RawPusherMessage { event: "App\\Events\\TaskUpdated", data: String("{\"task\":{\"id\":156087,\"uuid\":\"5112b51a-c92b-4baf
// -a47a-38124920be6c\",\"organization_id\":2245,\"bubble_id\":null,\"user_id\":5302428,\"title\":\"Completed Task\",\"notes\":\"Random notes\\n\\nNow
// updated\",\"assigneeuser_id\":5302428,\"due\":\"2025-09-30 00:00:00\",\"reminder_local\":null,\"reminder_utc\":null,\"remindedassignee\":false,\"completed\":null,\"created_at\":\"2024-10-16 00:47:11\",\"updated_at\":\"2024-10-18 00:02:40\"}}"), channel: Some("private-user.5302428") }
// Received unknown message: RawPusherMessage { event: "App\\Events\\TaskUpdated", data: String("{\"task\":{\"id\":156087,\"uuid\":\"5112b51a-c92b-4baf
// -a47a-38124920be6c\",\"organization_id\":2245,\"bubble_id\":null,\"user_id\":5302428,\"title\":\"Completed Task\",\"notes\":\"Random notes\\n\\nNow
// updated\",\"assigneeuser_id\":5302428,\"due\":\"2025-09-30 00:00:00\",\"reminder_local\":null,\"reminder_utc\":null,\"remindedassignee\":false,\"com
// pleted\":\"2024-10-18 00:02:46\",\"created_at\":\"2024-10-16 00:47:11\",\"updated_at\":\"2024-10-18 00:02:46\"}}"), channel: Some("private-user.5302428") }

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
    PusherServerAnnouncementAddedEvent(PusherServerAnnouncementAddedEvent),
    PusherServerAnnouncementUpdatedEvent(PusherServerAnnouncementUpdatedEvent),
    PusherServerAnnouncementRemovedEvent(PusherServerAnnouncementRemovedEvent),
    PusherServerTaskUpdatedEvent(PusherServerTaskUpdatedEvent),
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
                create_event!(
                    raw,
                    PusherServerUserPresenceEvent,
                    PusherServerUserPresenceEvent
                )
            }
            "App\\Events\\BubbleStats" => {
                create_event!(
                    raw,
                    PusherServerBubbleStatsEvent,
                    PusherServerBubbleStatsEvent
                )
            }
            "App\\Events\\MembershipUpdated" => {
                create_event!(
                    raw,
                    PusherServerMembershipUpdatedEvent,
                    PusherServerMembershipUpdatedEvent
                )
            }
            "App\\Events\\MessageUpdated" => {
                create_event!(
                    raw,
                    PusherServerMessageUpdatedEvent,
                    PusherServerMessageUpdatedEvent
                )
            }
            "App\\Events\\MessageAdded" => {
                create_event!(
                    raw,
                    PusherServerMessageAddedEvent,
                    PusherServerMessageAddedEvent
                )
            }
            "App\\Events\\MessageRemoved" => {
                create_event!(
                    raw,
                    PusherServerMessageRemovedEvent,
                    PusherServerMessageRemovedEvent
                )
            }
            "client-App\\Events\\UserTyping" => {
                create_event!(
                    raw,
                    PusherServerUserTypingEvent,
                    PusherServerUserTypingEvent
                )
            }
            "client-App\\Events\\UserStoppedTyping" => {
                create_event!(
                    raw,
                    PusherServerUserStoppedTypingEvent,
                    PusherServerUserStoppedTypingEvent
                )
            }
            "App\\Events\\MarkUpdated" => {
                create_event!(raw, PusherMarkUpdatedEvent, PusherServerMarkUpdatedEvent)
            }
            "App\\Events\\ReactionAdded" => {
                create_event!(
                    raw,
                    PusherServerReactionAddedEvent,
                    PusherServerReactionAddedEvent
                )
            }
            "App\\Events\\ReactionRemoved" => {
                create_event!(
                    raw,
                    PusherServerReactionRemovedEvent,
                    PusherServerReactionRemovedEvent
                )
            }
            "App\\Events\\UserUpdated" => {
                create_event!(
                    raw,
                    PusherServerUserUpdatedEvent,
                    PusherServerUserUpdatedEvent
                )
            }
            "App\\Events\\AnnouncementAdded" => {
                create_event!(
                    raw,
                    PusherServerAnnouncementAddedEvent,
                    PusherServerAnnouncementAddedEvent
                )
            }
            "App\\Events\\AnnouncementUpdated" => {
                create_event!(
                    raw,
                    PusherServerAnnouncementUpdatedEvent,
                    PusherServerAnnouncementUpdatedEvent
                )
            }
            "App\\Events\\AnnouncementRemoved" => {
                create_event!(
                    raw,
                    PusherServerAnnouncementRemovedEvent,
                    PusherServerAnnouncementRemovedEvent
                )
            }
            "App\\Events\\TaskUpdated" => {
                create_event!(
                    raw,
                    PusherServerTaskUpdatedEvent,
                    PusherServerTaskUpdatedEvent
                )
            }
            _ => Self::Other(raw),
        }
    }
}
