use std::sync::mpsc;
use std::thread;

use futures_util::StreamExt;
use slint::Weak;
use tokio_tungstenite::connect_async;
use crate::net_worker::WorkerTasks;

pub(crate) mod client;
pub(crate) mod storage;
mod websocket;
pub(crate) mod settings;
mod net_worker;

slint::include_modules!();

static PRONTO_BASE_URL: &str = "https://stanfordohs.pronto.io/api/";

#[derive(Clone, Debug)]
enum WebsocketTasks {
    ChangeChannel(u64)
}

async fn websocket_worker(ui: Weak<AppWindow>, rx: mpsc::Receiver<WebsocketTasks>) {
    let url = url::Url::parse("wss://ws-mt1.pusher.com/app/f44139496d9b75f37d27?protocol=7&client=js&version=8.3.0&flash=false").unwrap();
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");

    let (write, read) = ws_stream.split();
    // TODO
}

#[tokio::main]
async fn async_thread(ui_handle: Weak<AppWindow>, rx: mpsc::Receiver<WorkerTasks>) {
    net_worker::worker(ui_handle, rx).await.unwrap();
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let (tx, rx) = mpsc::channel::<WorkerTasks>();

    thread::spawn({
        let ui_handle = ui.as_weak();
        move || {
            async_thread(ui_handle, rx);
        }
    });


    ui.on_setChannel({
        let tx = tx.clone();
        let ui_handle = ui.as_weak();
        move |channel_id| { // TODO: channel id is broken
            let ui = ui_handle.unwrap();
            tx.send(WorkerTasks::ChangeChannel(ui.get_current_sidebar_item_id() as u64)).unwrap();
        }
    });

    ui.on_scrollChannel({
        let ui_handle = ui.as_weak();
        let tx = tx.clone();
        move || {
            let ui = ui_handle.unwrap();
            println!("{} {} {}", ui.get_visible_height(), ui.get_viewport_y(), ui.get_viewport_height());
            if ui.get_viewport_y() > -100.0 { // TODO: Do not hardcode
                let top_msg_id = ui.get_top_msg_id();
                let channel_id = ui.get_channel_id();
                tx.send(WorkerTasks::ScrollChannel(channel_id as u64, top_msg_id as u64)).unwrap();
            }
        }
    });

    ui.on_sendMessage({
        let ui_handle = ui.as_weak();
        let tx = tx.clone();
        move |message| {
            let ui = ui_handle.unwrap();
            tx.send(WorkerTasks::AddMessage(ui.get_channel_id() as u64, None, ui.get_message().to_string())).unwrap();
            ui.set_message("".to_string().into());
        }
    });

    ui.on_openLink({
        move |link| {
            open::that(&link.to_string()).unwrap();
        }
    });


    // let image = storage::load_image(Arc::clone(&client), image.url.clone()).await;
    // let buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
    //     image.as_raw(),
    //     image.width(),
    //     image.height(),
    // );
    // let slint_image = Image::from_rgba8(buffer);

    ui.run()
}
