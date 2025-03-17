use crate::announcement_list::GetAnnouncementListRequest;
use crate::{
    ProntoClient, ResponseError, announcement_create, announcement_list, announcement_mark_read,
};

impl ProntoClient {
    pub async fn create_announcement(
        &self,
        target_bubbles: Vec<u64>,
        announcement: String,
    ) -> Result<announcement_create::PostAnnouncementCreateResponse, ResponseError> {
        Ok(announcement_create::post(
            &self.api_base_url,
            &self.http_client,
            announcement_create::PostAnnouncementCreateRequest {
                targets: announcement_create::PostAnnouncementCreateRequestTargets {
                    bubble_ids: target_bubbles,
                },
                announcement,
            },
        )
        .await?
        .to_result()?)
    }

    pub async fn announcement_list(
        &self,
        query: String,
    ) -> Result<announcement_list::GetAnnouncementListResponse, ResponseError> {
        Ok(announcement_list::get(
            &self.api_base_url,
            &self.http_client,
            GetAnnouncementListRequest {
                query,
                per_page: 20,
            },
        )
        .await?
        .to_result()?)
    }

    pub async fn mark_read_announcement(
        &self,
        announcement_id: u64,
    ) -> Result<announcement_mark_read::GetAnnouncementMarkReadResponse, ResponseError> {
        Ok(announcement_mark_read::get(
            &self.api_base_url,
            &self.http_client,
            announcement_mark_read::GetAnnouncementMarkReadRequest { announcement_id },
        )
        .await?
        .to_result()?)
    }
}
