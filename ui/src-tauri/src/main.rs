// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    let _guard = sentry::init(("https://11ea16af2a9b5fb2bc56d6283ea0f129@o4507958552297472.ingest.us.sentry.io/4508003269410816", sentry::ClientOptions {
        release: sentry::release_name!(),
        ..Default::default()
    }));
    ui_lib::run()
}
