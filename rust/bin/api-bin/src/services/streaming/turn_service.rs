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
    error::Result,
    WsState,
};
use types::proto::Turn;

#[proto::service("turn")]
async fn turn(stream: Stream, state: Data<WsState>) -> Result<()> {
    let avs = state.avs(stream.id().to_owned()).await;
    let user = state.user(stream.id().to_owned()).await;
    if avs.is_none() && user.is_none() {
        return stream.disconnect().await;
    }
    let turn = Turn {
        url: "turn:159.223.68.165:3478".to_owned(),
        username: "brandio".to_owned(),
        password: "brandio".to_owned(),
    };
    stream.write("turn", turn).await
}
