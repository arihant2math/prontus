use crate::{task_complete, task_list, ProntoClient, ResponseError};

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

    pub async fn task_complete(&self, task_id: u64) -> Result<task_complete::PostTaskCompleteResponse, ResponseError> {
        Ok(task_complete::post(
            &self.api_base_url,
            &self.http_client,
            task_complete::PostTaskCompleteRequest {
                task_id,
            },
        )
            .await?
            .to_result()?)
    }
}