use tauri::ipc::InvokeError;

#[derive(Debug, thiserror::Error)]
pub enum BackendError {
    #[error("The application state has not been loaded yet")]
    NotLoaded,
    #[error("The user is not authenticated")]
    NotAuthenticated,
    #[error("Response error: {0}")]
    ResponseError(#[from] client::ResponseError),
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

impl Into<InvokeError> for BackendError {
    fn into(self) -> InvokeError {
        InvokeError::from_error(self)
    }
}
