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

use crate::states::stream::StreamingState;
use api_db::ApiDatabase;
use proto::{
    app::{App, Data},
    server::Server,
    WsState,
};
use utils::crypto::Jwt;

pub(crate) mod auth;
pub(crate) mod avs;
pub(crate) mod lifecycle;
pub(crate) mod streaming;
pub(crate) mod sync;

/// Start ws server.
pub async fn start_ws(port: u16, db: ApiDatabase, jwt: Jwt, state: WsState) {
    let streaming_state = StreamingState::new(state.streaming());
    let app = App::new()
        .add_state(Data::new(state))
        .add_state(Data::new(db))
        .add_state(Data::new(jwt))
        .add_state(Data::new(streaming_state))
        .service(lifecycle::start)
        .service(lifecycle::ping)
        .service(lifecycle::end)
        .service(auth::auth)
        .service(sync::sync)
        .service(streaming::turn)
        .service(streaming::offer)
        .service(streaming::answer)
        .service(streaming::ices)
        .service(streaming::volume)
        .service(avs::avs_info)
        .service(avs::command);

    tokio::spawn(async move {
        match Server::new(app, port).run().await {
            Ok(_) => {
                std::process::exit(0);
            }
            Err(e) => {
                log::error!("Failed to start server: {}", e);
                std::process::exit(1);
            }
        }
    });
}
