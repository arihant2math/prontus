use crate::BackendError;
use settings::Settings;
use tauri::command;

#[command]
pub async fn get_settings() -> Result<Settings, BackendError> {
    Ok(Settings::load().await?)
}

#[command]
pub async fn set_settings(settings: Settings) -> Result<(), BackendError> {
    settings.save().await?;
    Ok(())
}
