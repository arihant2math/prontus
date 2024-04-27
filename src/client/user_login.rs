use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::client::user_info::UserInfo;

#[derive(Serialize, Deserialize)]
pub struct DeviceInfo {
    pub browsername: String,
    pub browserversion: String,
    pub osname: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserLoginRequest {
    pub email: String,
    pub code: String,
    pub device: DeviceInfo,
}

#[derive(Serialize, Deserialize)]
pub struct LoginUser {
    pub user: UserInfo,
    #[serde(rename = "logintoken")]
    pub login_token: String,
    #[serde(rename = "tokenexpiration")]
    pub token_expiration: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserLoginResponse {
    pub ok: bool,
    pub users: Vec<LoginUser>
}


pub async fn post(request: UserLoginRequest) -> UserLoginResponse {
    let client = Client::new();
    let resp = client.post("https://accounts.pronto.io/api/v3/user.login");
    let resp = resp.json(&request).send().await.unwrap();
    let resp = resp.json::<UserLoginResponse>().await.unwrap();
    return resp;
}
