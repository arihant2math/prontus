use crate::models::Bubble;
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

pub async fn get(pronto_base_url: &str, client: &reqwest::Client, request: GetBubbleInfoRequest) -> Result<GetBubbleInfoResult, crate::ResponseError> {
    let r = client.get(format!("{pronto_base_url}{}", "v2/bubble.info")).query(&request).send().await?;
    let text = r.text().await?;
    let json = serde_json::from_str(&text);
    match json {
        Ok(json) => { return Ok(json); }
        Err(e) => {
            log::error!("Error parsing JSON response: {:?}" , e );
            return Err(crate::ResponseError::from(e));
        }
    }
}
