use reqwest::Client;
use serde::{Serialize, Deserialize};
use serde_json::json;
use slint::{ModelRc, VecModel};
use crate::client::user_info::UserInfo;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageMedia {
    pub id: u64,
    pub url: String,
    pub mediatype: String,
    pub urlmimetype: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageResource {
    pub id: u64,
    pub providerurl: String,
    pub snippet: String,
    pub url: String,
    pub title: String,
    pub thumbnailurl: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Reactions {
    #[serde(rename = "reactiontype_id")]
    pub id: u64,
    pub count: u64,
    users: Vec<u64>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: u64,
    pub user_id: u64,
    pub bubble_id: u64,
    pub message: String,
    pub user: UserInfo,
    #[serde(default, rename = "parentmessage_id")]
    pub parent_message_id: Option<u64>,
    #[serde(default, rename = "reactionsummary")]
    pub reactions: Vec<Reactions>,
    #[serde(default, rename = "messagemedia")]
    pub message_media: Vec<MessageMedia>,
    #[serde(default)]
    pub resource: Option<MessageResource>
}

impl From<Message> for crate::Message {
    fn from(message: Message) -> Self {
        let mut embeds = Vec::new();
        if let Some(resource) = message.resource {
            embeds.push(crate::Embed {
                link: resource.url.clone().into(),
                title: resource.title.clone().into(),
                description: resource.snippet.clone().into(),
            })
        }
        crate::Message {
            id: message.id as i32,
            content: message.message.into(),
            user: message.user.fullname.into(),
            images: ModelRc::new(VecModel::from(Vec::new())),
            embeds: ModelRc::new(VecModel::from(embeds)),
            has_parent: message.parent_message_id.is_some(),
            parent_message: String::new().into() // TODO: Actually get the parent message
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetBubbleHistoryResponse {
    pub ok: bool,
    pub pagesize: u64,
    pub messages: Vec<Message>,
    pub parentmessages: Vec<Message>
}

pub async fn get(pronto_base_url: &str, client: &Client, bubble_id: u64, latest_message_id: Option<u64>) -> GetBubbleHistoryResponse {
    // TODO: catch {"ok":false,"error":"BUBBLE_NOTFOUND"}
    let r = if let Some(latest_message_id) = latest_message_id {
        client.get(format!("{pronto_base_url}v1/bubble.history"))
            .query(&json!({ "bubble_id": bubble_id, "latest": latest_message_id }))
            .send()
    } else {
        client.get(format!("{pronto_base_url}v1/bubble.history"))
            .query(&json!({ "bubble_id": bubble_id }))
            .send()
    }.await.unwrap();
    let json = r.json::<GetBubbleHistoryResponse>().await.unwrap();
    json
}
