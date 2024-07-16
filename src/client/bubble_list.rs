use crate::client::bubble::Bubble;
use reqwest::Client;
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
    pub unclaimed_task_count: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetBubbleListResponse {
    pub ok: bool,
    pub bubbles: Vec<Bubble>,
    pub stats: Vec<BubbleStats>,
}

pub type GetBubbleListResult = crate::APIResult<GetBubbleListResponse>;

pub async fn get(
    pronto_base_url: &str,
    client: &Client,
) -> Result<GetBubbleListResult, reqwest::Error> {
    let r = client
        .get(format!("{pronto_base_url}v2/bubble.list"))
        .send()
        .await?;
    let json = r.json::<GetBubbleListResult>().await?;
    Ok(json)
}
