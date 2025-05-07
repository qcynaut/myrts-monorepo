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
use api_db::{repos::ProvinceRepo, ApiDatabase};
use types::api::{
    city::CityResponse,
    province::{NewProvince, ProvinceResponse},
    ProvinceData,
};

use crate::{middlewares::auth::Auth, ApiError};

/// # Get all provinces.
/// ---
/// tags:
///     - provinces
/// responses:
///     - status: 200
///       content: !Vec ProvinceResponse
///       description: All provinces.
///     - status: 401
///       content: !T ApiError
///       description: Unauthorized.
///     - status: 500
///       content: !T ApiError
///       description: Server encountered an error.
/// auth: api_key
#[api_rt::route(get, "/provinces", Auth)]
async fn get(db: web::Data<ApiDatabase>) -> actix_web::Result<impl actix_web::Responder> {
    let repo = db.repository::<ProvinceRepo>();
    let provinces = repo.all().map_err(ApiError::from)?;
    ProvinceResponse::wrap_vec(
        provinces.into_iter().map(ProvinceResponse::from).collect(),
        None,
    )
}

/// # Get all provinces related.
///
/// This endpoint is responsible for getting all provinces along with their cities.
/// ---
/// tags:
///     - provinces
/// responses:
///     - status: 200
///       content: !Vec ProvinceData
///       description: All provinces along with their cities.
///     - status: 401
///       content: !T ApiError
///       description: Unauthorized.
///     - status: 500
///       content: !T ApiError
///       description: Server encountered an error.
/// auth: api_key
#[api_rt::route(get, "/provinces/related", Auth)]
async fn get_related(db: web::Data<ApiDatabase>) -> actix_web::Result<impl actix_web::Responder> {
    let repo = db.repository::<ProvinceRepo>();
    let provinces = repo.get_all_related().map_err(ApiError::from)?;
    ProvinceData::wrap_vec(
        provinces
            .into_iter()
            .map(|(p, c)| {
                ProvinceData::new(
                    p.id,
                    p.name,
                    c.into_iter().map(CityResponse::from).collect(),
                )
            })
            .collect(),
        None,
    )
}

/// # Create new province.
///
/// This endpoint is responsible for creating a new province.
/// ****Note:****
/// This endpoint should not be used because every province in Indonesia has been already registered.
/// ---
/// tags:
///     - provinces
/// request:
///     content: !T NewProvince
///     content_type: application/json
///     description: The province data.
/// responses:
///     - status: 201
///       content: !T ProvinceResponse
///       description: The province data.
///     - status: 400
///       content: !T ApiError
///       description: Bad request.
///     - status: 401
///       content: !T ApiError
///       description: Unauthorized.
///     - status: 500
///       content: !T ApiError
///       description: Server encountered an error.
/// auth: api_key
#[api_rt::route(post, "/provinces", Auth)]
async fn post(
    db: web::Data<ApiDatabase>,
    data: web::Json<NewProvince>,
) -> actix_web::Result<ProvinceResponse> {
    let data = data.into_inner();
    let repo = db.repository::<ProvinceRepo>();
    let province = repo.create(data).map_err(ApiError::from)?;
    ProvinceResponse::from(province).status(201).wrap()
}

api_rt::routes! {
    get
    get_related
    post
}
