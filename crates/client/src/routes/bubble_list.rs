use crate::models::Bubble;
use crate::BubbleStats;
use reqwest::Client;
use serde::{Deserialize, Serialize};

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
