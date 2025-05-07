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
use api_db::{
    repos::{AvsRepo, UserRepo},
    ApiDatabase,
};
use proto::{
    app::{Data, MsgData, Stream},
    error::Result,
    WsState,
};
use std::collections::HashMap;
use types::proto::{Offer, WsErr};

#[proto::service("offer")]
async fn offer(
    stream: Stream,
    data: MsgData<Offer>,
    ws_state: Data<WsState>,
    stream_state: Data<StreamingState>,
    db: Data<ApiDatabase>,
) -> Result<()> {
    let user = if let Some(user) = ws_state.user_id(stream.id().to_owned()).await {
        user
    } else {
        stream.disconnect().await?;
        return Ok(());
    };
    if data.target.is_empty() {
        return stream
            .write(
                "offer:fail",
                WsErr {
                    msg: "target avs is empty".to_owned(),
                },
            )
            .await;
    }
    {
        let repo = db.repository::<UserRepo>();
        let avs_repo = db.repository::<AvsRepo>();
        let user = if let Ok(user) = repo.get(user) {
            user
        } else {
            stream.disconnect().await?;
            return Ok(());
        };
        if user.role_id == 3 {
            for target_id in data.target.iter() {
                let avs_id = avs_repo
                    .get_unique(&target_id)
                    .map(|a| a.map(|a| a.id))
                    .unwrap_or(Some(0))
                    .unwrap_or(0);
                if !user.device_ids.contains(&Some(avs_id)) {
                    return stream
                        .write(
                            "offer:fail",
                            WsErr {
                                msg: "target avs not found".to_owned(),
                            },
                        )
                        .await;
                }
            }
        }
    }
    let mut target = HashMap::new();
    for target_id in data.target.iter() {
        if let Some(avs) = ws_state.avs_by_id(target_id.to_string()).await {
            target.insert(target_id.to_string(), avs);
        } else {
            log::debug!(
                "target avs {} not found in {:?}",
                target_id,
                ws_state.get_ref()
            );
        }
    }
    if target.is_empty() {
        return stream
            .write(
                "offer:fail",
                WsErr {
                    msg: "target avs not found".to_owned(),
                },
            )
            .await;
    }
    tokio::spawn(async move {
        stream_state
            .new_streaming(stream, user, data.offer.clone(), target)
            .await;
    });
    Ok(())
}
