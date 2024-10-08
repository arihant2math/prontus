use crate::UserInfo;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MembershipInfo {
    pub id: i64,
    pub user_id: i64,
    pub bubble_id: i64,
    pub mark: i64,
    pub friends: bool,
    pub system: bool,
    pub mute: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub markupdated: String,
    pub isdropin: bool,
    pub banned: bool,
    pub reactions: bool,
    pub notificationrollup: bool,
    pub removedby: Option<String>,
    pub muteuntil: Option<String>,
    pub is_pinned: bool,
    pub supergroup_alert_seen: bool,
    pub role: String,
    pub user: UserInfo,
}
