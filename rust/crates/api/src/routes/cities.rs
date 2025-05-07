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
use api_db::{repos::CityRepo, ApiDatabase};
use types::api::{
    city::{CityResponse, NewCity, PaginatedCityResponse},
    user::User,
    PageQuery,
};

/// # Get all cities.
///
/// Get all available registered cities.
/// ---
/// tags:
///     - cities
/// responses:
///     - status: 200
///       content: !Vec CityResponse
///       description: The city data.
///     - status: 401
///       content: !T ApiError
///       description: The user is not authenticated.
///     - status: 500
///       content: !T ApiError
///       description: Server encountered an error.
/// auth: api_key
#[api_rt::route(get, "/cities", Auth)]
async fn get(db: web::Data<ApiDatabase>) -> actix_web::Result<impl actix_web::Responder> {
    let repo = db.repository::<CityRepo>();
    let cities = repo.get_all().map_err(ApiError::from)?;
    CityResponse::wrap_vec(cities.into_iter().map(CityResponse::from).collect(), None)
}

/// # Get all cities with pagination.
///
/// Get all available registered cities.
/// ---
/// tags:
///     - cities
/// responses:
///     - status: 200
///       content: !T PaginatedCityResponse
///       description: The city data.
///     - status: 401
///       content: !T ApiError
///       description: The user is not authenticated.
///     - status: 500
///       content: !T ApiError
///       description: Server encountered an error.
/// params:
///     - name: page
///       kind: !Query i64
///       required: false
///       description: The page number, if not provided, it will default to 1.
/// auth: api_key
#[api_rt::route(get, "/cities/paginated", Auth)]
async fn get_paginated(
    page: web::Query<PageQuery>,
    db: web::Data<ApiDatabase>,
    cfg: web::Data<Config>,
) -> actix_web::Result<PaginatedCityResponse> {
    let repo = db.repository::<CityRepo>();
    let page = page.into_inner().page.unwrap_or(1);
    let (cities, total, total_pages) = repo.get_all_paginated(page).map_err(ApiError::from)?;
    let prev = if page > 1 {
        Some(cfg.format(format!("/cities?page={}", page - 1)))
    } else {
        None
    };
    let next = if total > page * 10 {
        Some(cfg.format(format!("/cities?page={}", page + 1)))
    } else {
        None
    };
    PaginatedCityResponse::new(
        cities.into_iter().map(CityResponse::from).collect(),
        total,
        page,
        total_pages,
        next,
        prev,
    )
    .wrap()
}

/// # Get city by province id.
///
/// Get city by province id.
/// ---
/// tags:
///     - cities
/// responses:
///     - status: 200
///       content: !T CityResponse
///       description: The city data.
///     - status: 400
///       content: !T ApiError
///       description: The request is invalid.
///     - status: 401
///       content: !T ApiError
///       description: The user is not authenticated.
///     - status: 500
///       content: !T ApiError
///       description: Server encountered an error.
/// params:
///     - name: province_id
///       kind: !Path i32
///       required: true
///       description: The province id.
/// auth: api_key
#[api_rt::route(get, "/cities/{province_id}", Auth)]
async fn get_by_province_id(
    province_id: web::Path<i32>,
    db: web::Data<ApiDatabase>,
) -> actix_web::Result<impl actix_web::Responder> {
    let repo = db.repository::<CityRepo>();
    let cities = repo
        .get_by_province_id(*province_id)
        .map_err(ApiError::from)?;
    CityResponse::wrap_vec(cities.into_iter().map(CityResponse::from).collect(), None)
}

/// # Create a new city.
///
/// This endpoint is responsible for creating a new city.
/// ****Note:****
/// This endpoint should not be used because every city in Indonesia has been already registered.
/// ---
/// tags:
///     - cities
/// request:
///     content: !T NewCity
///     content_type: application/json
///     description: The city data.
/// responses:
///     - status: 201
///       content: !T CityResponse
///       description: The province data.
///     - status: 400
///       content: !T ApiError
///       description: Bad request.
///     - status: 401
///       content: !T ApiError
///       description: The user is not authenticated.
///     - status: 500
///       content: !T ApiError
///       description: Server encountered an error.
/// auth: api_key
#[api_rt::route(post, "/cities", Auth)]
async fn post(
    user: web::ReqData<User>,
    data: web::Json<NewCity>,
    db: web::Data<ApiDatabase>,
) -> actix_web::Result<CityResponse> {
    let repo = db.repository::<CityRepo>();
    if user.role_id == 3 {
        return Err(ApiError::new("Unauthorized").status(401).into());
    }
    let res = repo.create(data.into_inner()).map_err(ApiError::from)?;
    CityResponse::from(res).wrap()
}

api_rt::routes! {
    get
    get_paginated
    get_by_province_id
    post
}
