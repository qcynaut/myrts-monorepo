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

use crate::{middlewares::auth::Auth, ApiError, Config};
use actix_web::web;
use api_db::{
    repos::{RecordsRepo, UserRepo},
    ApiDatabase,
};
use std::io::Read;
use types::api::{
    records::{NewRecords, RecordsResponse},
    user::User,
    RecordsReq, ValIntReq,
};
use utils::{audio::decode_audio, files::ApiAssets};

/// # Get all records.
///
/// This endpoint returns all records, If the user role is `1 (Root)` or `2 (SuperAdmin)` it will return all records.
/// Otherwise it will return only records created by the user.
/// ---
/// tags:
///     - records
/// responses:
///     - status: 200
///       content: !Vec RecordsResponse
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
#[api_rt::route(get, "/records", Auth)]
async fn get(
    user: web::ReqData<User>,
    db: web::Data<ApiDatabase>,
) -> actix_web::Result<impl actix_web::Responder> {
    let repo: RecordsRepo = db.repository();
    let user_repo: UserRepo = db.repository();
    let res = repo.get_all(user.id).map_err(ApiError::from)?;

    let mut response = vec![];

    for r in res.into_iter() {
        if let Some(user_id) = r.sender {
            let user = user_repo.get(user_id).map_err(ApiError::from)?;
            let mut record = RecordsResponse::from(r);
            record.sender = Some(user.into());
            response.push(record);
        } else {
            response.push(RecordsResponse::from(r));
        }
    }

    RecordsResponse::wrap_vec(response, Some(200))
}

/// # Create new record.
///
/// This endpoint creates a new record, user can either create record only or also share it with other users.
/// When the user_ids filled (send to another user), the record will be available on these user with status `0 (Pending)`.
///  ****Notes:****
/// Only user below the caller can receive these.
/// ---
/// tags:
///     - records
/// request:
///       content: !T RecordsReq
///       content_type: multipart/form-data
///       description: The record data.
/// responses:
///     - status: 201
///       content: !T RecordsResponse
///       content_type: application/json
///       description: Created record.
///     - status: 401
///       content: !T ApiError
///       content_type: application/json
///       description: Server encountered an error.
///     - status: 400
///       content: !T ApiError
///       content_type: application/json
///       description: Bad request.
///     - status: 500
///       content: !T ApiError
///       content_type: application/json
///       description: Server encountered an error.
/// auth: api_key
#[api_rt::route(post, "/records", Auth)]
async fn post(
    user: web::ReqData<User>,
    data: actix_multipart::form::MultipartForm<RecordsReq>,
    db: web::Data<ApiDatabase>,
    assets: web::Data<ApiAssets>,
    cfg: web::Data<Config>,
) -> actix_web::Result<RecordsResponse> {
    let data = data.into_inner();
    let repo: RecordsRepo = db.repository();

    if let Some(fname) = data.file.file_name {
        if !fname.ends_with(".mp3") {
            return Err(ApiError::new("Only mp3 files are allowed.")
                .status(400)
                .into());
        }
    } else {
        return Err(ApiError::new("File invalid").status(400).into());
    }

    let mut f = data.file.file.into_file();
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;

    // this was to make sure if the file is valid audio.
    let buf = decode_audio(buf);

    let hash = utils::crypto::hash(&buf);
    let file_url: String;
    let duration: u64;
    match repo.get_hash(&hash).map_err(ApiError::from)? {
        Some(x) => {
            file_url = x.file_url;
            duration = x.duration.parse().map_err(ApiError::new)?;
        }
        None => {
            let uuid = format!("{}.mp3", utils::crypto::uuid());
            assets.write_audio(&uuid, buf).map_err(ApiError::new)?;
            duration = assets.duration_audio(&uuid).map_err(ApiError::new)?;
            file_url = cfg.format(format!("/assets/audio/{}", uuid));
        }
    }

    let description = data.description.map(|d| d.clone());
    let name = data.name.clone();

    let res = repo
        .new(NewRecords {
            description: description.clone(),
            name: name.clone(),
            file_url: file_url.clone(),
            created_at: utils::time::now(),
            hash: hash.clone(),
            user_id: user.id,
            status: 1,
            duration: duration.to_string(),
            sender: None,
        })
        .map_err(ApiError::from)?;
    for other in data.user_ids {
        let _ = repo
            .new(NewRecords {
                description: description.clone(),
                name: name.clone(),
                file_url: file_url.clone(),
                created_at: utils::time::now(),
                hash: hash.clone(),
                user_id: *other,
                status: 0,
                duration: duration.to_string(),
                sender: Some(user.id),
            })
            .map_err(ApiError::from)?;
    }
    RecordsResponse::from(res).wrap()
}

