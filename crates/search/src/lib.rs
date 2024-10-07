use client::{Message, ProntoClient, UserInfo};
use heed::EnvOpenOptions;
use milli::documents::{DocumentsBatchBuilder, DocumentsBatchReader};
use milli::update::{IndexDocuments, IndexDocumentsConfig, IndexerConfig, Settings};
use milli::Index;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::io::Cursor;
use std::path::PathBuf;
use std::sync::Arc;

pub fn get_index(dataset: &str) -> milli::Result<Index> {
    let mut options = EnvOpenOptions::new();
    options.map_size(1 * 1024 * 1024 * 1024); // 1 GB

    Index::new(options, dataset)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StoredMessage {
    pub id: u64,
    pub user_id: u64,
    pub bubble_id: u64,
    pub message: String,
    pub user: UserInfo,
    pub system_event: Option<String>,
    pub parent_message_id: Option<u64>,
    pub first_child_message_id: Option<u64>,
    pub last_child_message_id: Option<u64>,
    pub created_at: String,
    pub message_resource_id: Option<u64>,
    pub message_resource_providerurl: Option<String>,
    pub message_resource_snippet: Option<String>,
    pub message_resource_url: Option<String>,
    pub message_resource_title: Option<String>,
    pub message_resource_thumbnailurl: Option<String>,
}

impl From<Message> for StoredMessage {
    fn from(value: Message) -> Self {
        Self {
            id: value.id,
            user_id: value.user_id,
            bubble_id: value.bubble_id,
            message: value.message,
            user: value.user,
            system_event: value.system_event,
            parent_message_id: value.parent_message_id,
            first_child_message_id: value.first_child_message_id,
            last_child_message_id: value.last_child_message_id,
            created_at: value.created_at.to_string(),
            message_resource_id: value.resource.as_ref().map(|r| r.id),
            message_resource_providerurl: value.resource.as_ref().map(|r| r.providerurl.clone()),
            message_resource_snippet: value.resource.as_ref().map(|r| r.snippet.clone()),
            message_resource_url: value.resource.as_ref().map(|r| r.url.clone()),
            message_resource_title: value.resource.as_ref().map(|r| r.title.clone()),
            message_resource_thumbnailurl: value.resource.as_ref().map(|r| r.thumbnailurl.clone()),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BubbleIndexInfo {
    pub latest_message: u64,
    pub first_message: u64,
    // TODO: use this
    pub upwards_index_complete: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageIndexInfo {
    #[serde(default)]
    pub bubbles: HashMap<u64, BubbleIndexInfo>
}

pub struct MessageIndexer {
    client: ProntoClient,
    info: MessageIndexInfo,
    index_info_path: String,
}

impl MessageIndexer {
    pub async fn new(index_path: &str, index_info_path: &str) -> Self {
        tokio::fs::create_dir_all(&index_path).await.unwrap();
        std::fs::create_dir_all(&PathBuf::from(index_info_path).parent().unwrap()).unwrap();
        if !PathBuf::from(index_info_path).exists() {
            let message_index_info = MessageIndexInfo { bubbles: HashMap::new() };
            serde_json::to_writer(std::fs::File::create(index_info_path).unwrap(), &message_index_info).unwrap();
        }
        let message_index_info: MessageIndexInfo = serde_json::from_reader(std::fs::File::open(index_info_path).unwrap()).unwrap();

        let client = ProntoClient::new("https://stanfordohs.pronto.io/api/".to_string(), "p10LpQ2V7bIJFkkhHlBNBGthClYHEUvXj2eVzpzQ.1258569865").unwrap();

        Self {
            client,
            info: message_index_info,
            index_info_path: index_info_path.to_string(),
        }
    }

    pub async fn execute(&mut self, index: Arc<Index>) -> Result<(), Box<dyn Error + Send + Sync>> {
        let filterable_fields = vec!["user_id".to_string(), "bubble_id".to_string(), "parent_message_id".to_string()];
        let searchable_fields = vec!["message".to_string()];

        let bubble_list = &self.client.bubble_list().await?;
        let mut wtxn = index.write_txn()?;
        let config = IndexerConfig::default();
        let mut settings = Settings::new(&mut wtxn, &index, &config);
        let searchable_fields = searchable_fields.iter().map(|s| s.to_string()).collect();
        settings.set_searchable_fields(searchable_fields);
        let filterable_fields = filterable_fields.iter().map(|s| s.to_string()).collect();
        settings.set_filterable_fields(filterable_fields);
        settings.set_primary_key("id".to_string());

        settings.execute(|_| (), || false)?;

        let config = IndexerConfig::default();
        let indexing_config = IndexDocumentsConfig::default();

        let builder =
            IndexDocuments::new(&mut wtxn, &index, &config, indexing_config, |_| (), || false).unwrap();

        let mut documents_batch = DocumentsBatchBuilder::new(Vec::new());
        let mut tasks = vec![];
        for channel in bubble_list.bubbles.clone() {
            let channel = channel.clone();
            tasks.push({
                let future = if let Some(index_info) = &self.info.bubbles.get(&channel.id) {
                    self.client.bubble_history(channel.id, Some(index_info.latest_message))
                } else {
                    self.client.bubble_history(channel.id, None)
                };
                async move {
                    (channel.id, future.await.unwrap())
                }
            });
        }
        let new_messages = futures::future::join_all(tasks).await;
        for (id, messages) in new_messages {
            for message in messages.messages.clone().into_iter() {
                documents_batch.append_json_array(serde_json::to_string(&StoredMessage::from(message)).unwrap().as_bytes()).unwrap();
            }
            if let Some(index_info) = &self.info.bubbles.get(&id) {
                if messages.messages.len() > 0 {
                    self.info.bubbles.insert(id, BubbleIndexInfo { latest_message: messages.messages.first().unwrap().id, first_message: index_info.first_message, upwards_index_complete: false });
                } else {
                    self.info.bubbles.insert(id, BubbleIndexInfo { latest_message: index_info.latest_message, first_message: index_info.first_message, upwards_index_complete: true });
                }
            } else {
                if messages.messages.len() > 0 {
                    self.info.bubbles.insert(id, BubbleIndexInfo { latest_message: messages.messages.first().unwrap().id, first_message: messages.messages.last().unwrap().id, upwards_index_complete: false });
                }
            }
        }
        let documents_batch = documents_batch.into_inner()?;
        let documents = DocumentsBatchReader::from_reader(Cursor::new(documents_batch))?;
        let (builder, user_error) = builder.add_documents(documents)?;
        user_error.unwrap();
        builder.execute().unwrap();
        println!("Committing");
        wtxn.commit()?;

        serde_json::to_writer(std::fs::File::create(&self.index_info_path)?, &self.info)?;
        Ok(())
    }
}
