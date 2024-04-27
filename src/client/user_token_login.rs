// https://stanfordohs.pronto.io/api/v1/user.tokenlogin

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
pub struct TokenLoginResponse {
    pub ok: bool,
    pub users: Vec<TokenLoginUser>,
}
