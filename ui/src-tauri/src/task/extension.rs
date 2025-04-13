use extension::ExtensionManager;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExtensionThreadError {
    #[error("Extension Load Error: {0}")]
    ExtensionLoadError(#[from] extension::LoadExtensionsError),
    #[error("Extension Runtime Error: {0}")]
    ExtensionRuntimeError(anyhow::Error),
}

pub async fn run() -> Result<(), ExtensionThreadError> {
    let extensions_dir = settings::prontus_dir().join("extensions");
    let mut extension_manager = {
        let mut extension_manager = ExtensionManager::default();
        extension_manager.load_extensions(extensions_dir).await?;
        extension_manager
    };

    extension_manager
        .run_tasks()
        .await
        .map_err(|e| ExtensionThreadError::ExtensionRuntimeError(e))?;
    Ok(())
}
