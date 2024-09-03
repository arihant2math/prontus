use std::collections::HashMap;
use std::ops::Deref;

use client::{Bubble, Message, ProntoClient, UserInfo};
use tokio::sync::RwLock;
use tauri::{Manager, State};
use tauri::ipc::InvokeError;
use client::user_login::{DeviceInfo, UserLoginRequest};

#[derive(Debug, thiserror::Error)]
enum BackendError {
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

struct AppData {
    user_info: UserInfo,
    users: HashMap<u64, UserInfo>,
    client: ProntoClient,
    channel_list: Vec<Bubble>,
    current_channel: u64,
    message_list: Vec<Message>
}

enum InnerAppState {
    Unloaded,
    Loaded(AppData),
}

impl InnerAppState {
    fn try_inner(&self) -> Result<&AppData, BackendError> {
        match self {
            InnerAppState::Loaded(data) => Ok(data),
            InnerAppState::Unloaded => Err(BackendError::NotLoaded),
        }
    }

    fn try_inner_mut(&mut self) -> Result<&mut AppData, BackendError> {
        match self {
            InnerAppState::Loaded(data) => Ok(data),
            InnerAppState::Unloaded => Err(BackendError::NotLoaded),
        }
    }
}

struct AppState {
    inner: RwLock<InnerAppState>
}

impl AppState {
    async fn is_loaded(&self) -> bool {
        match self.inner.read().await.deref() {
            InnerAppState::Loaded(_) => true,
            InnerAppState::Unloaded => false,
        }
    }

    fn inner(&self) -> &RwLock<InnerAppState> {
        &self.inner
    }

    fn unloaded() -> Self {
        Self {
            inner: RwLock::new(InnerAppState::Unloaded)
        }
    }
}

#[tauri::command]
async fn get_code(state: State<'_, AppState>, email: String) -> Result<(), BackendError> {
    let response = client::user_verify::post(client::user_verify::UserVerifyRequest::Email(email)).await.unwrap().to_result();
    // TODO: Error handling
    Ok(())
}

#[tauri::command]
async fn send_code(state: State<'_, AppState>, email: String, code: String) -> Result<(), BackendError> {
    let response = client::user_login::post(UserLoginRequest {
        email,
        code,
        // TODO: Fix
        device: DeviceInfo {
            browsername: "".to_string(),
            browserversion: "".to_string(),
            osname: "".to_string(),
            r#type: "".to_string(),
        },
    }).await.unwrap().to_result().unwrap();
    let token = &response.users[0].login_token;
    let mut settings = settings::Settings::load()?;
    settings.api_key = Some(token.clone());
    settings.save()?;
    // TODO: This is the part where we can switch base urls
    let client = ProntoClient::new("https://stanfordohs.pronto.io/api/".to_string(), token).unwrap();
    // TODO: Standardize device info
    let response = client.user_token_login(token, DeviceInfo {
        browsername: "".to_string(),
        browserversion: "".to_string(),
        osname: "".to_string(),
        r#type: "".to_string(),
    }).await?;

    let mut settings = settings::Settings::load()?;
    settings.api_key = Some(response.users[0].access_token.clone());
    settings.save()?;
    // TODO: Error handling as usual
    Ok(())
}

#[tauri::command]
async fn load(state: State<'_, AppState>) -> Result<(), BackendError> {
    if state.is_loaded().await {
        return Ok(());
    }
    let settings = settings::Settings::load()?;
    let client = ProntoClient::new("https://stanfordohs.pronto.io/api/".to_string(), &settings.api_key.ok_or(BackendError::NotAuthenticated)?).unwrap();
    let user_info_future = client.get_user_info();
    let channel_list_future = client.get_bubble_list();
    let (user_info, channel_list) = futures::join!(user_info_future, channel_list_future);
    let user_info = user_info?.user;
    let mut users = HashMap::new();
    users.insert(user_info.id, user_info.clone());
    let channel_list = channel_list?.bubbles;
    let data = AppData {
        user_info,
        users,
        client,
        channel_list,
        current_channel: 0,
        message_list: vec![]
    };
    *state.inner().inner().write().await = InnerAppState::Loaded(data);
    Ok(())
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn load_channel(state: State<'_, AppState>, id: u64) -> Result<(), BackendError> {
    let state = state.inner().inner();
    let mut state = state.write().await;
    let state = state.try_inner_mut()?;

    // FIXME: This commented statement panics
    // let bubble_info = state.client.get_bubble_info(id).await.unwrap();
    state.current_channel = id;
    Ok(())
}

#[tauri::command]
async fn get_channel_list(state: State<'_, AppState>) -> Result<Vec<Bubble>, BackendError> {
    let state = state.inner().inner();
    let state = state.read().await;
    let state = state.try_inner()?;

    Ok(state.channel_list.clone())
}

#[tauri::command]
async fn load_messages(state: State<'_, AppState>) -> Result<(), BackendError> {
    let state = state.inner().inner();
    let mut state = state.write().await;
    let state = state.try_inner_mut()?;

    let id = state.current_channel;
    let messages = state.client.get_bubble_history(id, None).await?;
    state.message_list = messages.messages;
    Ok(())
}

#[tauri::command]
async fn get_messages(state: State<'_, AppState>) -> Result<Vec<Message>, BackendError> {
    let state = state.inner().inner();
    let state = state.read().await;
    let state = state.try_inner()?;

    Ok(state.message_list.clone())
}

#[tauri::command]
async fn get_more_messages(state: State<'_, AppState>, last_message_id: u64) -> Result<Vec<Message>, BackendError> {
    let state = state.inner().inner();
    let mut state = state.write().await;
    let state = state.try_inner_mut()?;

    let id = state.current_channel;
    let messages = state.client.get_bubble_history(id, Some(last_message_id)).await?;
    let messages = messages.messages;
    state.message_list.extend_from_slice(&mut messages.clone());
    Ok(messages)
}

#[tauri::command]
async fn send_message(state: State<'_, AppState>, message: String) -> Result<Message, BackendError> {
    let state = state.inner().inner();
    let mut state = state.write().await;
    let state = state.try_inner_mut()?;

    let user_id = state.user_info.id;
    let id = state.current_channel;
    let response = state.client.post_message(user_id, id, message, None).await?;
    Ok(response.message)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let client = ProntoClient::new("https://stanfordohs.pronto.io/api/".to_string(),
                                           "DdGfHDsYKsIF9D3ZIXKShiXEUUf46Us5bXA4tSRj.1227720825")
                .unwrap();
            app.manage(AppState::unloaded());
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_code, send_code, load, load_channel, get_channel_list, get_messages, get_more_messages, load_messages, send_message])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
