use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::client::bubble::Bubble;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BubbleStats {
    pub id: u64,
    pub user_id: u64,
    pub title: String,
    pub isdm: bool,
    pub voice_only: bool,
    pub issupergroup: bool,
    pub archived: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetBubbleInfoResponse {
    pub ok: bool,
    pub bubble: Bubble,
    pub stats: BubbleStats,
}

pub async fn get(pronto_base_url: &str, client: &Client, bubble_id: u64) -> GetBubbleInfoResponse {
    let r = client.get(format!("{pronto_base_url}v2/bubble.info"))
        .query(&json!({ "bubble_id": bubble_id }))
        .send()
        .await
        .unwrap();
    let json = r.json::<GetBubbleInfoResponse>().await.unwrap();
    json
}
