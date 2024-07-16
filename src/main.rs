// TODO: Support text fallback for profile pictures

use inquire::Text;
use log::{debug, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use log4rs::Config;
use std::rc::Rc;
use std::sync::mpsc;
use std::thread;

use slint::{ModelRc, VecModel, Weak};
use tokio::join;

use crate::net_worker::WorkerTasks;

pub(crate) mod client;
mod image_service;
mod net_worker;
pub(crate) mod settings;
pub(crate) mod util;
mod websocket;
mod websocket_worker;

use crate::client::user_verify::UserVerifyResult;
use crate::client::ReactionType;
use crate::settings::Settings;
pub use client::APIResult;

slint::include_modules!();

#[tokio::main]
async fn async_thread(ui_handle: Weak<AppWindow>, rx: mpsc::Receiver<WorkerTasks>) {
    let (websocket_tx, websocket_rx) = tokio::sync::mpsc::channel(128);
    let net_worker_future = net_worker::worker(ui_handle.clone(), rx, websocket_tx);
    // let websocket_worker_future = websocket_worker::worker(ui_handle, websocket_rx);
    // join!(net_worker_future, websocket_worker_future);
    join!(net_worker_future);
}

fn run() -> Result<(), slint::PlatformError> {
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
        move |channel| {
            // TODO: channel id is broken
            tx.send(WorkerTasks::ChangeChannel(channel)).unwrap();
        }
    });

    ui.on_scrollChannel({
        let ui_handle = ui.as_weak();
        let tx = tx.clone();
        move || {
            let ui = ui_handle.unwrap();
            debug!(
                "{} {} {}",
                ui.get_visible_height(),
                ui.get_viewport_y(),
                ui.get_viewport_height()
            );
            if ui.get_viewport_y() > -100.0 {
                // TODO: Do not hardcode
                let top_msg_id = ui.get_top_msg_id();
                let channel_id = ui.get_current_channel().id;
                tx.send(WorkerTasks::ScrollChannel(
                    channel_id as u64,
                    top_msg_id as u64,
                ))
                .unwrap();
            }
        }
    });

    // TODO: on resize reset viewport y

    ui.on_sendMessage({
        let ui_handle = ui.as_weak();
        let tx = tx.clone();
        move |message| {
            let ui = ui_handle.unwrap();
            tx.send(WorkerTasks::AddMessage(
                ui.get_current_channel().id as u64,
                None,
                ui.get_message().to_string(),
            ))
            .unwrap();
            ui.set_message("".to_string().into());
        }
    });

    ui.on_openLink({
        move |link| {
            open::that(&link.to_string()).unwrap();
        }
    });

    ui.on_reactionClicked({
        let tx = tx.clone();
        move |message_id, reaction_id, selected| {
            tx.send(WorkerTasks::Reaction(
                message_id as u64,
                ReactionType::from(reaction_id),
                selected,
            ))
            .unwrap();
        }
    });

    ui.on_deleteMessage({
        let tx = tx.clone();
        move |message_id| {
            tx.send(WorkerTasks::RemoveMessage(message_id as u64))
                .unwrap();
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

fn main() {
    // TODO: Better date styling
    let encoder = log4rs::encode::pattern::PatternEncoder::new(
        "[{P} {i}] {h([{d(%Y-%m-%d %H:%M:%S)} {l}])} {m}{n}",
    );
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(encoder))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))
        .unwrap();
    let _handle = log4rs::init_config(config).unwrap();

    let mut settings = Settings::load("settings.json").unwrap();
    if settings.pronto_api_token.is_none() {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let e = Text::new("What is your pronto api token?")
                .prompt()
                .unwrap();
            settings.pronto_api_token = Some(e.trim_matches(' ').to_string());
            settings.save("settings.json").unwrap();
            // TODO: prompt with real UI
        });
    }
    run().unwrap();
}
