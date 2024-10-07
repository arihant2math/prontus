use crate::models::Bubble;
use client_macros::api;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BubbleStats {
    pub bubble_id: u64,
    pub mark: u64,
    pub updated: Option<String>,
    pub unread: u64,
    pub unread_mentions: u64,
    pub latest_message_id: u64,
    pub latest_message_created_at: Option<String>,
    pub unclaimed_task_count: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetBubbleInfoResponse {
    pub ok: bool,
    pub bubble: Bubble,
    pub stats: Vec<BubbleStats>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetBubbleInfoRequest {
    pub bubble_id: u64,
}

pub type GetBubbleInfoResult = crate::APIResult<GetBubbleInfoResponse>;

api!(
    get,
    "v2/bubble.info",
    GetBubbleInfoResult,
    GetBubbleInfoRequest
);
