use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: u64,
    pub firstname: String,
    pub lastname: String,
    pub username: Option<String>,
    pub profilepicurl: String,
    pub isverified: bool,
    pub isonline: bool,
    pub mute: bool,
    pub isbot: u8,
    pub hasmobileapp: Option<bool>,
    pub fullname: String,
    pub hasactivity: bool,
    pub inactive: bool,
    pub language: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetUserInfoResponse {
    pub ok: bool,
    pub user: UserInfo,
}

pub type GetUserInfoResult = crate::APIResult<GetUserInfoResponse>;

pub async fn get(pronto_base_url: &str, client: &Client) -> Result<GetUserInfoResult, reqwest::Error> {
    let r = client.get(format!("{pronto_base_url}v1/user.info"))
        .send()
        .await?;
    let json = r.json().await?;
    Ok(json)
}