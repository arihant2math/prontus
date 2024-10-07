use crate::models::Bubble;
use crate::{BubbleStats, Membership};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetBubbleListResponse {
    pub ok: bool,
    pub bubbles: Vec<Bubble>,
    pub stats: Vec<BubbleStats>,
    pub memberships: Vec<Membership>,
}

pub type GetBubbleListResult = crate::APIResult<GetBubbleListResponse>;

client_macros::api!(get, "v3/bubble.list", GetBubbleListResult, !);
