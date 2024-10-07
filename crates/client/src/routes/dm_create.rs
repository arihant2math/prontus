use crate::Bubble;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct PostDMCreateRequest {
    pub user_id: u64,
    pub organization_id: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostDMCreateResponse {
    pub ok: bool,
    pub bubble: Bubble,
}

pub type PostDMCreateResult = crate::APIResult<PostDMCreateResponse>;

client_macros::api!(
    post,
    "v1/dm.create",
    PostDMCreateResult,
    PostDMCreateRequest
);
