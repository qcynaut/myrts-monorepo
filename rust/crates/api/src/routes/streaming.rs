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

use actix_web::web;
use api_db::{
    repos::{AvsRepo, UserRepo},
    ApiDatabase,
};
use proto::WsState;
use types::api::{
    avs::AvsResponse, package::PackageResponse, province::ProvinceResponse, role::RoleResponse,
    user::User, user_group::UserGroupResponse, CityData, OnGoingStreaming, SubscriptionData,
    UserData,
};

use crate::{middlewares::auth::Auth, ApiError};

/// # On going streaming.
///
/// This endpoint responsible to get the on going streaming.
/// ---
/// tags:
///     - streaming
/// responses:
///     - status: 200
///       content: !Vec OnGoingStreaming
///       content_type: application/json
///       description: Created record.
///     - status: 401
///       content: !T ApiError
///       content_type: application/json
///       description: Server encountered an error.
///     - status: 500
///       content: !T ApiError
///       content_type: application/json
///       description: Server encountered an error.
/// auth: api_key
#[api_rt::route(get, "/streaming/ongoing", Auth)]
async fn get_ongoing(
    user: web::ReqData<User>,
    ws_state: web::Data<WsState>,
    db: web::Data<ApiDatabase>,
) -> actix_web::Result<impl actix_web::Responder> {
    let repo = db.repository::<AvsRepo>();
    let user_repo = db.repository::<UserRepo>();
    if user.role_id == 1 || user.role_id == 2 {
        let on_going = ws_state.get_ongoing().await;
        let mut res: Vec<OnGoingStreaming> = Vec::new();

        for (user_id, avs_ids) in on_going {
            let (user, cp, g, r, p) = user_repo.get_data(user_id).map_err(ApiError::from)?;

            let user = UserData::new(
                user.id,
                user.name,
                user.email,
                user.image_url,
                RoleResponse::from(r),
                cp.map(|(c, p)| CityData::new(c.id, c.name, ProvinceResponse::new(p.id, p.name))),
                g.into_iter().map(UserGroupResponse::from).collect(),
                p.map(|(s, p)| {
                    SubscriptionData::new(
                        s.id,
                        s.user_id,
                        PackageResponse::from(p),
                        s.order_date,
                        s.expire_date,
                    )
                }),
                user.device_ids,
                user.user_group_ids,
            );
            let mut avs = Vec::new();
            for avs_id in avs_ids {
                let res = repo.get_unique(&avs_id).map_err(ApiError::from)?;
                if let Some(res) = res {
                    let mut av = AvsResponse::from(res);
                    av.cpu_temp = None;
                    av.mem_total = None;
                    av.mem_free = None;
                    av.disk_total = None;
                    av.disk_free = None;
                    avs.push(av);
                }
            }

            res.push(OnGoingStreaming::new(user, avs));
        }

        OnGoingStreaming::wrap_vec(res, Some(200))
    } else {
        let device_ids = user
            .device_ids
            .clone()
            .into_iter()
            .flatten()
            .collect::<Vec<i32>>();
        let avs = repo.get_many(&device_ids).map_err(ApiError::from)?;
        let unique_ids = avs
            .iter()
            .map(|a| a.unique_id.clone())
            .collect::<Vec<String>>();
        let on_going = ws_state.get_ongoing_from_unique(unique_ids).await;
        let mut res: Vec<OnGoingStreaming> = Vec::new();

        for (user_id, avs_ids) in on_going {
            let (user, cp, g, r, p) = user_repo.get_data(user_id).map_err(ApiError::from)?;

            let user = UserData::new(
                user.id,
                user.name,
                user.email,
                user.image_url,
                RoleResponse::from(r),
                cp.map(|(c, p)| CityData::new(c.id, c.name, ProvinceResponse::new(p.id, p.name))),
                g.into_iter().map(UserGroupResponse::from).collect(),
                p.map(|(s, p)| {
                    SubscriptionData::new(
                        s.id,
                        s.user_id,
                        PackageResponse::from(p),
                        s.order_date,
                        s.expire_date,
                    )
                }),
                user.device_ids,
                user.user_group_ids,
            );
            let mut avs = Vec::new();
            for avs_id in avs_ids {
                let res = repo.get_unique(&avs_id).map_err(ApiError::from)?;
                if let Some(res) = res {
                    let mut av = AvsResponse::from(res);
                    av.cpu_temp = None;
                    av.mem_total = None;
                    av.mem_free = None;
                    av.disk_total = None;
                    av.disk_free = None;
                    avs.push(av);
                }
            }

            res.push(OnGoingStreaming::new(user, avs));
        }

        OnGoingStreaming::wrap_vec(res, Some(200))
    }
}

api_rt::routes! {
    get_ongoing
}
