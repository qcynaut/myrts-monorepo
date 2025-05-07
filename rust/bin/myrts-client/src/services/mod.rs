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

use crate::states::ClientState;
use proto::{
    app::{App, Data},
    client::Client,
};
use proto_db::ProtoDatabase;

mod command;
mod lifecycle;
mod streaming;
mod syncing;

/// Start the service.
pub async fn start_service(url: &str, state: ClientState, db: ProtoDatabase) {
    let app = App::new()
        .add_state(Data::new(state))
        .add_state(Data::new(db))
        .service(lifecycle::start)
        .service(lifecycle::pong)
        .service(lifecycle::end)
        .service(lifecycle::authenticated)
        .service(syncing::sync)
        .service(syncing::resync)
        .service(streaming::offer)
        .service(streaming::ices)
        .service(streaming::stream_close)
        .service(streaming::volume)
        .service(command::command);

    if let Err(e) = Client::new(app, url).run().await {
        log::error!("Error: {}", e);
    }
}
