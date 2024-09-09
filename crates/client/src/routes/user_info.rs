use crate::UserInfo;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetUserInfoResponse {
    pub ok: bool,
    pub user: UserInfo,
}

pub type GetUserInfoResult = crate::APIResult<GetUserInfoResponse>;

pub async fn get(
    pronto_base_url: &str,
    client: &Client,
) -> Result<GetUserInfoResult, reqwest::Error> {
    let r = client
        .get(format!("{pronto_base_url}v1/user.info"))
        .send()
        .await?;
    let json = r.json().await?;
    Ok(json)
}
