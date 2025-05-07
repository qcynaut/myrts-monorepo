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
use proto::{
    app::{Data, MsgData, Stream},
    error::Result,
    WsState,
};
use types::proto::Answer;

#[proto::service("answer")]
async fn answer(
    stream: Stream,
    data: MsgData<Answer>,
    ws_state: Data<WsState>,
    stream_state: Data<StreamingState>,
) -> Result<()> {
    let avs_id = if let Some(avs_id) = ws_state.avs_id(stream.id().to_owned()).await {
        avs_id
    } else {
        stream.disconnect().await?;
        return Ok(());
    };
    tokio::spawn(async move {
        let avs_id = avs_id.clone();
        stream_state
            .add_answer_forwarder(&avs_id, data.answer.clone())
            .await;
    });
    Ok(())
}
