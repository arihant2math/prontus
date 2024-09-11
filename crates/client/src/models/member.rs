use serde::{Deserialize, Serialize};
use crate::UserInfo;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Member {
    pub id: i64,
    pub user_id: u64,
    pub bubble_id: u64,
    pub mark: u64,
    pub mute: bool,
    pub created_at: String,
    pub updated_at: String,
    pub markupdated: String,
    pub banned: bool,
    pub reactions: bool,
    pub is_pinned: bool,
    pub user: UserInfo,
}
