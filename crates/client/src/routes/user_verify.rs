use std::{error, fs};
use std::path::PathBuf;
use std::sync::Arc;
use log::warn;
use thiserror::Error;
use crate::info::ExtensionInfo;
pub use crate::wasm_host::WasmExtension;

mod wasm_host;
pub mod info;

pub const EXTENSION_FILE_NAME: &str = "extension.wasm";
pub const MANIFEST_FILE_NAME: &str = "manifest.toml";

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
        