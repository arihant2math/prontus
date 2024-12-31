use crate::bubble_history::GetBubbleHistoryResponse;
use crate::bubble_info::GetBubbleInfoResponse;
use crate::bubble_list::GetBubbleListResponse;
use crate::bubble_mark::PostBubbleMarkRequest;
use crate::{
    bubble_create, bubble_delete, bubble_history, bubble_info, bubble_list, bubble_mark,
    bubble_membership_search, bubble_update, dm_create, membership_update,
    MembershipUpdateModification, NotificationsPreference, PostBubbleMembershipSearchRequest,
    PostMembershipUpdateRequest, ProntoClient, ResponseError,
};

impl ProntoClient {
    pub async fn create_dm(
        &self,
        organization_id: u64,
        user_id: u64,
    ) -> Result<dm_create::PostDMCreateResponse, ResponseError> {
        Ok(dm_create::post(
            &self.api_base_url,
            &self.http_client,
            dm_create::PostDMCreateRequest {
                organization_id,
                user_id,
            },
        )
        .await?
        .to_result()?)
    }

    pub async fn create_bubble(
        &self,
        organization_id: u64,
        name: String,
    ) -> Result<bubble_create::PostBubbleCreateResponse, ResponseError> {
        Ok(bubble_create::post(
            &self.api_base_url,
            &self.http_client,
            bubble_create::PostBubbleCreateRequest {
                organization_id,
                title: name,
            },
        )
        .await?
        .to_result()?)
    }

    pub async fn bubble_list(&self) -> Result<GetBubbleListResponse, ResponseError> {
        Ok(bubble_list::get(&self.api_base_url, &self.http_client)
            .await?
            .to_result()?)
    }

    pub async fn bubble_info(
        &self,
        bubble_id: u64,
    ) -> Result<GetBubbleInfoResponse, ResponseError> {
        Ok(bubble_info::get(
            &self.api_base_url,
            &self.http_client,
            bubble_info::GetBubbleInfoRequest {
                bubble_id: bubble_id,
            },
        )
        .await?
        .to_result()?)
    }

    pub async fn bubble_history(
        &self,
        bubble_id: u64,
        latest_message_id: Option<u64>,
    ) -> Result<GetBubbleHistoryResponse, ResponseError> {
        Ok(bubble_history::get(
            &self.api_base_url,
            &self.http_client,
            bubble_id,
            latest_message_id,
        )
        .await?
        .to_result()?)
    }

    pub async fn update_bubble_mark(
        &self,
        bubble_id: u64,
        message_id: u64,
    ) -> Result<bubble_mark::PostBubbleMarkResponse, ResponseError> {
        Ok(bubble_mark::post(
            &self.api_base_url,
            &self.http_client,
            PostBubbleMarkRequest {
                bubble_id,
                message_id,
            },
        )
        .await?
        .to_result()?)
    }

    pub async fn bubble_membership(
        &self,
        request: PostBubbleMembershipSearchRequest,
    ) -> Result<bubble_membership_search::PostBubbleMembershipSearchResponse, ResponseError> {
        Ok(
            bubble_membership_search::post(&self.api_base_url, &self.http_client, request)
                .await?
                .to_result()?,
        )
    }

    pub async fn pin_bubble(
        &self,
        bubble_id: u64,
        state: bool,
    ) -> Result<membership_update::PostMembershipUpdateResponse, ResponseError> {
        Ok(membership_update::post(
            &self.api_base_url,
            &self.http_client,
            PostMembershipUpdateRequest {
                bubble_id,
                modification: MembershipUpdateModification::IsPinned(state),
            },
        )
        .await?
        .to_result()?)
    }

    pub async fn hide_bubble(
        &self,
        bubble_id: u64,
    ) -> Result<membership_update::PostMembershipUpdateResponse, ResponseError> {
        Ok(membership_update::post(
            &self.api_base_url,
            &self.http_client,
            PostMembershipUpdateRequest {
                bubble_id,
                modification: MembershipUpdateModification::Hide,
            },
        )
        .await?
        .to_result()?)
    }

    pub async fn mute_bubble(
        &self,
        bubble_id: u64,
        state: bool,
    ) -> Result<membership_update::PostMembershipUpdateResponse, ResponseError> {
        if state {
            Ok(membership_update::post(
                &self.api_base_url,
                &self.http_client,
                PostMembershipUpdateRequest {
                    bubble_id,
                    modification: MembershipUpdateModification::Mute(None),
                },
            )
            .await?
            .to_result()?)
        } else {
            Ok(membership_update::post(
                &self.api_base_url,
                &self.http_client,
                PostMembershipUpdateRequest {
                    bubble_id,
                    modification: MembershipUpdateModification::Unmute,
                },
            )
            .await?
            .to_result()?)
        }
    }

