use log::{error, info};
use settings::Settings;
use std::path::PathBuf;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SearchError {
    #[error("Settings error: {0}")]
    Settings(#[from] settings::SettingsError),
}

// TODO: handle changes in settings like path changing
#[tokio::main]
pub async fn run_search_thread() -> Result<(), SearchError> {
    let mut future = None;
    loop {
        let settings = Settings::load().await?;
        if future.is_none() && settings.search.messages.is_some() {
            if let Some(ref message_index_settings) = settings.search.messages {
                let path = PathBuf::from(message_index_settings.path.clone());
                tokio::fs::create_dir_all(&path).await.unwrap();
                let indexer = Arc::new(
                    search::MessageIndexer::new(
                        &path,
                        search::IndexerSettings {
                            max_size: Some(message_index_settings.max_size as usize),
                        },
                    )
                    .await,
                );
                future = Some(tokio::task::spawn({
                    let indexer = indexer.clone();
                    async move {
                        let result = indexer.fastforward().await;
                        info!("Fastforward complete");
                        if let Err(e) = result {
                            error!("Error: {:?}", e);
                        }
                    }
                }));
            }
        }
        if settings.search.messages.is_none() {
            let mut set_none = false;
            if let Some(future) = future.take() {
                future.abort();
                set_none = true;
            }
            if set_none {
                future = None;
            }
        }
    }
}
