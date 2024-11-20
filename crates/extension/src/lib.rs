use std::{error, fs};
use std::path::PathBuf;
use std::sync::Arc;
use log::warn;
use crate::wasm_host::WasmExtension;

mod wasm_host;
mod info;

#[derive(Default)]
pub struct ExtensionManager {
    extensions: Vec<WasmExtension>,
}

impl ExtensionManager {
    pub async fn load_extensions(&mut self, extensions_parent_dir: PathBuf) -> Result<(), Box<dyn error::Error + Send + Sync>> {
        // Every extension is a directory in the extensions_parent_dir
        for entry in fs::read_dir(&extensions_parent_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let info = Arc::new(extensions_parent_dir.join("extension.toml").try_into()?);
                let extension = WasmExtension::load(path, info).await?;
                self.extensions.push(extension);
            } else {
                warn!("Ignoring non-directory entry in top-level extensions directory {extensions_parent_dir:?}: {:?}", path);
            }
        }
        Ok(())
    }
}
