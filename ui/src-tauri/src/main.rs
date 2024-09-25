// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log4rs::append::console::ConsoleAppender;
use log4rs::Config;
use log4rs::config::{Appender, Root};
use log::LevelFilter;

fn init_logging() -> log4rs::Handle {
    let stdout = ConsoleAppender::builder().build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Warn))
        .unwrap();

    let handle = log4rs::init_config(config).unwrap();
    handle
}

fn main() {
    let _handle = init_logging();
    let _guard = sentry::init(("https://11ea16af2a9b5fb2bc56d6283ea0f129@o4507958552297472.ingest.us.sentry.io/4508003269410816", sentry::ClientOptions {
        release: sentry::release_name!(),
        ..Default::default()
    }));
    ui_lib::run()
}
