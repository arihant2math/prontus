use std::thread;
use tauri::Manager;

mod task;
#[cfg(desktop)]
mod tray;

pub use ui_lib::{AppState, BackendError};

use ui_handlers::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let context = AppState::unloaded();
            let thread_handle = app.handle().clone();
            thread::spawn({
                let context = context.clone();
                move || {
                    task::task_thread(thread_handle, context);
                }
            });

            app.manage(context);

            let main_window = app.get_window("main").unwrap();
            main_window.set_title("Prontus")?;

            #[cfg(all(desktop))]
            {
                let handle = app.handle();
                tray::create_tray(handle)?;
            }

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_code,
            send_code,
            load,
            load_channel,
            get_channel_info,
            get_current_user,
            get_user,
            get_current_channel,
            get_channel_list,
            get_message,
            get_messages,
            get_more_messages,
            get_parent_messages,
            load_messages,
            edit_message,
            send_message,
            set_reaction_state,
            delete_message,
            get_channel_users,
            load_channel_users,
            get_settings,
            set_settings,
            set_channel_mute,
            set_channel_pin,
            set_channel_alias,
            set_channel_notifications,
            set_channel_title,
            set_channel_category,
            modify_channel_permission,
            delete_channel,
            read_channel,
            create_dm,
            create_bubble,
            user_search,
            get_announcements,
            mark_announcement_read,
            get_tasks,
            complete_task,
            uncomplete_task,
            delete_task,
            set_typing,
            get_typing_users
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    app.run(|_app_handle, event| match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {}
    });
}
