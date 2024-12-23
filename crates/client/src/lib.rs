extern crate alloc;

use std::sync::Arc;

use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub use crate::bubble_membership_search::PostBubbleMembershipSearchRequest;
use crate::files::PutFileResponse;
pub use crate::membership_update::{
    MembershipUpdateModification, NotificationsPreference, PostMembershipUpdateRequest,
};
pub use crate::message_create::MessageModifyResponse;
pub use api_error::APIError;
pub use models::*;
pub use routes::*;

pub mod api_error;
mod client;
pub mod models;
pub mod routes;
pub(crate) mod serde_datetime;
pub(crate) mod custom_json;

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum APIResult<T> {
    Ok(T),
    Err(APIError),
}

impl<T> Clone for APIResult<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        match self {
            APIResult::Ok(t) => APIResult::Ok(t.clone()),
            APIResult::Err(e) => APIResult::Err(e.clone()),
        }
    }
}

impl<T> APIResult<T> {
    pub fn to_result(self) -> Result<T, APIError> {
        match self {
            APIResult::Ok(t) => Ok(t),
            APIResult::Err(e) => Err(e),
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum ReactionType {
    Null = -1,
    Like = 1,
    Dislike = 2,
    Laugh = 3,
    Love = 4,
    Cry = 5,
    Amazed = 6,
}

// Pronto name -> api response permission mappings
// Admin = System
// Manager = Owner
// User = Member

impl From<i32> for ReactionType {
    fn from(i: i32) -> Self {
        match i {
            1 => ReactionType::Like,
            2 => ReactionType::Dislike,
            3 => ReactionType::Laugh,
            4 => ReactionType::Love,
            5 => ReactionType::Cry,
            6 => ReactionType::Amazed,
            _ => ReactionType::Null,
        }
    }
}

#[derive(Clone)]
pub struct ProntoClient {
    pub api_base_url: String,
    pub http_client: reqwest::Client,
}

#[derive(Debug, Error)]
pub enum NewClientError {
    #[error("Reqwuest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Header parse error: {0}")]
    HeaderParseError(#[from] reqwest::header::InvalidHeaderValue),
    #[error("Url parse error: {0}")]
    UrlParseError(#[from] url::ParseError),
}

#[derive(Debug, Error)]
pub enum ResponseError {
    #[error("Reqwuest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Serde JSON error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("Serde JSON detailed error: {0}")]
    DetailedSerdeJsonError(#[from] serde_path_to_error::Error<serde_json::Error>),
    #[error("Not JSON error: {0}")]
    NotJson(String),
    #[error("API error: {0}")]
    ApiError(String),
}

impl From<APIError> for ResponseError {
    fn from(e: APIError) -> Self {
        ResponseError::ApiError(e.to_string())
    }
}

impl ProntoClient {
    /// Create a new ProntoClient with the base url and api token.
    pub fn new(api_base_url: String, pronto_api_token: &str) -> Result<Self, NewClientError> {
        // create the cookie store
        let cookies = vec![format!("api_token={}", pronto_api_token)];
        let jar = reqwest::cookie::Jar::default();
        for cookie in cookies {
            jar.add_cookie_str(&cookie, &reqwest::Url::parse(&api_base_url)?);
        }

        let mut headers = HeaderMap::new();
        headers.insert(
            "Accept",
            HeaderValue::from_str("application/json, text/plain, */*")?,
        );
        headers.insert("Accept-Language", HeaderValue::from_str("en-US,en;q=0.5")?);
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {pronto_api_token}"))?,
        );
        let client = reqwest::Client::builder()
            .cookie_store(true)
            .cookie_provider(Arc::new(jar))
            .default_headers(headers)
            .brotli(true)
            .build()?;
        Ok(Self {
            api_base_url,
            http_client: client,
        })
    }

    pub async fn upload_file(
        &self,
        filename: &str,
        file: Vec<u8>,
    ) -> Result<PutFileResponse, ResponseError> {
        Ok(files::put(
            &self.api_base_url,
            &self.http_client,
            files::PutFileRequest {
                file_name: filename.to_string(),
                file_data: file,
            },
        )
        .await?
        .to_result()?)
    }
}

// TODO: All of this
// GET https://stanfordohs.pronto.io/api/clients/files/0a43fa48-403c-4a4e-8af5-ca0c01bab35c/normalized?preset=PHOTO
// Request = None
// Response = {"data":{"original":{"mimetype":"image\/png","key":"0a43fa48-403c-4a4e-8af5-ca0c01bab35c","name":"image.png","width":1002,"height":832,"filesize":74720},"normalized":{"mimetype":"image\/png","key":"e6e3084c-7222-4241-85a6-1ee11d584a39","name":"image.png","width":1002,"height":832,"filesize":52600},"is_animated":false}}
// When sending message, see {"ok":true,"message":{"id":89171261,"bubble_id":3738656,"user_id":5302428,"message":"","resource_id":null,"clickcount":0,"likecount":0,"dislikecount":0,"viewcount":0,"version":0,"user_edited_version":0,"user_edited_at":null,"created_at":"2024-09-17 15:45:15","updated_at":"2024-09-17 15:45:15","livestream_id":null,"videosession_id":null,"systemmessageparts":null,"uuid":"e0703e87-c181-4f2e-858c-a679e16ebdf9","task_id":null,"parentmessage_id":null,"firstchildmessage_id":null,"lastchildmessage_id":null,"systemevent":null,"reactionsummary":[],"lang":null,"videosession":null,"user":{"id":5302428,"firstname":"Ashwin","lastname":"Naren","username":null,"locale":"","lastseen":"2024-09-17 15:45:15","profilepic":true,"status":0,"created_at":"2023-08-04 00:44:12","updated_at":"2024-09-17 15:44:47","deactivated_at":null,"email_verified_at":"2024-09-15 23:34:54","phone_verified_at":null,"isverified":false,"dropinorder":0,"maxstreams":10,"autotranslate":false,"isonline":true,"lastpresencetime":"2024-09-17 15:44:06","acceptedtos":"2024-09-15 23:34:54","sentwelcomemsg":"2023-08-15 19:22:02","role":"user","mute":false,"muteuntil":null,"isbot":0,"fullname":"Ashwin Naren","hasactivity":true,"inactive":false,"language":"en","permissions":{"change_name":"system","change_email":"system","change_phone":"system","remove_user":"system","change_title":"admin","change_pronouns":"admin","change_own_name":false,"change_own_email":false,"change_own_phone":false,"change_own_title":true,"change_own_pronouns":true},"profilepicpath":"\/files\/users\/5302428\/profilepic?pronto_time=1695523284","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/5302428\/profilepic?pronto_time=1695523284"},"mentions":[],"messagemedia":[{"message_id":89171261,"title":"image.png","url":"https:\/\/files.chat.trypronto.com\/files\/media\/3738656\/d56f7980-750b-11ef-9a28-6f7e119ffd69","uuid":"e6e3084c-7222-4241-85a6-1ee11d584a39","width":1002,"height":832,"filesize":52600,"duration":null,"updated_at":"2024-09-17 15:45:16","created_at":"2024-09-17 15:45:16","id":7081542,"mediatype":"PHOTO","urlmimetype":"image\/png","thumbnailmimetype":null,"path":"\/files\/media\/3738656\/d56f7980-750b-11ef-9a28-6f7e119ffd69","thumbnailpath":null}]}}

// TODO: Important: we need more data before implementation ...
// POST /api/v1/bubble.invite
// Request = {"bubble_id":3844880,"invitations":[{"user_id":5302428}],"sendemails":false,"sendsms":false}
// Response = {"ok":true,"users":[],"invitationgroups":[]}

// Also see &filter[query]=test
// relation can be [all, connections, or "filter[bubble_ids][]=2747415"]
// POST https://stanfordohs.pronto.io/api/clients/users/search?page[size]=30&filter[relation]=all
// Request = None
// Response = {"data":[{"id":5282599,"firstname":"(Clara) Ver\u00f3nica Valdano, PhD","lastname":"","fullname":"(Clara) Ver\u00f3nica Valdano, PhD","role":"user","profilepicpath":"\/files\/users\/5282599\/profilepic?pronto_time=1707263427","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/5282599\/profilepic?pronto_time=1707263427","active":true,"isbot":false,"locked":true,"deactivated_at":null,"created_at":"2023-07-31T16:22:43+00:00","updated_at":"2024-10-04T23:13:10+00:00"},{"id":6056672,"firstname":"Aadish","lastname":"Verma","fullname":"Aadish Verma","role":"user","profilepicpath":"\/files\/users\/6056672\/profilepic?pronto_time=1724175784","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/6056672\/profilepic?pronto_time=1724175784","active":true,"isbot":false,"locked":true,"deactivated_at":null,"created_at":"2024-08-01T21:35:27+00:00","updated_at":"2024-10-05T05:09:48+00:00"},{"id":5301905,"firstname":"Aaishah","lastname":"Khan","fullname":"Aaishah Khan","role":"user","profilepicpath":"\/files\/users\/5301905\/profilepic?pronto_time=-58979923200","profilepicurl":null,"active":true,"isbot":false,"locked":true,"deactivated_at":null,"created_at":"2023-08-04T00:33:22+00:00","updated_at":"2024-10-02T20:46:13+00:00"},{"id":5301877,"firstname":"Aanya","lastname":"Arunkumar","fullname":"Aanya Arunkumar","role":"user","profilepicpath":"\/files\/users\/5301877\/profilepic?pronto_time=-58979923200","profilepicurl":null,"active":true,"isbot":false,"locked":true,"deactivated_at":null,"created_at":"2023-08-04T00:33:18+00:00","updated_at":"2024-10-03T01:36:21+00:00"},{"id":5302450,"firstname":"Aanya","lastname":"Bhutani","fullname":"Aanya Bhutani","role":"user","profilepicpath":"\/files\/users\/5302450\/profilepic?pronto_time=1691183192","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/5302450\/profilepic?pronto_time=1691183192","active":true,"isbot":false,"locked":true,"deactivated_at":null,"created_at":"2023-08-04T00:44:14+00:00","updated_at":"2024-10-04T23:57:58+00:00"},{"id":5302376,"firstname":"Aanya","lastname":"Gupta","fullname":"Aanya Gupta","role":"user","profilepicpath":"\/files\/users\/5302376\/profilepic?pronto_time=1691250066","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/5302376\/profilepic?pronto_time=1691250066","active":true,"isbot":false,"locked":true,"deactivated_at":null,"created_at":"2023-08-04T00:44:05+00:00","updated_at":"2024-10-05T04:55:48+00:00"},{"id":5279681,"firstname":"Aarit","lastname":"Atreja","fullname":"Aarit Atreja","role":"user","profilepicpath":"\/files\/users\/5279681\/profilepic?pronto_time=1691438360","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/5279681\/profilepic?pronto_time=1691438360","active":true,"isbot":false,"locked":true,"deactivated_at":null,"created_at":"2023-07-28T18:15:39+00:00","updated_at":"2024-10-05T01:40:42+00:00"},{"id":5301874,"firstname":"Aariya","lastname":"Amarsaikhan","fullname":"Aariya Amarsaikhan","role":"user","profilepicpath":"\/files\/users\/5301874\/profilepic?pronto_time=1722346392","profilepicurl":null,"active":true,"isbot":false,"locked":true,"deactivated_at":null,"created_at":"2023-08-04T00:33:18+00:00","updated_at":"2024-09-15T16:12:34+00:00"},{"id":5279857,"firstname":"Aarna","lastname":"Vikram","fullname":"Aarna Vikram","role":"user","profilepicpath":"\/files\/users\/5279857\/profilepic?pronto_time=1694583255","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/5279857\/profilepic?pronto_time=1694583255","active":true,"isbot":false,"locked":true,"deactivated_at":null,"created_at":"2023-07-28T18:17:38+00:00","updated_at":"2024-10-04T21:50:41+00:00"},{"id":5302339,"firstname":"Aaron","lastname":"Bai","fullname":"Aaron Bai","role":"user","profilepicpath":"\/files\/users\/5302339\/profilepic?pronto_time=1692594969","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/5302339\/profilepic?pronto_time=1692594969","active":true,"isbot":false,"locked":true,"deactivated_at":null,"created_at":"2023-08-04T00:44:00+00:00","updated_at":"2024-10-05T02:55:08+00:00"},{"id":5301912,"firstname":"Aaron","lastname":"Lei","fullname":"Aaron Lei","role":"user","profilepicpath":"\/files\/users\/5301912\/profilepic?pronto_time=-62169984000","profilepicurl":null,"active":true,"isbot":false,"locked":true,"deactivated_at":null,"created_at":"2023-08-04T00:33:23+00:00","updated_at":"2023-08-14T05:44:59+00:00"},{"id":5301922,"firstname":"Aaron","lastname":"Mizrachi","fullname":"Aaron Mizrachi","role":"user","profilepicpath":"\/files\/users\/5301922\/profilepic?pronto_time=1715373212","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/5301922\/profilepic?pronto_time=1715373212","active":true,"isbot":false,"locked":true,"deactivated_at":null,"created_at":"2023-08-04T00:33:24+00:00","updated_at":"2024-10-02T20:29:57+00:00"},{"id":5302035,"firstname":"Aaron","lastname":"Tong","fullname":"Aaron Tong","role":"user","profilepicpath":"\/files\/users\/5302035\/profilepic?pronto_time=1694569741","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/5302035\/profilepic?pronto_time=1694569741","active":true,"isbot":false,"locked":true,"deactivated_at":null,"created_at":"2023-08-04T00:33:36+00:00","updated_at":"2024-06-10T20:47:16+00:00"},{"id":5279859,"firstname":"Aaron","lastname":"Wang","fullname":"Aaron Wang","role":"user","profilepicpath":"\/files\/users\/5279859\/profilepic?pronto_time=-58979923200","profilepicurl":null,"active":true,"isbot":false,"locked":true,"deactivated_at":null,"created_at":"2023-07-28T18:17:39+00:00","updated_at":"2024-09-29T15:42:08+00:00"},{"id":6057026,"firstname":"Abby (Russel)","lastname":"Roxas","fullname":"Abby (Russel) Roxas","role":"user","profilepicpath":"\/files\/users\/6057026\/profilepic?pronto_time=-58979923200","profilepicurl":null,"active":true,"isbot":false,"locked":true,"deactivated_at":null,"created_at":"2024-08-02T04:46:03+00:00","updated_at":"2024-08-26T16:34:38+00:00"},{"id":5302385,"firstname":"Abby","lastname":"Hurst","fullname":"Abby Hurst","role":"user","profilepicpath":"\/files\/users\/5302385\/profilepic?pronto_time=1691528554","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/5302385\/profilepic?pronto_time=1691528554","active":true,"isbot":false,"locked":true,"deactivated_at":null,"created_at":"2023-08-04T00:44:06+00:00","updated_at":"2024-10-04T22:42:34+00:00"},{"id":6056667,"firstname":"Abhinav","lastname":"Srinivas","fullname":"Abhinav Srinivas","role":"user","profilepicpath":"\/files\/users\/6056667\/profilepic?pronto_time=-58979923200","profilepicurl":null,"active":true,"isbot":false,"locked":true,"deactivated_at":null,"created_at":"2024-08-01T21:35:27+00:00","updated_at":"2024-10-05T01:54:55+00:00"},{"id":6056584,"firstname":"Abigail","lastname":"Tak","fullname":"Abigail Tak","role":"user","profilepicpath":"\/files\/users\/6056584\/profilepic?pronto_time=-62169984000","profilepicurl":null,"active":true,"isbot":false,"locked":true,"deactivated_at":null,"created_at":"2024-08-01T21:33:51+00:00","updated_at":"2024-09-11T00:27:04+00:00"},{"id":6056653,"firstname":"Adalyn","lastname":"Miller","fullname":"Adalyn Miller","role":"user","profilepicpath":"\/files\/users\/6056653\/profilepic?pronto_time=1723130987","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/6056653\/profilepic?pronto_time=1723130987","active":true,"isbot":false,"locked":true,"deactivated_at":null,"created_at":"2024-08-01T21:35:25+00:00","updated_at":"2024-10-04T18:44:48+00:00"},{"id":6056502,"firstname":"Adam","lastname":"Ali","fullname":"Adam Ali","role":"user","profilepicpath":"\/files\/users\/6056502\/profilepic?pronto_time=-58979923200","profilepicurl":null,"active":true,"isbot":false,"locked":true,"deactivated_at":null,"created_at":"2024-08-01T21:33:39+00:00","updated_at":"2024-10-01T18:24:05+00:00"},{"id":5302488,"firstname":"Adam","lastname":"Chestovaliev","fullname":"Adam Chestovaliev","role":"user","profilepicpath":"\/files\/users\/5302488\/profilepic?pronto_time=1724946557","profilepicurl":null,"active":true,"isbot":false,"locked":true,"deactivated_at":null,"created_at":"2023-08-04T00:44:16+00:00","updated_at":"2024-10-05T01:13:41+00:00"},{"id":6056541,"firstname":"Adam","lastname":"Lei","fullname":"Adam Lei","role":"user","profilepicpath":"\/files\/users\/6056541\/profilepic?pronto_time=1723856275","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/6056541\/profilepic?pronto_time=1723856275","active":true,"isbot":false,"locked":true,"deactivated_at":null,"created_at":"2024-08-01T21:33:45+00:00","updated_at":"2024-09-07T19:29:59+00:00"},{"id":5282555,"firstname":"Adam","lastname":"Lips","fullname":"Adam Lips","role":"user","profilepicpath":"\/files\/users\/5282555\/profilepic?pronto_time=1691190913","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/5282555\/profilepic?pronto_time=1691190913","active":true,"isbot":false,"locked":true,"deactivated_at":null,"created_at":"2023-07-31T16:22:37+00:00","updated_at":"2024-01-10T21:58:30+00:00"},{"id":5282556,"firstname":"Adamou","lastname":"Made","fullname":"Adamou Made","role":"user","profilepicpath":"\/files\/users\/5282556\/profilepic?pronto_time=-62169984000","profilepicurl":null,"active":true,"isbot":false,"locked":true,"deactivated_at":null,"created_at":"2023-07-31T16:22:38+00:00","updated_at":"2023-08-05T02:08:34+00:00"},{"id":6056679,"firstname":"Addie","lastname":"Zhang","fullname":"Addie Zhang","role":"user","profilepicpath":"\/files\/users\/6056679\/profilepic?pronto_time=1727120924","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/6056679\/profilepic?pronto_time=1727120924","active":true,"isbot":false,"locked":true,"deactivated_at":null,"created_at":"2024-08-01T21:35:28+00:00","updated_at":"2024-10-05T05:10:52+00:00"},{"id":5279784,"firstname":"Aden","lastname":"Mahieu","fullname":"Aden Mahieu","role":"user","profilepicpath":"\/files\/users\/5279784\/profilepic?pronto_time=1690926921","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/5279784\/profilepic?pronto_time=1690926921","active":true,"isbot":false,"locked":true,"deactivated_at":null,"created_at":"2023-07-28T18:16:49+00:00","updated_at":"2024-10-04T22:08:46+00:00"},{"id":6056655,"firstname":"Aditi","lastname":"Oak","fullname":"Aditi Oak","role":"user","profilepicpath":"\/files\/users\/6056655\/profilepic?pronto_time=1724706156","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/6056655\/profilepic?pronto_time=1724706156","active":true,"isbot":false,"locked":true,"deactivated_at":null,"created_at":"2024-08-01T21:35:25+00:00","updated_at":"2024-10-05T00:21:16+00:00"},{"id":5302340,"firstname":"Aditya","lastname":"Banerjee","fullname":"Aditya Banerjee","role":"user","profilepicpath":"\/files\/users\/5302340\/profilepic?pronto_time=1691188470","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/5302340\/profilepic?pronto_time=1691188470","active":true,"isbot":false,"locked":true,"deactivated_at":null,"created_at":"2023-08-04T00:44:00+00:00","updated_at":"2024-10-03T07:52:41+00:00"},{"id":5302648,"firstname":"Adora","lastname":"Yin","fullname":"Adora Yin","role":"user","profilepicpath":"\/files\/users\/5302648\/profilepic?pronto_time=1726107989","profilepicurl":null,"active":true,"isbot":false,"locked":true,"deactivated_at":null,"created_at":"2023-08-04T00:44:32+00:00","updated_at":"2024-10-02T13:58:02+00:00"},{"id":6056547,"firstname":"Adrian","lastname":"Ma","fullname":"Adrian Ma","role":"user","profilepicpath":"\/files\/users\/6056547\/profilepic?pronto_time=1724207080","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/6056547\/profilepic?pronto_time=1724207080","active":true,"isbot":false,"locked":true,"deactivated_at":null,"created_at":"2024-08-01T21:33:46+00:00","updated_at":"2024-10-05T04:59:50+00:00"}],"cursors":{"prev":null,"next":"eyIwIjoiQWRyaWFuIE1hIiwiMSI6NjA1NjU0NywiX3BvaW50c1RvTmV4dEl0ZW1zIjp0cnVlfQ"},"links":{"first":null,"last":null,"next":"https:\/\/stanfordohs.pronto.io\/clients\/users\/search?filter%5Brelation%5D=all&page%5Bsize%5D=30&cursor=eyIwIjoiQWRyaWFuIE1hIiwiMSI6NjA1NjU0NywiX3BvaW50c1RvTmV4dEl0ZW1zIjp0cnVlfQ","prev":null}}
// Response = {"data":[{"id":5282594,"firstname":"Instructor","lastname":"Test","fullname":"Instructor Test","role":"user","profilepicpath":"\/files\/users\/5282594\/profilepic?pronto_time=-58979923200","profilepicurl":null,"active":true,"isbot":false,"locked":true,"deactivated_at":null,"created_at":"2023-07-31T16:22:42+00:00","updated_at":"2023-07-31T16:22:42+00:00"},{"id":5277162,"firstname":"MeMeTesting12","lastname":"Test","fullname":"MeMeTesting12 Test","role":"user","profilepicpath":"\/files\/users\/5277162\/profilepic?pronto_time=-62169984000","profilepicurl":null,"active":true,"isbot":false,"locked":true,"deactivated_at":null,"created_at":"2023-07-27T02:08:26+00:00","updated_at":"2024-07-15T20:21:34+00:00"},{"id":6056471,"firstname":"Test","lastname":"Test","fullname":"Test Test","role":"user","profilepicpath":"\/files\/users\/6056471\/profilepic?pronto_time=-62169984000","profilepicurl":null,"active":true,"isbot":false,"locked":true,"deactivated_at":null,"created_at":"2024-08-01T20:19:11+00:00","updated_at":"2024-08-01T20:19:11+00:00"}],"cursors":{"prev":null,"next":null},"links":{"first":null,"last":null,"next":null,"prev":null}}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Once;

    static INIT: Once = Once::new();

    async fn get_client() -> ProntoClient {
        INIT.call_once(|| {
            simple_logger::init_with_level(log::Level::Debug).unwrap();
        });
        let settings = settings::Settings::load().await.unwrap();
        let client = ProntoClient::new(
            "https://stanfordohs.pronto.io/api/".to_string(),
            &settings.auth.unwrap().api_key,
        );
        client.unwrap()
    }

    #[tokio::test]
    async fn test_client() {
        get_client().await;
    }

    #[tokio::test]
    async fn test_get_current_user_info() {
        let client = get_client().await;
        let _response = client.current_user_info().await.unwrap();
    }

    #[tokio::test]
    async fn test_get_bubble_list() {
        let client = get_client().await;
        let _response = client.bubble_list().await.unwrap();
    }

    #[tokio::test]
    async fn test_get_bubble_info() {
        let client = get_client().await;
        let bubble_list = client.bubble_list().await.unwrap();
        let bubble_id = bubble_list.bubbles[0].id;
        let _response = client.bubble_info(bubble_id).await.unwrap();
    }

    #[tokio::test]
    async fn test_get_announcement_list() {
        let client = get_client().await;
        client.announcement_list("RECEIVED".to_string()).await.unwrap();
    }

    #[tokio::test]
    async fn test_get_task_list() {
        let client = get_client().await;
        let response = client.current_user_info().await.unwrap();
        client.task_list(response.user.organizations[0].id, false).await.unwrap();
        client.task_list(response.user.organizations[0].id, true).await.unwrap();
    }

    #[tokio::test]
    async fn test_get_bubble_history() {
        let client = get_client().await;
        let bubble_list = client.bubble_list().await.unwrap();
        let bubble_id = bubble_list.bubbles[0].id;
        let _response = client.bubble_history(bubble_id, None).await.unwrap();
    }

    #[tokio::test]
    async fn test_user_search() {
        let client = get_client().await;
        let _response = client
            .user_search(user_search::GetUserSearchRequest {
                query: "test".to_string(),
                ..Default::default()
            })
            .await
            .unwrap();
    }
}
