use serde::{Serialize, Deserialize};

pub enum UserVerifyRequest {
    // TODO: Email type
    Email(String),
    // example: +14086692442
    // Which is +[country code][phone number]
    Phone(String), // TODO: Phone type
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserVerifyResponseSuccess {
    pub ok: bool,
    pub length: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserVerifyResponseError {
    pub ok: bool,
    pub error: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum UserVerifyResponse {
    Success(UserVerifyResponseSuccess),
    Error(UserVerifyResponseError),
}
