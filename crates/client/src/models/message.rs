use crate::serde_datetime;
use crate::UserInfo;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MessageMediaFile {
    pub thumbnail: String,
    pub duration: i64,

    pub external: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageMedia {
    pub id: u64,
    pub message_id: i64,
    pub uuid: String,
    pub url: String,
    pub title: Option<String>,
    pub mediatype: String,
    #[serde(rename = "urlmimetype")]
    pub url_mimetype: String,
    pub width: u64,
    pub height: u64,
    pub filesize: u64,
    pub path: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageResource {
    pub id: u64,
    pub providerurl: String,
    pub snippet: String,
    pub url: String,
    pub title: String,
    pub thumbnailurl: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Reactions {
    #[serde(rename = "reactiontype_id")]
    pub id: u64,
    pub count: u64,
    pub users: Vec<u64>,
}

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
    #[serde(default, rename = "firstchildmessage_id")]
    pub first_child_message_id: Option<u64>,
    #[serde(default, rename = "lastchildmessage_id")]
    pub last_child_message_id: Option<u64>,
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
