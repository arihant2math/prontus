use crate::bubble_history::{MessageMedia, MessageResource, Reactions};
use crate::serde_datetime;
use crate::UserInfo;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: u64,
    pub user_id: u64,
    pub bubble_id: u64,
    pub message: String,
    pub user: UserInfo,
    #[serde(default, rename = "systemevent")]
    pub system_event: Option<String>,
    #[serde(default, rename = "parentmessage_id")]
    pub parent_message_id: Option<u64>,
    #[serde(default, rename = "reactionsummary")]
    pub reactions: Vec<Reactions>,
    #[serde(default, rename = "messagemedia")]
    pub message_media: Vec<MessageMedia>,
    #[serde(default)]
    pub resource: Option<MessageResource>,
    #[serde(with = "serde_datetime")]
    pub created_at: NaiveDateTime,
    // #[serde(default, with = "serde_datetime")]
    // pub updated_at: Option<NaiveDateTime>,
}
