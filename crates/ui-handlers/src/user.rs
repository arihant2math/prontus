use client::UserInfo;
use log::debug;
use tauri::{command, State};
use ui_lib::{AppState, BackendError};

#[command]
pub async fn get_current_user(state: State<'_, AppState>) -> Result<UserInfo, BackendError> {
    let state = state.inner().inner();
    let state = state.read().await;
    let state = state.try_inner()?;

    Ok(state.user_info.clone())
}

#[command]
pub async fn get_user(state: State<'_, AppState>, id: u64) -> Result<UserInfo, BackendError> {
    let user_info = {
        let state = state.clone().inner().inner();
        let state = state.read().await;
        let state = state.try_inner()?;

        if let Some(user) = state.users.get(&id) {
            return Ok(user.clone());
        }
        debug!("{:?}", state.users.keys());
        state.client.user_info(Some(id)).await?
    };

    let state = state.inner().inner();
    let mut state = state.write().await;
    let state = state.try_inner_mut()?;
    state
        .users
        .insert(user_info.user.id, user_info.user.clone());

    Ok(user_info.user)
}
