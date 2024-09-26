use crate::routes::message_create::MessageModifyResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageEditRequest {
    pub message: String,
    pub message_id: u64,
}

client_macros::api!(post, "v1/message.edit", MessageModifyResult, MessageEditRequest);
