use crate::client::bubble::Bubble;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BubbleStats {
    pub id: u64,
    pub user_id: u64,
    pub title: String,
    pub isdm: bool,
    pub voice_only: bool,
    pub issupergroup: bool,
    pub archived: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetBubbleInfoResponse {
    pub ok: bool,
    pub bubble: Bubble,
    pub stats: BubbleStats,
}

pub type GetBubbleInfoResult = crate::APIResult<GetBubbleInfoResponse>;

pub async fn get(
    pronto_base_url: &str,
    client: &Client,
    bubble_id: u64,
) -> Result<GetBubbleInfoResult, reqwest::Error> {
    let r = client
        .get(format!("{pronto_base_url}v2/bubble.info"))
        .query(&json!({ "bubble_id": bubble_id }))
        .send()
        .await?;
    let json = r.json::<GetBubbleInfoResult>().await?;
    Ok(json)
}
