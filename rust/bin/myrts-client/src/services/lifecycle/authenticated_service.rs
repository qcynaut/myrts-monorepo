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

use crate::{get_os_info, send_sync};
use proto::{
    app::{Data, Stream},
    error::Result,
};
use proto_db::ProtoDatabase;

#[proto::service("authenticated")]
async fn authenticated(stream: Stream, db: Data<ProtoDatabase>) -> Result<()> {
    log::info!("Syncing with server");
    let stream_clone = stream.clone();
    tokio::spawn(async move {
        let stream = stream_clone;
        let mut tick = tokio::time::interval(std::time::Duration::from_secs(60 * 5));
        loop {
            tick.tick().await;
            let info = get_os_info();
            match stream.write("avs_info", info).await {
                Ok(_) => {}
                Err(e) => match e {
                    proto::error::Error::Connection(_) => break,
                    _ => {}
                },
            }
        }
    });
    send_sync!(db, stream);
    Ok(())
}
