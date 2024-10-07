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
    pub taskmedia: Vec<()>,
    pub user: UserInfo,
    pub completed: Option<()>,
    pub due: String,
    pub reminder_local: String,
    pub reminder_utc: String,
    pub created_at: String,
    pub updated_at: String,
}
