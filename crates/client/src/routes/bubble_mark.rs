use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct PostBubbleMarkRequest {
    pub bubble_id: u64,
    pub message_id: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostBubbleMarkResponse {
    pub ok: bool
}

pub type PostBubbleMarkResult = crate::APIResult<PostBubbleMarkResponse>;

pub async fn post(
    pronto_base_url: &str,
    client: &Client,
    request: PostBubbleMarkRequest
) -> Result<PostBubbleMarkResult, reqwest::Error> {
    let r = client
        .post(format!("{pronto_base_url}v1/bubble.mark"))
        .json(&request)
        .send()
        .await?;
    let json = r.json::<crate::bubble_info::GetBubbleInfoResult>().await?;
    Ok(json)
}

