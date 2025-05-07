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
use super::user::{User, UserResponse};
#[cfg(feature = "db")]
use diesel::prelude::*;
use types_rt::ty;

/// Records.
/// The records data.
#[derive(Clone)]
#[ty(db(kind: Query, table: records, relations: [User]))]
pub struct Records {
    pub id: i32,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub description: Option<String>,
    pub file_url: String,
    pub hash: String,
    pub user_id: i32,
    pub status: i32,
    pub duration: String,
    pub sender: Option<i32>,
}

/// RecordsResponse.
/// The records data.
#[ty(web(Response(pagination: true)))]
pub struct RecordsResponse {
    pub id: i32,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub description: Option<String>,
    pub file_url: String,
    pub hash: String,
    pub user_id: i32,
    pub status: i32,
    pub duration: String,
    pub sender: Option<UserResponse>,
}

#[cfg(feature = "web")]
impl From<Records> for RecordsResponse {
    fn from(r: Records) -> Self {
        Self {
            __status: 200,
            id: r.id,
            name: r.name,
            created_at: r.created_at,
            description: r.description,
            file_url: r.file_url,
            hash: r.hash,
            user_id: r.user_id,
            status: r.status,
            duration: r.duration,
            sender: None,
        }
    }
}

/// Create new Records.
#[ty(db(kind: Insert, table: records))]
pub struct NewRecords {
    pub name: String,
    pub description: Option<String>,
    pub file_url: String,
    pub created_at: chrono::NaiveDateTime,
    pub hash: String,
    pub user_id: i32,
    pub status: i32,
    pub duration: String,
    pub sender: Option<i32>,
}

api_rt::schemas! {
    RecordsResponse
    PaginatedRecordsResponse
}
