use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BubbleStats {
    pub bubble_id: u64,
    pub mark: u32,
    pub updated: String,
    pub unread: u32,
    pub unread_mentions: u32,
    pub latest_message_id: u64,
    pub latest_message_created_at: Option<String>,
    #[serde(default)]
    pub unclaimed_task_count: u32,
}
