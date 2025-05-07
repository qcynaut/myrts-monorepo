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

use api_db::{
    repos::{AvsRepo, ScheduleRepo},
    ApiDatabase,
};
use proto::{
    app::{Data, MsgData, Stream},
    error::Result,
    WsState,
};
use types::proto::{Schedule, Sync, SyncReq};

#[proto::service("sync")]
async fn sync(
    stream: Stream,
    data: MsgData<SyncReq>,
    db: Data<ApiDatabase>,
    state: Data<WsState>,
) -> Result<()> {
    let data = data.into_inner();
    let avs = match state.avs_id(stream.id().to_owned()).await {
        Some(avs) => avs,
        None => {
            let _ = stream.disconnect().await;
            return Ok(());
        }
    };
    let repo = db.repository::<AvsRepo>();
    let schedule_repo = db.repository::<ScheduleRepo>();

    let avs = match repo.get_unique(&avs) {
        Ok(res) => {
            if let Some(avs) = res {
                avs
            } else {
                let _ = stream.disconnect().await;
                return Ok(());
            }
        }
        Err(_) => {
            let _ = stream.disconnect().await;
            return Ok(());
        }
    };
    let schedules = match schedule_repo.get_by_avs_with_records(avs.id) {
        Ok(schedules) => schedules,
        Err(_) => {
            let _ = stream.disconnect().await;
            return Ok(());
        }
    };

    let mut filtered = vec![];
    let mut new_local = vec![];
    for (schedule, record) in schedules {
        new_local.push(schedule.id);
        let mut volume = None;
        for schedule_volume in schedule.volumes {
            if let Some(v) = schedule_volume {
                let split: Vec<&str> = v.split(':').collect();
                let id = split[0].parse::<i32>().unwrap_or(0);
                if id == avs.id {
                    volume = Some(split[1].parse::<f32>().unwrap_or(1.0));
                }
            }
        }
        if !data.local.contains(&schedule.id) {
            filtered.push(Schedule {
                sid: schedule.id,
                name: schedule.name,
                days: schedule.days,
                dates: schedule.dates,
                times: schedule.times,
                record_url: record.file_url,
                kind: schedule.kind,
                weeks: schedule.weeks,
                month: schedule.month,
                year: schedule.year,
                volume,
            });
        }
    }

    let remove = data
        .local
        .into_iter()
        .filter(|schedule_id| !new_local.contains(schedule_id))
        .collect::<Vec<i32>>();
    stream
        .write(
            "sync",
            Sync {
                add: filtered,
                remove,
            },
        )
        .await
}
