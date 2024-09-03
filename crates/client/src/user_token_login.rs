use crate::user_info::UserInfo;
use serde::{Deserialize, Serialize};

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
    pub user: UserInfo,
}

#[derive(Serialize, Deserialize)]
pub struct TokenLoginResponse {
    pub ok: bool,
    pub users: Vec<TokenLoginUser>,
}

pub type TokenLoginResult = crate::APIResult<TokenLoginResponse>;

pub async fn post(
    pronto_base_url: &str,
    client: &reqwest::Client,
    login_tokens: Vec<String>,
) -> Result<TokenLoginResult, reqwest::Error> {
    let r = client
        .post(format!("{pronto_base_url}v1/user.tokenlogin"))
        .json(&TokenLoginRequest {
            login_tokens,
            device: DeviceInfo {
                browsername: "firefox".to_string(),
                browserversion: "130.0.0".to_string(),
                osname: "macOS".to_string(),
                r#type: "WEB".to_string(),
                uuid: "314c9314-d5e5-4ae4-84e2-9f2f3938ca28".to_string(),
                osversion: "10.15.6".to_string(),
                appversion: "1.0.0".to_string(),
            },
        })
        .send()
        .await?;
    let json = r.json::<TokenLoginResult>().await?;
    Ok(json)
}
