use crate::reaction_add::ReactionModifyRequest;
use crate::routes::message_create::{MessageModifyResponse, MessageModifyResult};

client_macros::api!(
    post,
    "v1/message.removereaction",
    MessageModifyResult,
    ReactionModifyRequest
);
