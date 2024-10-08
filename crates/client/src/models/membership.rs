use crate::UserInfo;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Membership {
    pub id: u64,
    pub user_id: u64,
    pub bubble_id: u64,
    pub mark: u64,
    pub friends: bool,
    pub system: bool,
    pub mute: bool,
    pub created_at: String,
    pub updated_at: String,
    #[serde(rename = "markupdated")]
    pub mark_updated: String,
    #[serde(rename = "isdropin")]
    pub is_drop_in: bool,
    pub banned: bool,
    pub reactions: bool,
    #[serde(rename = "notificationrollup")]
    pub notification_rollup: bool,
    pub alias: Option<String>,
    #[serde(rename = "ishidden")]
    pub is_hidden: bool,
    #[serde(rename = "removedby")]
    pub removed_by: Option<String>,
    pub meetings: bool,
    #[serde(rename = "muteuntil")]
    pub mute_until: Option<String>,
    pub is_pinned: bool,
    pub role: String,
    pub snooze: Option<bool>,
    #[serde(rename = "notificationpreference")]
    pub notification_preference: String,
    pub user: Option<UserInfo>,
}
