mod extension;
mod proxy;
mod pusher;
mod search;

use futures::join;
use log::error;
use tauri::AppHandle;
use ui_lib::AppState;

#[tokio::main]
pub async fn task_thread(handle: AppHandle, context: AppState) {
    // spawn tasks
    let f1 = tokio::task::spawn({
        let context = context.clone();
        async move {
            if let Err(e) = pusher::run(handle, context).await {
                error!("Pusher Task Error: {:?}", e);
            }
        }
    });
    let f2 = tokio::task::spawn(async move {
        if let Err(e) = search::run().await {
            error!("Search Task Error: {:?}", e);
        }
    });
    let f3 = tokio::task::spawn(async move {
        if let Err(e) = proxy::run(context).await {
            error!("Proxy Task Error: {:?}", e);
        }
    });
    let f4 = tokio::task::spawn(async move {
        if let Err(e) = extension::run().await {
            error!("Extension Task Error: {:?}", e);
        }
    });
    let _ = join!(f1, f2, f3);
}
