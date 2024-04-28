use serde::{Deserialize, Serialize};
use crate::client::user_info::UserInfo;

#[derive(Serialize, Deserialize)]
struct DeviceInfo {
    pub browsername: String,
    pub browserversion: String,
    pub osname: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub uuid: String,
    pub osversion: String,
    pub appversion: String,
}

#[derive(Serialize, Deserialize)]
pub struct TokenLoginRequest {
    #[serde(rename = "logintokens")]
    pub login_tokens: Vec<String>,
    pub device: DeviceInfo,
}

#[derive(Serialize, Deserialize)]
pub struct TokenLoginUser {
    #[serde(rename = "accesstoken")]
    pub access_token: String,
    pub user: UserInfo
}

#[derive(Serialize, Deserialize)]
pub struct TokenLoginSuccess {
    pub ok: bool,
    pub users: Vec<TokenLoginUser>,
}

#[derive(Serialize, Deserialize)]
pub enum TokenLoginResponse {
    Success(TokenLoginSuccess),
    Error(crate::client::APIError)
}

pub async fn post(pronto_base_url: &str, client: &reqwest::Client, login_tokens: Vec<String>) -> Result<TokenLoginResponse, reqwest::Error> {
    let r = client.post(format!("{pronto_base_url}v1/user.tokenlogin"))
        .json(&TokenLoginRequest {
            login_tokens,
            device: DeviceInfo {
                browsername: "Prontus".to_string(),
                browserversion: "1.0.0".to_string(),
                osname: "macOS".to_string(),
                r#type: "desktop".to_string(),
                uuid: "".to_string(),
                osversion: "10.15.6".to_string(),
                appversion: "1.0.0".to_string(),
            },
        })
        .send()
        .await?;
    let json = r.json::<TokenLoginResponse>().await?;
    Ok(json)
}