    pub async fn set_bubble_alias(
        &self,
        bubble_id: u64,
        alias: Option<String>,
    ) -> Result<membership_update::PostMembershipUpdateResponse, ResponseError> {
        if let Some(alias) = alias {
            Ok(membership_update::post(
                &self.api_base_url,
                &self.http_client,
                PostMembershipUpdateRequest {
                    bubble_id,
                    modification: MembershipUpdateModification::Alias(alias),
                },
            )
            .await?
            .to_result()?)
        } else {
            Ok(membership_update::post(
                &self.api_base_url,
                &self.http_client,
                PostMembershipUpdateRequest {
                    bubble_id,
                    modification: MembershipUpdateModification::RemoveAlias,
                },
            )
            .await?
            .to_result()?)
        }
    }

    pub async fn remove_bubble_alias(
        &self,
        bubble_id: u64,
    ) -> Result<membership_update::PostMembershipUpdateResponse, ResponseError> {
        Ok(membership_update::post(
            &self.api_base_url,
            &self.http_client,
            PostMembershipUpdateRequest {
                bubble_id,
                modification: MembershipUpdateModification::RemoveAlias,
            },
        )
        .await?
        .to_result()?)
    }

    pub async fn set_bubble_notifications_preferences(
        &self,
        bubble_id: u64,
        preference: NotificationsPreference,
    ) -> Result<membership_update::PostMembershipUpdateResponse, ResponseError> {
        Ok(membership_update::post(
            &self.api_base_url,
            &self.http_client,
            PostMembershipUpdateRequest {
                bubble_id,
                modification: MembershipUpdateModification::NotificationsPreference(preference),
            },
        )
        .await?
        .to_result()?)
    }

    pub async fn pin_message(
        &self,
        bubble_id: u64,
        message_id: u64,
        end: chrono::NaiveDateTime,
    ) -> Result<bubble_update::BubbleUpdateResponse, ResponseError> {
        Ok(bubble_update::post(
            &self.api_base_url,
            &self.http_client,
            bubble_update::PostBubbleUpdateRequest {
                bubble_id,
                modification: bubble_update::BubbleUpdateModification::SetPinnedMessage((
                    message_id, end,
                )),
            },
        )
        .await?
        .to_result()?)
    }

    pub async fn unpin_message(
        &self,
        bubble_id: u64,
    ) -> Result<bubble_update::BubbleUpdateResponse, ResponseError> {
        Ok(bubble_update::post(
            &self.api_base_url,
            &self.http_client,
            bubble_update::PostBubbleUpdateRequest {
                bubble_id,
                modification: bubble_update::BubbleUpdateModification::RemovePinnedMessage(),
            },
        )
        .await?
        .to_result()?)
    }

    pub async fn set_bubble_title(
        &self,
        bubble_id: u64,
        title: String,
    ) -> Result<bubble_update::BubbleUpdateResponse, ResponseError> {
        Ok(bubble_update::post(
            &self.api_base_url,
            &self.http_client,
            bubble_update::PostBubbleUpdateRequest {
                bubble_id,
                modification: bubble_update::BubbleUpdateModification::SetTitle(title),
            },
        )
        .await?
        .to_result()?)
    }

    pub async fn set_bubble_category(
        &self,
        bubble_id: u64,
        category_id: Option<u64>,
    ) -> Result<bubble_update::BubbleUpdateResponse, ResponseError> {
        Ok(bubble_update::post(
            &self.api_base_url,
            &self.http_client,
            bubble_update::PostBubbleUpdateRequest {
                bubble_id,
                modification: bubble_update::BubbleUpdateModification::SetCategory(category_id),
            },
        )
        .await?
        .to_result()?)
    }

    pub async fn modify_bubble_permission(
        &self,
        bubble_id: u64,
        name: String,
        value: String,
    ) -> Result<bubble_update::BubbleUpdateResponse, ResponseError> {
        Ok(bubble_update::post(
            &self.api_base_url,
            &self.http_client,
            bubble_update::PostBubbleUpdateRequest {
                bubble_id,
                modification: bubble_update::BubbleUpdateModification::ModifyPermission(
                    crate::Property {
                        key: name,
                        value: serde_json::Value::String(value),
                    },
                ),
            },
        )
        .await?
        .to_result()?)
    }

    pub async fn delete_bubble(
        &self,
        bubble_id: u64,
    ) -> Result<bubble_delete::PostBubbleDeleteResponse, ResponseError> {
        Ok(bubble_delete::post(
            &self.api_base_url,
            &self.http_client,
            bubble_delete::PostBubbleDeleteRequest { bubble_id },
        )
        .await?
        .to_result()?)
    }
}
