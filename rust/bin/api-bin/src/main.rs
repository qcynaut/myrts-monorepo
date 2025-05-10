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

use api_bin::services::start_ws;
use api_db::{new_api_database, repos::AvsRepo};
use proto::WsState;
use utils::{crypto::Jwt, files::ApiAssets, mail::Mail};

fn get_log_level() -> log::LevelFilter {
    match &*utils::env::load_env("LOG_LEVEL", "info") {
        "debug" => log::LevelFilter::Debug,
        "info" => log::LevelFilter::Info,
        "warn" => log::LevelFilter::Warn,
        "error" => log::LevelFilter::Error,
        _ => log::LevelFilter::Info,
    }
}

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    env_logger::builder().filter_level(get_log_level()).init();
    let database_url = utils::env::load_env(
        "DATABASE_URL",
        "postgres://postgres:postgres@localhost:5432/myrts",
    );
    let jwt_secret = utils::env::load_env("JWT_SECRET", "myrts");
    let smtp_host = utils::env::load_env("SMTP_KEY", "none");
    let smtp_from = utils::env::load_env("SMTP_FROM", "qcynaut");
    let api_assets = utils::env::load_env("API_ASSETS", "devdata/data");
    let base_url = utils::env::load_env("BASE_URL", "http://localhost:1451");
    let web_url = utils::env::load_env("WEB_URL", "http://localhost:5173");
    let log_path = utils::env::load_env("LOG_PATH", "devdata/logs");
    let api_port = utils::env::load_env("API_PORT", "1451")
        .parse::<u16>()
        .unwrap();
    let stream_port = utils::env::load_env("STREAM_PORT", "1452")
        .parse::<u16>()
        .unwrap();

    let db = match new_api_database(&database_url) {
        Ok(db) => db,
        Err(e) => {
            log::error!("Failed to connect to database: {}", e);
            std::process::exit(1);
        }
    };
    let jwt = Jwt::new(&jwt_secret);
    let mail = Mail::new(&smtp_key, &smtp_from).expect("Failed to create mail");

    let assets = ApiAssets::new(&api_assets).expect("Failed to create assets");

    match api_db::setup_initial_data(&db) {
        Ok(_) => {}
        Err(e) => {
            log::error!("Failed to setup initial data: {}", e);
            std::process::exit(1);
        }
    }

    match api_db::setup_province_data(&db).await {
        Ok(_) => {}
        Err(e) => {
            log::error!("Failed to setup province data: {}", e);
            std::process::exit(1);
        }
    }

    {
        let repo: AvsRepo = db.repository();
        let _ = repo.disconnect_all();
    }

    let ws_state = WsState::default();
    start_ws(stream_port, db.clone(), jwt.clone(), ws_state.clone()).await;

    api::run(
        api_port,
        db.clone(),
        jwt.clone(),
        mail.clone(),
        assets.clone(),
        &base_url,
        &web_url,
        ws_state,
    )
    .await
}
