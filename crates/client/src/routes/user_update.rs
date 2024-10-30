use crate::UserInfo;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum UserUpdateModification {
    Pronouns(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserUpdateResponse {
    pub ok: bool,
    pub user: UserInfo
}
