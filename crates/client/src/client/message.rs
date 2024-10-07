use crate::message_edit::MessageEditRequest;
use crate::{
    message_create, message_delete, message_edit, MessageModifyResponse, ProntoClient,
    ResponseError,
};
use chrono::Utc;

impl ProntoClient {
    pub async fn send_message(
        &self,
        user_id: u64,
        bubble_id: u64,
        message: String,
        parent_message_id: Option<u64>,
    ) -> Result<MessageModifyResponse, ResponseError> {
        Ok(message_create::post(
            &self.api_base_url,
            &self.http_client,
            bubble_id,
            message,
            user_id,
            Utc::now(),
            parent_message_id,
        )
        .await?
        .to_result()?)
    }

    pub async fn edit_message(
        &self,
        message_id: u64,
        message: String,
    ) -> Result<MessageModifyResponse, ResponseError> {
        Ok(message_edit::post(
            &self.api_base_url,
            &self.http_client,
            MessageEditRequest {
                message_id,
                message,
            },
        )
        .await?
        .to_result()?)
    }

    pub async fn delete_message(
        &self,
        message_id: u64,
    ) -> Result<message_delete::DeleteMessageResult, ResponseError> {
        Ok(
            message_delete::post(&self.api_base_url, &self.http_client, message_id)
                .await?
                .to_result()?,
        )
    }
}
