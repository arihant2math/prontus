use client::{Message, ProntoClient};
use dashmap::DashMap;
use heed::EnvOpenOptions;
use log::{debug, info};
use milli::documents::{DocumentsBatchBuilder, DocumentsBatchReader};
use milli::update::{IndexDocuments, IndexDocumentsConfig, IndexerConfig, Settings};
use milli::Index;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::Cursor;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use sysinfo::Disks;
use tokio::sync::mpsc;

#[allow(unused)]
pub fn message_index_location() -> PathBuf {
    PathBuf::from(r#"D:\tmp-milli-message-index-location"#.to_string())
}

pub fn get_index(dataset: &PathBuf) -> milli::Result<Index> {
    let disks = Disks::new_with_refreshed_list();
    let max_disk_space = disks.list().iter()
        .max_by(|&d1, &d2| d1.total_space().cmp(&d2.total_space()))
        .map(|d| d.total_space());
    let mut options = EnvOpenOptions::new();
    options.map_size(max_disk_space
        .unwrap_or(128 * 1024 * 1024 * 1024) // 128 GB
        as usize
    );

    Index::new(options, dataset.to_str().unwrap())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StoredMessage {
    pub id: u64,
    pub user_id: u64,
    pub user_firstname: String,
    pub user_lastname: String,
    pub user_fullname: String,
    pub bubble_id: u64,
    pub message: String,
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
            user_firstname: value.user.firstname,
            user_lastname: value.user.lastname,
            user_fullname: value.user.fullname,
            bubble_id: value.bubble_id,
            message: value.message,
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

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct BubbleIndexInfo {
    pub latest_message: u64,
    pub first_message: u64,
    pub upwards_index_complete: bool,
}

impl BubbleIndexInfo {
    pub fn extend(&self, first_message_span: u64, last_message_span: u64) -> Self {
        let mut new_bubble_index_info = self.clone();
        if first_message_span < new_bubble_index_info.first_message {
            new_bubble_index_info.first_message = first_message_span;
        }
        if last_message_span > new_bubble_index_info.latest_message {
            new_bubble_index_info.latest_message = last_message_span;
        }
        new_bubble_index_info
    }

    pub fn complete(&self) -> Self {
        let mut new_bubble_index_info = self.clone();
        new_bubble_index_info.upwards_index_complete = true;
        new_bubble_index_info
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageIndexInfo {
    #[serde(default)]
    pub bubbles: DashMap<u64, BubbleIndexInfo>,
}

impl MessageIndexInfo {
    // TODO: Load function

    pub fn save(&self, path: &PathBuf) -> Result<(), Box<dyn Error + Send + Sync>> {
        serde_json::to_writer(std::fs::File::create(path)?, &self)?;
        Ok(())
    }
}

#[derive(Clone, Default)]
pub struct IndexerSettings {
    /// Virtual limit imposed on index size
    pub max_size: Option<usize>,
}

pub struct MessageIndexer {
    client: ProntoClient,
    info: MessageIndexInfo,
    index_info_path: String,
    index: Index,
    mpsc_rx: Arc<Mutex<mpsc::Receiver<Message>>>,
    mpsc_tx: Arc<mpsc::Sender<Message>>,
    indexer_settings: IndexerSettings,
}

impl MessageIndexer {
    pub async fn new(index_path: &PathBuf, indexer_settings: IndexerSettings) -> Self {
        let index_info_path = index_path.join("index_info.json");
        tokio::fs::create_dir_all(index_info_path.parent().unwrap())
            .await
            .unwrap();
        if !index_info_path.exists() {
            let message_index_info = MessageIndexInfo {
                bubbles: DashMap::new(),
            };
            serde_json::to_writer(
                std::fs::File::create(&index_info_path).unwrap(),
                &message_index_info,
            )
            .unwrap();
        }
        let message_index_info: MessageIndexInfo =
            serde_json::from_reader(std::fs::File::open(&index_info_path).unwrap()).unwrap();

        let client = ProntoClient::new(
            "https://stanfordohs.pronto.io/api/".to_string(),
            "p10LpQ2V7bIJFkkhHlBNBGthClYHEUvXj2eVzpzQ.1258569865",
        )
        .unwrap();

        let (tx, rx) = mpsc::channel(512);

        Self {
            client,
            index: get_index(index_path).unwrap(),
            mpsc_rx: Arc::new(Mutex::new(rx)),
            info: message_index_info,
            index_info_path: index_info_path.to_str().unwrap().to_string(),
            mpsc_tx: Arc::new(tx),
            indexer_settings,
        }
    }

    pub async fn fastforward(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let bubble_list = &self.client.bubble_list().await?;
        // This clone is necessary so that we don't process updates from the execution function,
        // which will update the latest message after it receives messages via the mpsc.
        let mut handles = vec![];
        for (bubble, info) in self.info.bubbles.clone() {
            let stats = bubble_list
                .stats
                .iter()
                .find(|stat| stat.bubble_id == bubble)
                .unwrap();
            if stats.latest_message_id != info.latest_message {
                let handle = tokio::task::spawn({
                    let client = self.client.clone();
                    let mpsc_tx = self.mpsc_tx.clone();
                    async move {
                        loop {
                            let messages = client
                                .bubble_history(bubble, Some(info.latest_message))
                                .await
                                .unwrap();
                            let should_break = messages
                                .messages
                                .iter()
                                .any(|m| m.id <= info.latest_message);
                            for message in messages.messages {
                                mpsc_tx.send(message).await.unwrap();
                            }
                            if should_break {
                                break;
                            }
                        }
                    }
                });
                handles.push(handle);
            }
        }
        futures::future::join_all(handles).await;
        self.info
            .save(&PathBuf::from(self.index_info_path.clone()))?;
        Ok(())
    }

    pub async fn execute(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Do nothing if the index is too large
        if self.index.on_disk_size().unwrap()
            > self.indexer_settings.max_size.unwrap_or(usize::MAX) as u64
        {
            return Ok(());
        }

        let filterable_fields = vec![
            "user_id".to_string(),
            "bubble_id".to_string(),
            "parent_message_id".to_string(),
        ];
        let searchable_fields = vec!["message".to_string(), "user_fullname".to_string()];

        let bubble_list = &self.client.bubble_list().await?;
        let mut wtxn =
            tokio::time::timeout(Duration::from_secs(2), async { self.index.write_txn() })
                .await??;
        let config = IndexerConfig::default();
        let mut settings = Settings::new(&mut wtxn, &self.index, &config);
        let searchable_fields = searchable_fields.iter().map(|s| s.to_string()).collect();
        settings.set_searchable_fields(searchable_fields);
        let filterable_fields = filterable_fields.iter().map(|s| s.to_string()).collect();
        settings.set_filterable_fields(filterable_fields);
        settings.set_primary_key("id".to_string());

        settings.execute(|_| (), || false)?;

        let indexing_config = IndexDocumentsConfig::default();

        let builder = IndexDocuments::new(
            &mut wtxn,
            &self.index,
            &config,
            indexing_config,
            |_| (),
            || false,
        )
        .unwrap();

        let mut documents_batch = DocumentsBatchBuilder::new(Vec::new());
        debug!("Getting messages");
        let mut tasks = vec![];
        for channel in bubble_list.bubbles.clone() {
            let channel = channel.clone();
            tasks.push({
                let future = if let Some(index_info) = &self.info.bubbles.get(&channel.id) {
                    if index_info.upwards_index_complete {
                        None
                    } else {
                        Some(
                            self.client
                                .bubble_history(channel.id, Some(index_info.first_message)),
                        )
                    }
                } else {
                    Some(self.client.bubble_history(channel.id, None))
                };
                async move {
                    if let Some(future) = future {
                        (channel.id, future.await.unwrap().messages)
                    } else {
                        (channel.id, Default::default())
                    }
                }
            });
        }

        let new_messages = futures::future::join_all(tasks).await;
        debug!("Messages received");
        for (id, messages) in new_messages {
            if messages.len() > 0 {
                if let Some(index_info) = &self.info.bubbles.get(&id) {
                    info!(
                        "Backtrack {}: {} -> {}",
                        id,
                        index_info.first_message,
                        messages.last().unwrap().id
                    );
                } else {
                    info!(
                        "New channel {}: None -> {}",
                        id,
                        messages.last().unwrap().id
                    );
                }
            }
            for message in messages.clone().into_iter() {
                documents_batch
                    .append_json_array(
                        serde_json::to_string(&StoredMessage::from(message))
                            .unwrap()
                            .as_bytes(),
                    )
                    .unwrap();
            }
            if let Some(index_info) = &self.info.bubbles.get(&id).map(|v| v.clone()) {
                if messages.last().map(|m| m.id) == Some(index_info.first_message) {
                    self.info.bubbles.insert(id, index_info.complete());
                } else if messages.len() > 0 {
                    self.info.bubbles.insert(
                        id,
                        index_info
                            .extend(messages.last().unwrap().id, messages.first().unwrap().id),
                    );
                } else {
                    self.info.bubbles.insert(id, index_info.complete());
                }
            } else {
                if messages.len() > 0 {
                    self.info.bubbles.insert(
                        id,
                        BubbleIndexInfo {
                            latest_message: messages.first().unwrap().id,
                            first_message: messages.last().unwrap().id,
                            upwards_index_complete: false,
                        },
                    );
                }
            }
        }
        debug!("Receiving channel messages");
        debug!("Processing {} messages", self.mpsc_rx.lock().unwrap().len());
        while !self.mpsc_rx.lock().unwrap().is_empty() {
            let message = self.mpsc_rx.lock().unwrap().recv().await.unwrap();
            if let Some(index_info) = &self.info.bubbles.get(&message.bubble_id) {
                self.info
                    .bubbles
                    .insert(message.bubble_id, index_info.extend(message.id, message.id));
            } else {
                self.info.bubbles.insert(
                    message.bubble_id,
                    BubbleIndexInfo {
                        latest_message: message.id,
                        first_message: message.id,
                        upwards_index_complete: false,
                    },
                );
            }
            documents_batch
                .append_json_array(
                    serde_json::to_string(&StoredMessage::from(message))
                        .unwrap()
                        .as_bytes(),
                )
                .unwrap();
        }
        let documents_batch = documents_batch.into_inner()?;
        let documents = DocumentsBatchReader::from_reader(Cursor::new(documents_batch))?;
        let (builder, user_error) = builder.add_documents(documents)?;
        user_error?;
        builder.execute()?;
        debug!("Committing");
        wtxn.commit()?;

        self.info
            .save(&PathBuf::from(self.index_info_path.clone()))?;

        Ok(())
    }
}
