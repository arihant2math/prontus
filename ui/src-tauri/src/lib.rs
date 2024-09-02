use client::{Bubble, Message, ProntoClient, UserInfo};
use std::ops::Deref;
use std::sync::{Mutex, RwLock};
use tauri::{Manager, State};

struct AppData {
    user_info: RwLock<Option<UserInfo>>,
    users: RwLock<Vec<UserInfo>>,
    client: ProntoClient,
    channel_list: Mutex<Vec<Bubble>>,
    // TODO: atomic is better for u64
    current_channel: Mutex<u64>,
    message_list: RwLock<Vec<Message>>
}

#[tauri::command]
async fn load_user_info(state: State<'_, AppData>) -> Result<UserInfo, ()> {
    let user_info = state.client.get_user_info().await.unwrap();
    *state.user_info.write().unwrap() = Some(user_info.user.clone());
    Ok(user_info.user)
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn load_channel(state: State<'_, AppData>, id: u64) -> Result<(), ()> {
    // FIXME: This commented statement panics
    // let bubble_info = state.client.get_bubble_info(id).await.unwrap();
    *state.current_channel.lock().unwrap() = id;
    Ok(())
}

#[tauri::command]
async fn load_channel_list(state: State<'_, AppData>) -> Result<(), ()> {
    let bubbles = state.client.get_bubble_list().await.unwrap();
    *state.channel_list.lock().unwrap() = bubbles.bubbles;
    Ok(())
}

#[tauri::command]
fn get_channel_list(state: State<'_, AppData>) -> Vec<Bubble> {
    state.channel_list.lock().unwrap().clone()
}

#[tauri::command]
async fn load_messages(state: State<'_, AppData>) -> Result<(), ()> {
    let id = *state.current_channel.lock().unwrap().deref();
    let mut messages = state.client.get_bubble_history(id, None).await.unwrap();
    *state.message_list.write().unwrap() = messages.messages;
    Ok(())
}

#[tauri::command]
fn get_messages(state: State<'_, AppData>) -> Vec<Message> {
    state.message_list.read().unwrap().clone()
}

#[tauri::command]
async fn get_more_messages(state: State<'_, AppData>, last_message_id: u64) -> Result<Vec<Message>, ()> {
    let id = *state.current_channel.lock().unwrap().deref();
    let mut messages = state.client.get_bubble_history(id, Some(last_message_id)).await.unwrap();
    state.message_list.write().unwrap().append(&mut messages.messages.clone());
    Ok(messages.messages)
}

#[tauri::command]
async fn send_message(state: State<'_, AppData>, message: String) -> Result<Message, ()> {
    // FIXME: test below
    // FIXME: Message not being inputted properly (it's just blank)
    let user_id = state.user_info.read().unwrap().as_ref().unwrap().id;
    let id = *state.current_channel.lock().unwrap().deref();
    let response = state.client.post_message(user_id, id, message, None).await.unwrap();
    Ok(response.message)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let client = ProntoClient::new("https://stanfordohs.pronto.io/api/".to_string(),
                                           "DdGfHDsYKsIF9D3ZIXKShiXEUUf46Us5bXA4tSRj.1227720825")
                .unwrap();
            app.manage(AppData {
                users: RwLock::new(Vec::new()),
                client,
                channel_list: Mutex::new(Vec::new()),
                current_channel: Mutex::new(0),
                message_list: RwLock::new(Vec::new()),
                user_info: RwLock::new(None)
            });
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![load_user_info, load_channel, get_channel_list, load_channel_list, get_messages, get_more_messages, load_messages, send_message])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
