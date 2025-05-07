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
    app::{Data, Stream},
    WsState,
};
use timeslots::TimeSlots;
use types::{api::avs::NewAvs, proto::Authenticate};

pub(super) async fn auth(
    stream: Stream,
    data: Authenticate,
    state: Data<WsState>,
    db: Data<ApiDatabase>,
) {
    let repo = db.repository::<AvsRepo>();
    let avs = match repo.get_unique(&data.client_id) {
        Ok(val) => {
            if let Some(avs) = val {
                avs
            } else {
                let avs = repo.create(NewAvs {
                    unique_id: data.client_id.clone(),
                    description: Some(data.client_description),
                    status: 1,
                    lat: None,
                    lng: None,
                    address: Some(data.client_address),
                    kind: 1,
                    pending: 1,
                    slots: TimeSlots::new().into(),
                    networks: None,
                    mem_total: None,
                    mem_free: None,
                    disk_total: None,
                    disk_free: None,
                    cpu_temp: None,
                });
                if let Err(_) = avs {
                    let _ = stream.disconnect().await;
                    return;
                }
                state.set_avs(data.client_id, stream).await;
                return;
            }
        }
        Err(_) => {
            let _ = stream.disconnect().await;
            return;
        }
    };
    if let Some(_) = state.avs_by_id(avs.unique_id.clone()).await {
        let _ = stream.disconnect().await;
        return;
    }
    state.set_avs(data.client_id, stream.clone()).await;
    let _ = repo.connect(&avs.unique_id);
    if avs.pending == 0 {
        let _ = stream.write("authenticated", "").await;
    }
}
