/*
Copyright (c) 2023 Ade M Ramdani <qcynaut@gmail.com>

This software is proprietary and licensed to MyRTS under the terms of the Closed-Source Software License for Freelancers, which is available at https://dictionary.cambridge.org/us/dictionary/english/license.

MyRTS owns all right, title, and interest in and to the software, including all intellectual property rights therein.
MyRTS may use the software for any purpose, including commercial use.
MyRTS may modify the software, but only for their own internal use.
MyRTS may not distribute the software or any modified versions of the software to third parties.
MyRTS may not reverse engineer the software.
MyRTS may not create derivative works from the software.

MyRTS agrees to credit you as the developer of the software in all promotional materials and documentation for the software.

If MyRTS violates any of these terms, their license to use the software will automatically terminate.
*/

use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Root},
    Config,
};
use myrts_client::{services::start_service, states::ClientState};
use proto_db::new_proto_database;
use utils::files::ApiAssets;

fn get_log_level() -> log::LevelFilter {
    match &*utils::env::load_env("LOG_LEVEL", "info") {
        "debug" => log::LevelFilter::Debug,
        "info" => log::LevelFilter::Info,
        "warn" => log::LevelFilter::Warn,
        "error" => log::LevelFilter::Error,
        _ => log::LevelFilter::Info,
    }
}

/// configure the logger.
fn configure_logger(log_path: &str) {
    let stdout = ConsoleAppender::builder().build();
    let file = FileAppender::builder().build(log_path).unwrap();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("file", Box::new(file)))
        .build(
            Root::builder()
                .appender("stdout")
                .appender("file")
                .build(get_log_level()),
        )
        .unwrap();
    let _ = log4rs::init_config(config).unwrap();
}

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    {
        dotenvy::dotenv().ok();
    }

    let log_path = utils::env::load_env("LOG_PATH", "devdata/logs");
    let api_url = utils::env::load_env("API_URL", "ws://localhost:1452");
    let db_url = utils::env::load_env("DATABASE_URL", "devdata/db/myrts.db");
    let data_path = utils::env::load_env("DATA_PATH", "devdata/assets");
    let device_description: String = utils::env::load_env("DEVICE_DESCRIPTION", "MyRTS");
    let device_address: String = utils::env::load_env("DEVICE_ADDRESS", "myrts");
    configure_logger(&format!("{}/myrts-client.log", log_path));

    let db = new_proto_database(&db_url).unwrap();
    {
        use proto_db::repos::DeviceRepo;
        let repo = db.repository::<DeviceRepo>();
        if !repo.exists().unwrap() {
            repo.create(
                &utils::crypto::random_string(8),
                &device_description,
                &device_address,
            )
            .unwrap();
        }
    }

    let assets = ApiAssets::new(&data_path).unwrap();

    let state = ClientState::new(assets, db.clone()).await;

    loop {
        start_service(&api_url, state.clone(), db.clone()).await;
        tokio::time::sleep(tokio::time::Duration::from_secs(15)).await;
    }
}
