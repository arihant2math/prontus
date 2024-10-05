// POST https://stanfordohs.pronto.io/api/v1/bubble.delete
// Request = {"bubble_id":"3844880"}
// Response = {"ok":true}

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostBubbleDeleteRequest {
    pub bubble_id: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostBubbleDeleteResponse {
    pub ok: bool,
}

pub type PostBubbleDeleteResult = crate::APIResult<PostBubbleDeleteResponse>;

client_macros::api!(post, "v1/bubble.delete", PostBubbleDeleteResult, PostBubbleDeleteRequest);
