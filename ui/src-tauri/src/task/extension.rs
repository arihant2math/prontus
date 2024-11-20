use thiserror::Error;
use extension::ExtensionManager;

#[derive(Debug, Error)]
pub enum ExtensionThreadError {
    #[error("Extension Error: {0}")]
    ExtensionError(#[from] extension::LoadExtensionsError)
}

pub async fn run() -> Result<(), ExtensionThreadError> {
    let extensions_dir = settings::prontus_dir().join("extensions");
    let extension_manager = {
        let mut extension_manager = ExtensionManager::default();
        extension_manager.load_extensions(extensions_dir).await?;
        extension_manager
    };
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}