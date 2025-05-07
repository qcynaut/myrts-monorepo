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
use api_db::{repos::UserGroupRepo, ApiDatabase};
use types::api::{
    user::User,
    user_group::{NewUserGroup, UpdateUserGroup, UserGroupResponse},
    PageQuery, PaginatedGroupData,
};

use crate::{middlewares::auth::Auth, ApiError, Config};

/// # Get all groups.
///
/// Get all available registered groups.
/// ---
/// tags:
///     - groups
/// responses:
///     - status: 200
///       content: !Vec UserGroupResponse
///       description: The group data.
///     - status: 401
///       content: !T ApiError
///       description: The user is not authenticated.
///     - status: 500
///       content: !T ApiError
///       description: Server encountered an error.
/// auth: api_key
#[api_rt::route(get, "/groups", Auth)]
async fn get(
    user: web::ReqData<User>,
    db: web::Data<ApiDatabase>,
) -> actix_web::Result<impl actix_web::Responder> {
    let repo = db.repository::<UserGroupRepo>();
    let groups = repo.get_all(user.id).map_err(ApiError::from)?;
    UserGroupResponse::wrap_vec(
        groups.into_iter().map(UserGroupResponse::from).collect(),
        None,
    )
}

/// # Get all groups with it's related data.
///
/// Get all available registered groups along with it's related data.
/// for user with role `3 (Admin)` it will only show groups that below the user.
/// ---
/// tags:
///     - groups
/// responses:
///     - status: 200
///       content: !T PaginatedGroupData
///       description: The group data.
///     - status: 400
///       content: !T ApiError
///       description: Bad request.
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
#[api_rt::route(get, "/groups/paginated", Auth)]
async fn get_paginated(
    user: web::ReqData<User>,
    db: web::Data<ApiDatabase>,
    query: web::Query<PageQuery>,
    cfg: web::Data<Config>,
) -> actix_web::Result<PaginatedGroupData> {
    let repo = db.repository::<UserGroupRepo>();
    let page = query.page.unwrap_or(1);
    let (groups, total, total_pages) = repo
        .get_all_related_paginated(user.id, page)
        .map_err(ApiError::from)?;
    let prev = if page > 1 {
        Some(cfg.format(format!("/groups/paginated?page={}", page - 1)))
    } else {
        None
    };
    let next = if total > page * 10 {
        Some(cfg.format(format!("/groups/paginated?page={}", page + 1)))
    } else {
        None
    };
    PaginatedGroupData::new(groups, total, page, total_pages, next, prev).wrap()
}

/// # Create a new group.
///
/// Create a new group, Only user with role `1 (Root)` and `2 (SuperAdmin)` can create a new group.
/// ---
/// tags:
///     - groups
/// request:
///     content: !T NewUserGroup
///     content_type: application/json
///     description: The group data.
/// responses:
///     - status: 201
///       content: !T UserGroupResponse
///       description: The group data.
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
#[api_rt::route(post, "/groups", Auth)]
async fn post(
    user: web::ReqData<User>,
    data: web::Json<NewUserGroup>,
    db: web::Data<ApiDatabase>,
) -> actix_web::Result<UserGroupResponse> {
    if user.role_id == 3 {
        return Err(ApiError::new("Unauthorized").status(401).into());
    }

    let repo = db.repository::<UserGroupRepo>();
    let res = repo.create(data.into_inner()).map_err(ApiError::from)?;
    UserGroupResponse::from(res).wrap()
}

/// # Update a group.
///
/// Update a group, Only user with role `1 (Root)` and `2 (SuperAdmin)` can update a group.
/// ---
/// tags:
///     - groups
/// request:
///     content: !T UpdateUserGroup
///     content_type: application/json
///     description: The group data.
/// responses:
///     - status: 200
///       content: !T UserGroupResponse
///       description: The group data.
///     - status: 400
///       content: !T ApiError
///       description: Bad request.
///     - status: 401
///       content: !T ApiError
///       description: The user is not authenticated.
///     - status: 404
///       content: !T ApiError
///       description: The group is not found.
///     - status: 500
///       content: !T ApiError
///       description: Server encountered an error.
/// auth: api_key
#[api_rt::route(patch, "/groups", Auth)]
async fn patch(
    user: web::ReqData<User>,
    data: web::Json<UpdateUserGroup>,
    db: web::Data<ApiDatabase>,
) -> actix_web::Result<UserGroupResponse> {
    if user.role_id != 1 && user.role_id != 2 {
        return Err(ApiError::new("Unauthorized").status(401).into());
    }

    let repo = db.repository::<UserGroupRepo>();
    let res = repo.update(data.into_inner()).map_err(ApiError::from)?;
    UserGroupResponse::from(res).wrap()
}

api_rt::routes! {
    get
    get_paginated
    post
    patch
}
