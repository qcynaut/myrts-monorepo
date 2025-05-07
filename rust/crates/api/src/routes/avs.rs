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

use crate::{middlewares::auth::Auth, ApiError};
use actix_web::{web, Responder};
use api_db::{repos::AvsRepo, ApiDatabase};
use proto::WsState;
use timeslots::TimeSlots;
use types::api::{
    avs::{AvsResponse, PartialUpdateAvs},
    user::User,
    Message,
};

/// # Get all avs.
///
/// This endpoint returns all avs with the following rules:
///
/// If the user role is `1 (Root)` or `2 (SuperAdmin)` it will returns all avs, even if the avs are on pending status `3 (pending)`.
/// If the user role is `3 (Admin)` it will returns only the avs that are connected to the user itself, and only show active avs.
/// `pending`: `1 (pending)` and `0 (active)`.
/// `status`: `1 (connected)` and `2 (disconnected)`.
/// ---
/// tags:
///     - avs
/// responses:
///     - status: 200
///       content: !Vec AvsResponse
///       description: List of available avs.
///     - status: 401
///       content: !T ApiError
///       description: The user is not authenticated.
///     - status: 500
///       content: !T ApiError
///       description: Server encountered an error.
/// auth: api_key
#[api_rt::route(get, "/avs", Auth)]
async fn get(
    user: web::ReqData<User>,
    db: web::Data<ApiDatabase>,
) -> actix_web::Result<impl actix_web::Responder> {
    let repo = db.repository::<AvsRepo>();
    if user.role_id < 3 {
        let res = repo.get_all().map_err(ApiError::from)?;
        AvsResponse::wrap_vec(res.into_iter().map(AvsResponse::from).collect(), None)
    } else {
        let mut avs_ids = Vec::new();
        for id in user.device_ids.iter() {
            if let Some(id) = id {
                avs_ids.push(*id);
            }
        }
        let res = repo.get_many(&avs_ids).map_err(ApiError::from)?;
        AvsResponse::wrap_vec(
            res.into_iter()
                .map(|a| {
                    let mut avs = AvsResponse::from(a);
                    avs.networks = None;
                    avs.mem_total = None;
                    avs.mem_free = None;
                    avs.disk_total = None;
                    avs.disk_free = None;
                    avs.cpu_temp = None;
                    avs
                })
                .collect(),
            None,
        )
    }
}

/// # Update avs info.
///
/// This endpoint responsible for updating information such as address, description, kind, etc.
/// ****Rules:****
/// Only Root and SuperAdmin can access this endpoint.
/// ---
/// tags:
///     - avs
/// request:
///     content: !T PartialUpdateAvs
///     content_type: application/json
///     description: The data to partially update an avs.
/// responses:
///     - status: 200
///       content: !T AvsResponse
///       description: The avs is updated.
///     - status: 400
///       content: !T ApiError
///       description: The request is invalid.
///     - status: 401
///       content: !T ApiError
///       description: The user is not authenticated.
///     - status: 404
///       content: !T ApiError
///       description: The avs is not found.
///     - status: 500
///       content: !T ApiError
///       description: Server encountered an error.
/// params:
///     - name: id
///       kind: !Path i32
///       required: true
/// auth: api_key
#[api_rt::route(patch, "/avs/{id}", Auth)]
async fn patch_id(
    user: web::ReqData<User>,
    data: web::Json<PartialUpdateAvs>,
    id: web::Path<i32>,
    db: web::Data<ApiDatabase>,
) -> actix_web::Result<AvsResponse> {
    let repo = db.repository::<AvsRepo>();
    if user.role_id == 1 || user.role_id == 2 {
        let res = repo
            .update_info(data.into_inner(), id.into_inner())
            .map_err(ApiError::from)?;
        AvsResponse::from(res).wrap()
    } else {
        Err(ApiError::new("Unauthorized").status(401).into())
    }
}

/// # Accept pending avs.
///
/// This endpoint responsible for accepting pending avs.
/// ****Rules:****
/// Only Root and SuperAdmin can access this endpoint.
/// ---
/// tags:
///     - avs
/// responses:
///     - status: 200
///       content: !T Message
///       description: The avs is updated.
///     - status: 400
///       content: !T ApiError
///       description: The request is invalid.
///     - status: 401
///       content: !T ApiError
///       description: The user is not authenticated.
///     - status: 404
///       content: !T ApiError
///       description: The avs is not found.
///     - status: 500
///       content: !T ApiError
///       description: Server encountered an error.
/// params:
///     - name: id
///       kind: !Path i32
///       required: true
/// auth: api_key
#[api_rt::route(patch, "/avs/accept/{id}", Auth)]
async fn patch_accept(
    user: web::ReqData<User>,
    id: web::Path<i32>,
    db: web::Data<ApiDatabase>,
    ws_state: web::Data<WsState>,
) -> actix_web::Result<Message> {
    if user.role_id == 1 || user.role_id == 2 {
        let repo = db.repository::<AvsRepo>();
        let avs = repo.accept(id.into_inner()).map_err(ApiError::from)?;
        if let Some(stream) = ws_state.avs_by_id(avs.unique_id).await {
            let _ = stream.write("authenticated", "").await;
        }
        Message::new("ok".to_owned()).wrap()
    } else {
        Err(ApiError::new("Unauthorized").status(401).into())
    }
}

#[api_rt::route(get, "/avs/slot/{id}")]
async fn get_slot(
    id: web::Path<i32>,
    db: web::Data<ApiDatabase>,
) -> actix_web::Result<impl Responder> {
    let repo = db.repository::<AvsRepo>();
    let slots: String = TimeSlots::new().into();
    repo.update_slots(id.into_inner(), &slots)
        .map_err(ApiError::from)?;
    Ok("")
}

/// # Delete avs.
///
/// This endpoint responsible for deleting an avs.
/// ---
/// tags:
///     - avs
/// responses:
///     - status: 200
///       content: !T Message
///       description: The avs is deleted.
///     - status: 401
///       content: !T ApiError
///       description: The user is not authenticated.
///     - status: 404
///       content: !T ApiError
///       description: The avs is not found.
///     - status: 500
///       content: !T ApiError
///       description: Server encountered an error.
/// params:
///     - name: id
///       kind: !Path i32
///       required: true
/// auth: api_key
#[api_rt::route(delete, "/avs/{id}", Auth)]
async fn delete(
    user: web::ReqData<User>,
    id: web::Path<i32>,
    db: web::Data<ApiDatabase>,
) -> actix_web::Result<Message> {
    if user.role_id != 1 {
        return Err(ApiError::new("Unauthorized").status(401).into());
    }
    let repo = db.repository::<AvsRepo>();
    repo.delete(id.into_inner()).map_err(ApiError::from)?;
    Message::new("ok".to_owned()).wrap()
}

api_rt::routes! {
    get
    patch_id
    patch_accept
    #[no_doc]
    get_slot
    delete
}
