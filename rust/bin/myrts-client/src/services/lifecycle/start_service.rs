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

use proto::{
    app::{Data, Stream},
    error::{OtherError, Result},
};
use proto_db::{repos::DeviceRepo, ProtoDatabase};
use types::proto::Authenticate;

#[proto::service("start")]
async fn start(stream: Stream, db: Data<ProtoDatabase>) -> Result<()> {
    log::info!("Starting service");
    let repo = db.repository::<DeviceRepo>();
    let device = repo.get().map_err(|e| OtherError::String(e.to_string()))?;
    let auth = Authenticate {
        client_id: device.uid,
        client_type: 2,
        client_description: device.description,
        client_address: device.address,
    };
    log::info!("Authenticating");
    stream.write("auth", auth).await?;
    stream.write("ping", "").await
}
