use client::{Bubble, Message, ProntoClient, UserInfo};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::{Mutex, RwLock};
use tauri::{Manager, State};

struct AppData {
    users: RwLock<Vec<UserInfo>>,
    client: ProntoClient,
    channel_list: Mutex<Vec<Bubble>>,
    // TODO: atomic is better
    current_channel: Mutex<u64>,
    message_list: RwLock<Vec<Message>>
}

#[tauri::command]
fn get_user_by_name(state: State<'_, AppData>, name: &str) -> UserInfo {
    let users = state.users.read().unwrap();
    users.iter().find(|u| u.fullname == name).unwrap().clone()
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn load_channel(state: State<'_, AppData>, id: u64) -> Result<(), ()> {
    // FIXME: This panics
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
    messages.messages.reverse();
    *state.message_list.write().unwrap() = messages.messages;
    Ok(())
}

#[tauri::command]
fn get_messages(state: State<'_, AppData>) -> Vec<Message> {
    state.message_list.read().unwrap().clone()
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
                message_list: RwLock::new(Vec::new())
            });
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![load_channel, get_channel_list, load_channel_list, get_messages, load_messages])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
