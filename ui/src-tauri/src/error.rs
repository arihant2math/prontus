use tauri::ipc::InvokeError;
use crate::state;

#[derive(Debug, thiserror::Error)]
pub enum BackendError {
    #[error("Unlock error: {0}")]
    UnlockError(#[from] state::UnlockError),
    #[error("The application state has not been loaded yet")]
    NotLoaded,
    #[error("The user is not authenticated")]
    NotAuthenticated,
    #[error("Response error: {0}")]
    ResponseError(#[from] client::ResponseError),
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Settings error: {0}")]
    SettingsError(#[from] settings::SettingsError)
}

impl Into<InvokeError> for BackendError {
    fn into(self) -> InvokeError {
        InvokeError::from_error(self)
    }
}
