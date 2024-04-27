use serde::{Serialize, Deserialize};

pub enum UserVerifyRequest {
    Email(String), // TODO: Email type
    // example: +14086692442
    // Which is +[country code][phone number]
    Phone(String), // TODO: Phone type
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserVerifyRequestSuccess {
    pub ok: bool,
    pub length: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserVerifyRequestError {
    pub ok: bool,
    pub error: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum UserVerifyResponse {
    Success(UserVerifyResponseSuccess),
    Error(UserVerifyResponseError),
}
