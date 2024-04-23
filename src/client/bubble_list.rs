use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::client::bubble::Bubble;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BubbleStats {
    pub bubble_id: u64,
    pub mark: u32,
    pub updated: String,
    pub unread: u32,
    pub unread_mentions: u32,
    pub latest_message_id: u64,
    pub latest_message_created_at: Option<String>,
    pub unclaimed_task_count: u32
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetBubbleListResponse {
    pub ok: bool,
    pub bubbles: Vec<Bubble>,
    pub stats: Vec<BubbleStats>
}

pub async fn get(pronto_base_url: &str, client: &Client) -> GetBubbleListResponse {
    let r = client.get(format!("{pronto_base_url}v2/bubble.list"))
        .send()
        .await
        .unwrap();
    let json = r.json::<GetBubbleListResponse>().await.unwrap();
    json
}
