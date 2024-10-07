use crate::user_info::GetUserInfoRequest;
use crate::{user_info, ProntoClient, ResponseError};

impl ProntoClient {
    pub async fn current_user_info(&self) -> Result<user_info::GetUserInfoResponse, ResponseError> {
        self.user_info(None).await
    }

    pub async fn user_info(
        &self,
        id: Option<u64>,
    ) -> Result<user_info::GetUserInfoResponse, ResponseError> {
        Ok(user_info::get(
            &self.api_base_url,
            &self.http_client,
            GetUserInfoRequest { id },
        )
        .await?
        .to_result()?)
    }
}
