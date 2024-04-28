use std::error::Error;
use std::rc::Rc;
use std::sync::mpsc;
use std::thread;

use futures_util::StreamExt;
use slint::{Model, ModelRc, VecModel, Weak};

use crate::net_worker::WorkerTasks;

pub(crate) mod client;
pub(crate) mod storage;
mod websocket;
pub(crate) mod settings;
mod net_worker;
mod websocket_worker;

pub use client::APIResult;

slint::include_modules!();

static PRONTO_BASE_URL: &str = "https://stanfordohs.pronto.io/api/";


#[tokio::main]
async fn async_thread(ui_handle: Weak<AppWindow>, rx: mpsc::Receiver<WorkerTasks>) {
    let result = net_worker::worker(ui_handle, rx).await;
    if let Err(e) = result {
        println!("{:?}", e);
    }
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

    let message_model = Rc::new(VecModel::from(vec![]));
    let model_rc = ModelRc::from(message_model.clone());
    ui.set_messages(model_rc);


    ui.on_setChannel({
        let tx = tx.clone();
        move |channel| { // TODO: channel id is broken
            tx.send(WorkerTasks::ChangeChannel(channel)).unwrap();
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
                let channel_id = ui.get_current_channel().id;
                tx.send(WorkerTasks::ScrollChannel(channel_id as u64, top_msg_id as u64)).unwrap();
            }
        }
    });

    ui.on_sendMessage({
        let ui_handle = ui.as_weak();
        let tx = tx.clone();
        move |message| {
            let ui = ui_handle.unwrap();
            tx.send(WorkerTasks::AddMessage(ui.get_current_channel().id as u64, None, ui.get_message().to_string())).unwrap();
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
