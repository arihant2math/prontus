use crate::routes::message_create::{MessageModifyResponse, MessageModifyResult};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct ReactionModifyRequest {
    pub message_id: u64,
    #[serde(rename = "reactiontype_id")]
    pub reaction_type_id: u64,
}

client_macros::api!(
    post,
    "v1/message.addreaction",
    MessageModifyResult,
    ReactionModifyRequest
);
