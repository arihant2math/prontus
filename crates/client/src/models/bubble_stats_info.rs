use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BubbleStatsInfo {
    pub id: u64,
    pub user_id: u64,
    pub title: String,
    #[serde(rename = "isdm")]
    pub is_dm: bool,
    pub voice_only: bool,
    #[serde(rename = "issupergroup")]
    pub is_supergroup: bool,
    pub archived: u8,
}
