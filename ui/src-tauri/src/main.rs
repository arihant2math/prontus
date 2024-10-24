// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::LevelFilter;
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::filter::threshold::ThresholdFilter;
use log4rs::{Config, Handle};

fn init_logging() -> Handle {
    let level = LevelFilter::Info;
    let mut file_path = settings::prontus_dir().join("logs");
    file_path.push(format!(
        "{}.log",
        chrono::Local::now().format("%Y-%m-%d_%H-%M-%S")
    ));
    // Build a stderr logger.
    let stderr = ConsoleAppender::builder()
        .target(Target::Stderr)
        .encoder(Box::new(PatternEncoder::new("{m}\n")))
        .build();
    // Logging to log file.
    let logfile = FileAppender::builder()
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        .encoder(Box::new(PatternEncoder::new("[{l} {M} {d}] {m}\n")))
        .build(file_path.as_os_str().to_str().unwrap())
        .unwrap();
    let logfile_appender = Appender::builder().build("logfile", Box::new(logfile));
    let stderr_appender = Appender::builder()
        .filter(Box::new(ThresholdFilter::new(level)))
        .build("stderr", Box::new(stderr));
    // Create builder for log file and stderr with Trace level.
    let builder = Root::builder()
        .appender("logfile")
        .appender("stderr")
        .build(LevelFilter::Trace);
    // Log Trace level output to file where trace is the default level
    // and the programmatically specified level to stderr.
    let config = Config::builder()
        .appender(logfile_appender)
        .appender(stderr_appender)
        .logger(Logger::builder().build("reqwest", LevelFilter::Info))
        .logger(Logger::builder().build("rustls", LevelFilter::Info))
        .logger(Logger::builder().build("tokio_tungstenite", LevelFilter::Debug))
        .logger(Logger::builder().build("tungstenite", LevelFilter::Debug))
        .logger(Logger::builder().build("settings", LevelFilter::Info))
        .logger(Logger::builder().build("cookie_store", LevelFilter::Info))
        .build(builder)
        .unwrap();

    // Use this to change log levels at runtime.
    // This means you can change the default log level to trace
    // if you are trying to debug an issue and need more logs on then turn it off
    // once you are done.
    log4rs::init_config(config).expect("Logging init failed")
}

fn main() {
    let _handle = init_logging();
    let _guard = sentry::init(("https://11ea16af2a9b5fb2bc56d6283ea0f129@o4507958552297472.ingest.us.sentry.io/4508003269410816", sentry::ClientOptions {
        release: sentry::release_name!(),
        ..Default::default()
    }));
    ui_lib::run()
}
