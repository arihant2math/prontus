use client::Message;
use tauri::{Emitter, State, command};
use ui_lib::{AppState, BackendError};

#[command]
pub async fn send_message(
    handle: tauri::AppHandle,
    state: State<'_, AppState>,
    message: String,
    thread: Option<u64>,
) -> Result<(), BackendError> {
    let response = {
        let state = state.try_inner()?;
        let user_id = state.user_info.id;
        let id = state
            .current_channel
            .read()
            .map_err(|_| BackendError::RwLockReadError)?
            .id;
        state
            .client
            .send_message(user_id, id, message, thread)
            .await?
    };

    let state = state.try_inner()?;
    let mut message_list = state
        .message_list
        .write()
        .map_err(|_| BackendError::RwLockWriteError)?;
    if message_list
        .iter()
        .find(|m| m.id == response.message.id)
        .is_some()
    {
        return Ok(());
    }
    message_list.insert(0, response.message);
    let _ = handle.emit("messageListUpdate", ());
    Ok(())
}

#[command]
pub async fn load_messages(state: State<'_, AppState>) -> Result<(), BackendError> {
    let messages = {
        let state = state.try_inner()?;

        let id = state
            .current_channel
            .read()
            .map_err(|_| BackendError::RwLockReadError)?
            .id;
        state.client.bubble_history(id, None).await?
    };
    let state = state.try_inner()?;

    for message in messages.messages.iter() {
        if !state.users.contains_key(&message.user.id) {
            state.users.insert(message.user.id, message.user.clone());
        }
    }

    let mut message_list = state
        .message_list
        .write()
        .map_err(|_| BackendError::RwLockWriteError)?;
    *message_list = messages.messages;
    let mut parent_messages = state
        .parent_messages
        .write()
        .map_err(|_| BackendError::RwLockWriteError)?;
    *parent_messages = messages.parent_messages;
    Ok(())
}

#[command]
pub async fn get_message(
    state: State<'_, AppState>,
    id: u64,
) -> Result<Option<Message>, BackendError> {
    let state = state.try_inner()?;

    let message_list = state
        .message_list
        .write()
        .map_err(|_| BackendError::RwLockWriteError)?;
    Ok(message_list
        .iter()
        .find(|message| message.id == id)
        .cloned())
}

#[command]
pub async fn get_messages(state: State<'_, AppState>) -> Result<Vec<Message>, BackendError> {
    let state = state.try_inner()?;

    Ok(state
        .message_list
        .read()
        .map_err(|_| BackendError::RwLockReadError)?
        .clone())
}

#[command]
pub async fn get_parent_messages(state: State<'_, AppState>) -> Result<Vec<Message>, BackendError> {
    let state = state.try_inner()?;

    Ok(state
        .message_list
        .read()
        .map_err(|_| BackendError::RwLockReadError)?
        .clone())
}

#[command]
pub async fn get_more_messages(
    state: State<'_, AppState>,
    last_message_id: u64,
) -> Result<Vec<Message>, BackendError> {
    let mut messages = {
        let state = state.try_inner()?;

        let id = state
            .current_channel
            .read()
            .map_err(|_| BackendError::RwLockReadError)?
            .id;
        state
            .client
            .bubble_history(id, Some(last_message_id))
            .await?
    };

    let state = state.try_inner()?;
    for message in messages.messages.iter() {
        if !state.users.contains_key(&message.user.id) {
            state.users.insert(message.user.id, message.user.clone());
        }
    }
    let mut message_list = state
        .message_list
        .write()
        .map_err(|_| BackendError::RwLockWriteError)?;
    message_list.extend_from_slice(&mut messages.messages.clone());
    let mut parent_messages = state
        .parent_messages
        .write()
        .map_err(|_| BackendError::RwLockWriteError)?;
    parent_messages.extend_from_slice(&mut messages.parent_messages);
    Ok(messages.messages)
}

#[command]
pub async fn edit_message(
    handle: tauri::AppHandle,
    state: State<'_, AppState>,
    message_id: u64,
    message: String,
) -> Result<(), BackendError> {
    let message = {
        let state = state.try_inner()?;
        state.client.edit_message(message_id, message).await?
    };
    let state = state.try_inner()?;
    let mut message_list = state
        .message_list
        .write()
        .map_err(|_| BackendError::RwLockWriteError)?;
    *message_list
        .iter_mut()
        .find(|m| m.id == message_id)
        .unwrap() = message.message;
    let _ = handle.emit("messageListUpdate", ());
    Ok(())
}

#[command]
pub async fn delete_message(
    handle: tauri::AppHandle,
    state: State<'_, AppState>,
    message_id: u64,
) -> Result<(), BackendError> {
    {
        let state = state.try_inner()?;
        state.client.delete_message(message_id).await?;
    }
    let state = state.try_inner()?;
    let mut message_list = state
        .message_list
        .write()
        .map_err(|_| BackendError::RwLockWriteError)?;
    message_list.retain(|message| message.id != message_id);
    let _ = handle.emit("messageListUpdate", ());
    Ok(())
}
