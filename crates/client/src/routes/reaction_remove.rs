use crate::routes::message_create::MessageModifyResult;
use crate::reaction_add::ReactionModifyRequest;

client_macros::api!(post, "v1/message.removereaction", MessageModifyResult, ReactionModifyRequest);
