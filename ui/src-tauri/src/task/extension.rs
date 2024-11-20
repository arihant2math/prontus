use tauri::AppHandle;
use thiserror::Error;
use settings::SettingsError;
use ui_lib::AppState;

#[derive(Debug, Error)]
pub enum ExtensionThreadError {
}

pub async fn run() -> Result<(), ExtensionThreadError> {
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}