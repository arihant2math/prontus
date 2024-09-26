use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct PostBubbleMarkRequest {
    pub bubble_id: u64,
    pub message_id: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostBubbleMarkResponse {
    pub ok: bool
}

pub type PostBubbleMarkResult = crate::APIResult<PostBubbleMarkResponse>;

client_macros::api!(post, "v1/bubble.mark", PostBubbleMarkResult, PostBubbleMarkRequest);
