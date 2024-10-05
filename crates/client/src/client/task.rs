use crate::{task_list, ProntoClient, ResponseError};

impl ProntoClient {
    pub async fn task_list(&self, organization_id: u64, completed: bool) -> Result<task_list::PostTaskListResponse, ResponseError> {
        Ok(task_list::post(
            &self.api_base_url,
            &self.http_client,
            task_list::PostTaskListRequest {
                organization_id,
                completed,
            },
        )
            .await?
            .to_result()?)
    }
}