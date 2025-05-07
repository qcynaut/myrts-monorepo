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
    app::{Data, MsgData},
    error::{OtherError, Result},
};
use proto_db::{models::NewSchedule, repos::ScheduleRepo, ProtoDatabase};
use types::proto::Sync;

#[proto::service("sync")]
async fn sync(
    data: MsgData<Sync>,
    state: Data<ClientState>,
    db: Data<ProtoDatabase>,
) -> Result<()> {
    let data = data.into_inner();
    let repo = db.repository::<ScheduleRepo>();
    repo.remove(data.remove.clone())
        .map_err(|e| OtherError::String(e.to_string()))?;
    for schedule in data.add.clone() {
        repo.create(NewSchedule {
            sid: schedule.sid,
            name: schedule.name,
            days: serde_json::to_string(&schedule.days).unwrap(),
            record_url: schedule.record_url,
            kind: schedule.kind,
            weeks: serde_json::to_string(&schedule.weeks).unwrap(),
            dates: serde_json::to_string(&schedule.dates).unwrap(),
            times: serde_json::to_string(&schedule.times).unwrap(),
            month: schedule.month,
            year: schedule.year,
            volume: schedule.volume.map(|v| v as f64),
        })
        .map_err(|e| OtherError::String(e.to_string()))?;
    }
    if !data.remove.is_empty() || !data.add.is_empty() {
        state.update_schedule();
    }
    Ok(())
}
