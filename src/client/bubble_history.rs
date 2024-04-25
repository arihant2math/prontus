use reqwest::Client;
use serde::{Serialize, Deserialize};
use serde_json::json;
use slint::{Image, ModelRc, Rgba8Pixel, SharedPixelBuffer, VecModel};
use crate::client::user_info::UserInfo;
use crate::storage::fast_load_image;

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

impl Message {
    pub fn to_slint(self, parents: &Vec<Message>) -> crate::Message {
        let mut embeds = Vec::new();
        if let Some(resource) = self.resource {
            embeds.push(crate::Embed {
                link: resource.url.clone().into(),
                title: resource.title.clone().into(),
                description: resource.snippet.clone().into(),
            })
        }
        let mut images = Vec::new();

        let temp_image = SharedPixelBuffer::<Rgba8Pixel>::new(100, 100); // TODO: load that default.jpg image ...
        for _ in self.message_media {
            images.push(Image::from_rgba8(temp_image.clone()));
        }

        let profile_picture = match fast_load_image(&self.user.profilepicurl) {
            Some(image) => SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
                image.as_raw(),
                image.width(),
                image.height(),
            ),
            None => temp_image.clone(),
        };


        let parent = if let Some(parent_id) = self.parent_message_id {
            parents.iter().find(|parent| parent.id == parent_id)
        } else { None };
        let mut reactions = Vec::new();
        for reaction in self.reactions {
            reactions.push(crate::Reaction {
                id: reaction.id as i32,
                user_ids: ModelRc::new(VecModel::from(reaction.users.iter().map(|id| *id as i32).collect::<Vec<i32>>()))
            });
        }
        crate::Message {
            id: self.id as i32,
            content: self.message.into(),
            user: self.user.fullname.into(),
            profile_picture: Image::from_rgba8(temp_image.clone()),
            images: ModelRc::new(VecModel::from(images)),
            embeds: ModelRc::new(VecModel::from(embeds)),
            has_parent: self.parent_message_id.is_some(),
            parent_message: parent.map(|parent| parent.message.clone()).unwrap_or_default().into(), // TODO: This should really be the previous message in the thread tbh
            reactions: ModelRc::new(VecModel::from(reactions)),
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
