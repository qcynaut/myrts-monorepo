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

use crate::ApiDatabase;
use database::{DatabaseRepository, Result};
use diesel::prelude::*;
use types::api::{
    forgot_password::{ForgotPassword, NewForgotPassword},
    schema::*,
    user::User,
};

/// ForgotPasswordRepo.
/// Repository for `forgot_password` table.
#[derive(Clone)]
pub struct ForgotPasswordRepo {
    db: ApiDatabase,
}

impl ForgotPasswordRepo {
    /// Create new forgot password.
    pub fn create(&self, user_id: i32) -> Result<ForgotPassword> {
        self.db.run(|conn| {
            let uuid = utils::crypto::uuid();
            let now = utils::time::now();
            let new = NewForgotPassword {
                uuid: uuid.clone(),
                user_id,
                created_at: now,
            };
            let res = diesel::insert_into(forgot_password::table)
                .values(&new)
                .get_result::<ForgotPassword>(conn)?;
            Ok(res)
        })
    }

    /// Check if the given uuid exist.
    pub fn exists(&self, uuid: &str) -> Result<bool> {
        self.db.run(|conn| {
            let res = forgot_password::table
                .filter(forgot_password::uuid.eq(uuid))
                .first::<ForgotPassword>(conn)
                .optional()?;
            if let Some(res) = res {
                Ok(utils::time::is_greater_than(
                    res.created_at,
                    utils::time::minute(5),
                ))
            } else {
                Ok(false)
            }
        })
    }

    /// Reset password.
    pub fn reset(&self, uuid: &str, new_password: &str) -> Result<()> {
        self.db.run_transaction(|conn| {
            let forgot = forgot_password::table
                .filter(forgot_password::uuid.eq(uuid))
                .first::<ForgotPassword>(conn)?;
            if !utils::time::is_greater_than(forgot.created_at, utils::time::minute(5)) {
                return Err(database::DatabaseError::Expired);
            }
            let user = users::table
                .filter(users::id.eq(forgot.user_id))
                .first::<User>(conn)?;
            let _ = diesel::delete(web_session::table)
                .filter(web_session::user_id.eq(user.id))
                .execute(conn);
            let _ = diesel::delete(web_session_pending::table)
                .filter(web_session_pending::user_id.eq(user.id))
                .execute(conn);
            let _ = diesel::delete(mobile_session::table)
                .filter(mobile_session::user_id.eq(user.id))
                .execute(conn);
            let _ = diesel::delete(mobile_session_pending::table)
                .filter(mobile_session_pending::user_id.eq(user.id))
                .execute(conn);
            let _ = diesel::delete(forgot_password::table)
                .filter(forgot_password::uuid.eq(uuid))
                .execute(conn);
            let _ = diesel::update(users::table)
                .filter(users::id.eq(user.id))
                .set(users::password.eq(new_password))
                .execute(conn);
            Ok(())
        })
    }
}

impl DatabaseRepository<PgConnection> for ForgotPasswordRepo {
    fn new(db: database::Database<PgConnection>) -> Self {
        Self { db }
    }
}
