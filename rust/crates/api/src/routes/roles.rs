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
use actix_web::web;
use api_db::{repos::RoleRepo, ApiDatabase};
use types::api::role::RoleResponse;

/// # Get all roles.
///
/// This endpoint is responsible for getting all available roles.
/// ---
/// tags:
///     - roles
/// repsonses:
///     - status: 200
///       content: !T RoleResponse
///       content_type: application/json
///       description: List of available roles.
///     - status: 401
///       content: !T ApiError
///       content_type: application/json
///       description: Server encountered an error.
///     - status: 500
///       content: !T ApiError
///       content_type: application/json
///       description: Server encountered an error.
/// auth: api_key
#[api_rt::route(get, "/roles", Auth)]
async fn get(db: web::Data<ApiDatabase>) -> actix_web::Result<impl actix_web::Responder> {
    let repo: RoleRepo = db.repository();
    let res = repo.all().map_err(ApiError::from)?;
    RoleResponse::wrap_vec(res.into_iter().map(RoleResponse::from).collect(), Some(200))
}

api_rt::routes! {
    get
}
