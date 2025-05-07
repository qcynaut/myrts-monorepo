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
use api_db::{repos::PackageRepo, ApiDatabase};
use types::api::package::PackageResponse;

/// # Get all packages.
///
/// Get all available registered packages.
/// ---
/// tags:
///     - packages
/// responses:
///     - status: 200
///       content: !Vec PackageResponse
///       description: The package data.
///     - status: 401
///       content: !T ApiError
///       description: The user is not authenticated.
///     - status: 500
///       content: !T ApiError
///       description: Server encountered an error.
/// auth: api_key
#[api_rt::route(get, "/packages", Auth)]
async fn get(db: web::Data<ApiDatabase>) -> actix_web::Result<impl actix_web::Responder> {
    let repo = db.repository::<PackageRepo>();
    let packages = repo.get_all().map_err(ApiError::from)?;
    PackageResponse::wrap_vec(
        packages.into_iter().map(PackageResponse::from).collect(),
        None,
    )
}

/// # Get package by id.
///
/// Get package by id.
/// ---
/// tags:
///     - packages
/// responses:
///     - status: 200
///       content: !T PackageResponse
///       description: The package data.
///     - status: 400
///       content: !T ApiError
///       description: The request is invalid.
///     - status: 401
///       content: !T ApiError
///       description: The user is not authenticated.
///     - status: 400
///       content: !T ApiError
///       description: The package does not exist.
///     - status: 500
///       content: !T ApiError
///       description: Server encountered an error.
/// params:
///     - name: id
///       kind: !Path i32
///       required: true
///       description: The package id.
/// auth: api_key
#[api_rt::route(get, "/packages/{id}", Auth)]
async fn get_by_id(
    id: web::Path<i32>,
    db: web::Data<ApiDatabase>,
) -> actix_web::Result<PackageResponse> {
    let repo = db.repository::<PackageRepo>();
    let package = repo.get_by_id(*id).map_err(ApiError::from)?;
    PackageResponse::from(package).wrap()
}

api_rt::routes! {
    get
    get_by_id
}