/// # Update record status.
///
/// This endpoint updates the status of a record.
/// ---
/// tags:
///     - records
/// request:
///       content: !T ValIntReq
///       content_type: application/json
///       description: The request data.
/// responses:
///     - status: 200
///       content: !T RecordsResponse
///       content_type: application/json
///       description: Updated record.
///     - status: 400
///       content: !T ApiError
///       content_type: application/json
///       description: Bad request.
///     - status: 401
///       content: !T ApiError
///       content_type: application/json
///       description: Server encountered an error.
///     - status: 404
///       content: !T ApiError
///       content_type: application/json
///       description: Server encountered an error.
///     - status: 500
///       content: !T ApiError
///       content_type: application/json
///       description: Server encountered an error.
/// params:
///     - name: id
///       kind: !Path i32
///       required: true
///       description: The id of the record.
/// auth: api_key
#[api_rt::route(patch, "/records/status/{id}", Auth)]
async fn patch_status(
    user: web::ReqData<User>,
    id: web::Path<i32>,
    data: web::Json<ValIntReq>,
    db: web::Data<ApiDatabase>,
) -> actix_web::Result<RecordsResponse> {
    let repo: RecordsRepo = db.repository();
    let res = repo
        .update_status(id.into_inner(), data.val, user.id)
        .map_err(ApiError::from)?;
    RecordsResponse::from(res).wrap()
}

/// # Delete record.
///
/// This endpoint deletes a record.
/// ---
/// tags:
///     - records
/// responses:
///     - status: 200
///       content: !T RecordsResponse
///       content_type: application/json
///       description: Deleted record.
///     - status: 400
///       content: !T ApiError
///       content_type: application/json
///       description: Bad request.
///     - status: 401
///       content: !T ApiError
///       content_type: application/json
///       description: Unauthorized.
///     - status: 404
///       content: !T ApiError
///       content_type: application/json
///       description: Not found.
///     - status: 500
///       content: !T ApiError
///       content_type: application/json
///       description: Server encountered an error.
/// params:
///     - name: id
///       kind: !Path i32
///       required: true
///       description: The id of the record.
/// auth: api_key
#[api_rt::route(delete, "/records/{id}", Auth)]
async fn delete(
    user: web::ReqData<User>,
    id: web::Path<i32>,
    db: web::Data<ApiDatabase>,
) -> actix_web::Result<RecordsResponse> {
    let repo: RecordsRepo = db.repository();
    if user.id == 1 || user.id == 2 {
        let res = repo.delete(id.into_inner()).map_err(ApiError::from)?;
        RecordsResponse::from(res).wrap()
    } else {
        let id = id.into_inner();
        let current = repo.get(id).map_err(ApiError::from)?;
        if current.user_id != user.id {
            return Err(ApiError::new("You are not allowed to delete this record.")
                .status(401)
                .into());
        } else {
            let res = repo.delete(id).map_err(ApiError::from)?;
            RecordsResponse::from(res).wrap()
        }
    }
}

api_rt::routes! {
    get
    post
    patch_status
    delete
}
