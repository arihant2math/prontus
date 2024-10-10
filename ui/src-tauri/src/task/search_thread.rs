use std::path::PathBuf;
use std::sync::Arc;
use settings::Settings;

// TODO: handle changes in settings like path changing
#[tokio::main]
pub async fn run_search_thread() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut future = None;

    loop {
        let settings = Settings::load().await?;
        if future.is_none() {
            if let Some(ref message_index_location) = settings.options.search_messages {
                let path = PathBuf::from(message_index_location);
                tokio::fs::create_dir_all(&path).await.unwrap();
                let indexer = Arc::new(search::MessageIndexer::new(&path, search::IndexerSettings::default()).await);
                future = Some(tokio::task::spawn({
                    let indexer = indexer.clone();
                    async move {
                        let result = indexer.fastforward().await;
                        println!("Fastforward complete");
                        if let Err(e) = result {
                            eprintln!("Error: {:?}", e);
                        }
                    }
                }));
            }
        }
        if settings.options.search_messages.is_none() {
            let mut set_none = false;
            if let Some(mut future) = future.take() {
                future.abort();
                set_none = true;
            }
            if set_none {
                future = None;
            }
        }
    }
}