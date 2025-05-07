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

mod resync_service;
pub(super) use resync_service::resync;
mod sync_service;
pub(super) use sync_service::sync;

#[macro_export]
macro_rules! send_sync {
    ($db: expr, $stream: expr) => {
        use proto::error::OtherError;
        use proto_db::repos::ScheduleRepo;
        use types::proto::SyncReq;
        let repo = $db.repository::<ScheduleRepo>();
        let schedule_ids = repo
            .get_sids()
            .map_err(|e| OtherError::String(e.to_string()))?;
        let sync = SyncReq {
            local: schedule_ids,
        };
        $stream.write("sync", sync).await?;
    };
}
