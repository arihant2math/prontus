use crate::UserInfo;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct GetUserInfoRequest {
    pub id: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetUserInfoResponse {
    pub ok: bool,
    pub user: UserInfo,
}

pub type GetUserInfoResult = crate::APIResult<GetUserInfoResponse>;

pub async fn get(
    pronto_base_url: &str,
    client: &Client,
    request: GetUserInfoRequest,
) -> Result<GetUserInfoResult, reqwest::Error> {
    let r = if request.id.is_none() {
        client.get(format!("{pronto_base_url}v1/user.info"))
    } else {
        client
            .get(format!("{pronto_base_url}v1/user.info"))
            .json(&request)
    }
    .send()
    .await?;
    let json = r.json().await?;
    Ok(json)
}
