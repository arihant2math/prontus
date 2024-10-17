use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TaskInfo {
    pub organization_id: i64,
    pub uuid: String,
    pub title: String,
    pub notes: String,
    pub due: String,
    pub assigneeuser_id: i64,
}
