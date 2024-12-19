use std::{error, fs};
use std::path::PathBuf;
use std::sync::Arc;
use log::warn;
use thiserror::Error;
use crate::info::ExtensionInfo;
use crate::wasm_host::WasmExtension;

mod wasm_host;
mod info;

#[derive(Debug, Error)]
pub enum LoadExtensionsError {
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Extension Error: {0}")]
    ExtensionError(#[from] wasm_host::WasmExtensionError),
    #[error("Extension Info Error: {0}")]
    ExtensionInfoError(#[from] info::ExtensionInfoCreationError),
}

#[derive(Default)]
pub struct ExtensionManager {
    extensions: Vec<WasmExtension>,
}

impl ExtensionManager {
    pub async fn load_extensions(&mut self, extensions_parent_dir: PathBuf) -> Result<(), LoadExtensionsError> {
        // Every extension is a directory in the extensions_parent_dir
        if !extensions_parent_dir.exists() {
            fs::create_dir(&extensions_parent_dir)?;
        }
        for entry in fs::read_dir(&extensions_parent_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let info: Arc<ExtensionInfo> = Arc::new(extensions_parent_dir.join("extension-manifest.toml").try_into()?);
                if !self.extensions.iter().any(|e| &e.info.id == &info.id) {
                    let extension = WasmExtension::load(path, info).await?;
                    self.extensions.push(extension);
                }
            } else {
                warn!("Ignoring non-directory entry in top-level extensions directory {extensions_parent_dir:?}: {:?}", path);
            }
        }
        Ok(())
    }

    pub async fn run_tasks(&mut

                           self) -> Result<(), Box<dyn error::Error + Send + Sync>> {
        let tasks = self.extensions.iter_mut().map(|extension| extension.run_task());
        let results = futures::future::join_all(tasks).await;
        for result in results {
            result?;
        }
        Ok(())
    }
}
