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

#[cfg(feature = "db")]
use super::schema::*;
use super::{city::City, role::Role};
#[cfg(feature = "db")]
use diesel::prelude::*;
use types_rt::ty;

/// User.
/// The users data.
#[derive(Clone)]
#[ty(db(kind: Query, table: users, relations: [City, Role]))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub image_url: Option<String>,
    pub role_id: i32,
    pub device_ids: Vec<Option<i32>>,
    pub city_id: Option<i32>,
    pub user_group_ids: Vec<Option<i32>>,
}

#[ty(web(Response))]
pub struct UserResponse {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub image_url: Option<String>,
    pub role_id: i32,
    pub device_ids: Vec<Option<i32>>,
    pub city_id: Option<i32>,
    pub user_group_ids: Vec<Option<i32>>,
}

#[cfg(feature = "web")]
impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            __status: 200,
            id: user.id,
            name: user.name,
            email: user.email,
            image_url: user.image_url,
            role_id: user.role_id,
            device_ids: user.device_ids,
            city_id: user.city_id,
            user_group_ids: user.user_group_ids,
        }
    }
}

/// Create new User.
#[ty(db(kind: Insert, table: users))]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
    pub image_url: Option<String>,
    pub role_id: i32,
    pub device_ids: Vec<Option<i32>>,
    pub city_id: Option<i32>,
    pub user_group_ids: Vec<Option<i32>>,
}

/// Update User.
#[ty(db(kind: Update, table: users), web(Request))]
pub struct UpdateUser {
    pub id: i32,
    pub name: Option<String>,
    pub email: Option<String>,
    pub role_id: Option<i32>,
    pub device_ids: Vec<Option<i32>>,
    pub city_id: Option<i32>,
    pub user_group_ids: Vec<Option<i32>>,
}

api_rt::schemas! {
    UserResponse
    UpdateUser
}
