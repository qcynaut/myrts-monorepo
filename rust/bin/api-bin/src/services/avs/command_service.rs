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
    app::{Data, Msg, Stream},
    error::Result,
    WsState,
};
use types::proto::{CmdRequest, CmdResponse};

#[proto::service("command")]
async fn command(stream: Stream, data: Msg, ws_state: Data<WsState>) -> Result<()> {
    if let Some(_) = ws_state.avs_id(stream.id().to_string()).await {
        let data: CmdResponse = match data.deserialize() {
            Ok(data) => data,
            Err(err) => {
                log::error!("Failed to parse command data: {}", err);
                return Ok(());
            }
        };

        if let Some(user) = ws_state.user_by_id(data.sender).await {
            return user.write("command", data).await;
        } else {
            log::warn!("User not found: {}", data.sender);
            return Ok(());
        }
    } else if let Some(_) = ws_state.user_id(stream.id().to_string()).await {
        let data: CmdRequest = match data.deserialize() {
            Ok(data) => data,
            Err(err) => {
                log::error!("Failed to parse command data: {}", err);
                return Ok(());
            }
        };

        if let Some(avs) = ws_state.avs_by_id(data.target.clone()).await {
            return avs.write("command", data).await;
        } else {
            log::warn!("AVS not found: {}", data.target);
            return Ok(());
        }
    } else {
        return Ok(());
    }
}
