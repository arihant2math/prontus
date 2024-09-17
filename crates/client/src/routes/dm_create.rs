use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::Bubble;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct PostDMCreateRequest {
    pub user_id: u64,
    pub organization_id: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostDMCreateResponse {
    pub ok: bool,
    pub bubble: Bubble,
}

pub type PostDMCreateResult = crate::APIResult<PostDMCreateResponse>;

pub async fn post(
    pronto_base_url: &str,
    client: &Client,
    request: PostDMCreateRequest
) -> Result<PostDMCreateResult, reqwest::Error> {
    let r = client
        .post(format!("{pronto_base_url}v1/dm.create"))
        .json(&request)
        .send()
        .await?;
    let json = r.json().await?;
    Ok(json)
}

