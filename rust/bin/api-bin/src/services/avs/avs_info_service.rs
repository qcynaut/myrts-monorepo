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

use api_db::{repos::AvsRepo, ApiDatabase};
use proto::{
    app::{Data, MsgData, Stream},
    error::Result,
    WsState,
};
use types::{api::avs::UpdateAvsInfo, proto::AvsInfo};

#[proto::service("avs_info")]
async fn avs_info(
    stream: Stream,
    data: MsgData<AvsInfo>,
    ws_state: Data<WsState>,
    db: Data<ApiDatabase>,
) -> Result<()> {
    let data = data.into_inner();
    let avs_id = if let Some(avs) = ws_state.avs_id(stream.id().to_owned()).await {
        avs
    } else {
        return Ok(());
    };
    let repo = db.repository::<AvsRepo>();
    let avs = if let Ok(avs) = repo.get_unique(&avs_id) {
        if let Some(avs) = avs {
            avs
        } else {
            return Ok(());
        }
    } else {
        return Ok(());
    };

    let update = UpdateAvsInfo {
        networks: data.networks,
        mem_total: data.mem_total,
        mem_free: data.mem_free,
        disk_total: data.disk_total,
        disk_free: data.disk_free,
        cpu_temp: data.cpu_temp,
    };

    let _ = repo.update_os_info(update, avs.id);
    Ok(())
}
