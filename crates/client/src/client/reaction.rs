use crate::{reaction_add, reaction_remove, MessageModifyResponse, ProntoClient, ReactionType, ResponseError};
use crate::reaction_add::ReactionModifyRequest;

impl ProntoClient {
    pub async fn add_reaction(
        &self,
        message_id: u64,
        reaction_type: ReactionType,
    ) -> Result<MessageModifyResponse, ResponseError> {
        Ok(reaction_add::post(
            &self.api_base_url,
            &self.http_client,
            ReactionModifyRequest {
                message_id,
                reaction_type_id: reaction_type as i32 as u64
            },
        )
            .await?
            .to_result()?)
    }

    pub async fn remove_reaction(
        &self,
        message_id: u64,
        reaction_type: ReactionType,
    ) -> Result<MessageModifyResponse, ResponseError> {
        Ok(reaction_remove::post(
            &self.api_base_url,
            &self.http_client,
            ReactionModifyRequest {
                message_id,
                reaction_type_id: reaction_type as i32 as u64
            }
        )
            .await?
            .to_result()?)
    }
}