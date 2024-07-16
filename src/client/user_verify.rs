use serde::{Serialize, Deserialize};
use serde_json::json;
use crate::APIResult;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum UserVerifyRequest {
    // TODO: Email type
    #[serde(rename = "email")]
    Email(String),
    // example: +14086692442
    // Which is +[country code][phone number]
    #[serde(rename = "phone")]
    Phone(String), // TODO: Phone type
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserVerifyResponse {
    pub ok: bool,
    pub length: u16,
}

pub type UserVerifyResult = APIResult<UserVerifyResponse>;

pub async fn post(request: UserVerifyRequest) -> Result<UserVerifyResult, reqwest::Error> {
    let client = reqwest::Client::new();
    let resp = match request {
        UserVerifyRequest::Email(email) => client.post("https://accounts.pronto.io/api/v1/user.verify").json(&json!({"email": email})),
        UserVerifyRequest::Phone(phone) => client.post("https://accounts.pronto.io/api/v1/user.verify").json(&json!({"phone": phone})),
    };
    let resp = resp.send().await?;
    let resp = resp.json().await?;
    Ok(resp)
}
