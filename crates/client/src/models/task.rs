use crate::UserInfo;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: u64,
    pub assigneeuser_id: u64,
    pub bubble_id: Option<u64>,
    pub organization_id: u64,
    pub user_id: u64,
    pub notes: String,
    pub remindedassignee: bool,
    pub title: String,
    pub uuid: String,
    pub assigneeuser: UserInfo,
    pub user: UserInfo,
    pub taskmedia: Vec<serde_json::Value>,
    pub user: UserInfo,
    pub completed: Option<String>,
    pub due: String,
    pub reminder_local: Option<String>,
    pub reminder_utc: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
