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

use std::io::Read;

use crate::{middlewares::auth::Auth, ApiError, Config};
use actix_web::web;
use api_db::{
    repos::{PackageRepo, UserRepo},
    ApiDatabase,
};
use types::api::{
    package::PackageResponse,
    province::ProvinceResponse,
    role::RoleResponse,
    user::{NewUser, UpdateUser, User, UserResponse},
    user_group::UserGroupResponse,
    CityData, FileReq, SubscriptionData, UserData, UserReq,
};
use utils::{crypto::Bcrypt, files::ApiAssets};

/// # Get current user.
///
/// This endpoint responsible for getting current user. Mainly used after login.
/// ---
/// tags:
///     - users
/// responses:
///     - status: 200
///       content: !T UserData
///       description: Current user.
///     - status: 401
///       content: !T ApiError
///       description: Unauthorized.
///     - status: 500
///       content: !T ApiError
///       description: Server encountered an error.
/// auth: api_key
#[api_rt::route(get, "/users/current", Auth)]
async fn get_current(
    user: web::ReqData<User>,
    db: web::Data<ApiDatabase>,
) -> actix_web::Result<UserData> {
    let repo = db.repository::<UserRepo>();
    let (user, city, group, role, subscription) = repo.get_data(user.id).map_err(ApiError::from)?;
    UserData::new(
        user.id,
        user.name,
        user.email,
        user.image_url,
        RoleResponse::from(role),
        city.map(|(c, p)| CityData::new(c.id, c.name, ProvinceResponse::from(p))),
        group.into_iter().map(UserGroupResponse::from).collect(),
        subscription.map(|(s, p)| {
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
    )
    .wrap()
}

/// # Create a new user.
///
/// Only Root and SuperAdmin can create user.
///
/// ****Rules:****
/// - Root can create SuperAdmin and Admin
/// - SuperAdmin can create Admin
/// - Admin can't use this endpoint
/// ---
/// tags:
///     - users
/// request:
///     content: !T UserReq
///     content_type: application/json
///     description: The user data.
/// responses:
///     - status: 201
///       content: !T UserResponse
///       description: Created user.
///     - status: 400
///       content: !T ApiError
///       description: The request is invalid.
///     - status: 401
///       content: !T ApiError
///       description: Unauthorized.
///     - status: 500
///       content: !T ApiError
///       description: Server encountered an error.
/// auth: api_key
#[api_rt::route(post, "/users", Auth)]
async fn post(
    user: web::ReqData<User>,
    data: web::Json<UserReq>,
    db: web::Data<ApiDatabase>,
) -> actix_web::Result<UserResponse> {
    let data = data.into_inner();
    let repo = db.repository::<UserRepo>();
    let package_repo = db.repository::<PackageRepo>();
    if user.role_id == 3 {
        return Err(ApiError::new("Only super admin can create user")
            .status(401)
            .into());
    }

    if user.role_id == 2 && (data.role == 1 || data.role == 2) {
        return Err(ApiError::new("Admin can't create other admin or Root")
            .status(401)
            .into());
    }

    if user.role_id == 1 && data.role == 1 {
        return Err(ApiError::new("Root can't create other root")
            .status(401)
            .into());
    }

    if (data.role == 3) && data.package_id.is_none() {
        return Err(ApiError::new("Package is required").status(400).into());
    }
    let mut package_id = 0;
    if let Some(pid) = &data.package_id {
        let package = package_repo.get_by_id(*pid).map_err(ApiError::from)?;
        if data.devices.len() > package.max_devices as usize {
            return Err(ApiError::new("Too many devices").status(400).into());
        }
        package_id = package.id;
    }
    let password = Bcrypt::hash(&utils::crypto::random_string(12)).map_err(ApiError::new)?;
    let res = repo
        .create(
            NewUser {
                name: data.name.clone(),
                email: data.email.clone(),
                password,
                image_url: None,
                role_id: data.role,
                user_group_ids: data.group_ids.into_iter().map(|id| Some(id)).collect(),
                device_ids: data.devices.into_iter().map(|d| Some(d)).collect(),
                city_id: data.city_id.map(|id| id),
            },
            package_id,
        )
        .map_err(ApiError::from)?;
    UserResponse::from(res).wrap()
}

/// # Update user.
///
/// Only Root and SuperAdmin can update user.
///
/// ****Rules:****
/// - Root can update SuperAdmin and Admin
/// - SuperAdmin can update Admin
/// - Admin can't use this endpoint
/// ---
/// tags:
///     - users
/// request:
///     content: !T UpdateUser
///     content_type: application/json
///     description: The user data.
/// responses:
///     - status: 201
///       content: !T UserResponse
///       description: Created user.
///     - status: 400
///       content: !T ApiError
///       description: The request is invalid.
///     - status: 401
///       content: !T ApiError
///       description: Unauthorized.
///     - status: 500
///       content: !T ApiError
///       description: Server encountered an error.
/// auth: api_key
#[api_rt::route(patch, "/users", Auth)]
async fn patch(
    user: web::ReqData<User>,
    data: web::Json<UpdateUser>,
    db: web::Data<ApiDatabase>,
) -> actix_web::Result<UserResponse> {
    let data = data.into_inner();
    let repo = db.repository::<UserRepo>();
    if user.role_id == 3 {
        return Err(ApiError::new("Only super admin can update user")
            .status(401)
            .into());
    }

    let target = repo.get(data.id).map_err(ApiError::from)?;
    if user.role_id == 2 && (target.role_id == 1 || data.role_id == Some(1)) {
        return Err(ApiError::new("SuperAdmin can't be upgraded into root")
            .status(401)
            .into());
    }

    let res = repo.update(data).map_err(ApiError::from)?;
    UserResponse::from(res).wrap()
}

/// # Update user profile picture.
///
/// ---
/// tags:
///     - users
/// request:
///     content: !T FileReq
///     content_type: multipart/form-data
///     description: The user profile picture.
/// responses:
///     - status: 200
///       content: !T UserResponse
///       description: Created user.
///     - status: 400
///       content: !T ApiError
///       description: The request is invalid.
///     - status: 401
///       content: !T ApiError
///       description: Unauthorized.
///     - status: 500
///       content: !T ApiError
///       description: Server encountered an error.
/// auth: api_key
#[api_rt::route(patch, "/users/image", Auth)]
async fn patch_image(
    user: web::ReqData<User>,
    data: actix_multipart::form::MultipartForm<FileReq>,
    db: web::Data<ApiDatabase>,
    assets: web::Data<ApiAssets>,
    cfg: web::Data<Config>,
) -> actix_web::Result<UserResponse> {
    let repo = db.repository::<UserRepo>();
    let mut data = data.into_inner();
    let f = data.file.file.as_file_mut();
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).map_err(|e| ApiError::new(e))?;

    let uuid = utils::crypto::uuid();
    let filename = format!("{}.png", uuid);

    assets.write_image(&filename, &buf).map_err(ApiError::new)?;
    let image_url = cfg.format(format!("/assets/image/{}", filename));
    let res = repo
        .update_profile(user.id, &image_url)
        .map_err(ApiError::from)?;

    UserResponse::from(res).wrap()
}

/// # Delete a user.
///
/// Only Root and SuperAdmin can delete a user.
///
/// ****Rules:****
/// - Root can delete any user.
/// - SuperAdmin can delete any user except Root.
/// - Admin can't use this endpoint
/// ---
/// tags:
///     - users
/// responses:
///     - status: 200
///       content: !T UserResponse
///       description: Deleted user.
///     - status: 401
///       content: !T ApiError
///       description: Unauthorized.
///     - status: 404
///       content: !T ApiError
///       description: User not found.
///     - status: 500
///       content: !T ApiError
///       description: Server encountered an error.
/// params:
///     - name: id
///       kind: !Path i32
///       required: true
///       description: The user id.
/// auth: api_key
#[api_rt::route(delete, "/users/{id}", Auth)]
async fn delete(
    user: web::ReqData<User>,
    id: web::Path<i32>,
    db: web::Data<ApiDatabase>,
) -> actix_web::Result<UserResponse> {
    let repo = db.repository::<UserRepo>();
    let id = id.into_inner();
    let target = repo.get(id).map_err(ApiError::from)?;
    if (user.role_id != 3) && target.role_id == 3 {
        let res = repo.delete(id).map_err(ApiError::from)?;
        UserResponse::from(res).wrap()
    } else if user.role_id == 1 && target.role_id == 1
        || user.role_id == 2 && target.role_id == 1
        || user.role_id == 2 && target.role_id == 2
    {
        Err(ApiError::new("Only super admin can delete user")
            .status(401)
            .into())
    } else {
        let res = repo.delete(id).map_err(ApiError::from)?;
        UserResponse::from(res).wrap()
    }
}

api_rt::routes! {
    get_current
    post
    patch
    patch_image
    delete
}
