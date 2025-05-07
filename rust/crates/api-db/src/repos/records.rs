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
    records::{NewRecords, Records},
    schema::*,
    user::User,
};

/// RecordsRepo.
/// Repository for `records` table.
#[derive(Clone)]
pub struct RecordsRepo {
    db: ApiDatabase,
}

impl RecordsRepo {
    /// Create a new Records.
    pub fn new(&self, new: NewRecords) -> Result<Records> {
        self.db.run(|conn| {
            let res = diesel::insert_into(records::table)
                .values(&new)
                .get_result(conn)?;
            Ok(res)
        })
    }

    /// Get all records.
    pub fn get_all(&self, user_id: i32) -> Result<Vec<Records>> {
        self.db.run(|conn| {
            let user = users::table
                .filter(users::id.eq(user_id))
                .first::<User>(conn)?;
            if user.role_id == 1 || user.role_id == 2 {
                let res = records::table.load::<Records>(conn)?;
                Ok(res)
            } else {
                let res = records::table
                    .filter(records::user_id.eq(user_id))
                    .load::<Records>(conn)?;
                Ok(res)
            }
        })
    }

    /// Get record by id.
    pub fn get(&self, id: i32) -> Result<Records> {
        self.db.run(|conn| {
            let res = records::table
                .filter(records::id.eq(id))
                .first::<Records>(conn)?;
            Ok(res)
        })
    }

    /// Get by hash.
    pub fn get_hash(&self, hash: &str) -> Result<Option<Records>> {
        self.db.run(|conn| {
            let res = records::table
                .filter(records::hash.eq(hash))
                .first::<Records>(conn)
                .optional()?;
            Ok(res)
        })
    }

    /// Update record status by id.
    pub fn update_status(&self, id: i32, status: i32, user_id: i32) -> Result<Records> {
        self.db.run(|conn| {
            let user = users::table
                .filter(users::id.eq(user_id))
                .first::<User>(conn)?;
            if user.role_id == 1 || user.role_id == 2 {
                diesel::update(records::table)
                    .filter(records::id.eq(id))
                    .set(records::status.eq(status))
                    .get_result(conn)
                    .map_err(Into::into)
            } else {
                diesel::update(records::table)
                    .filter(records::user_id.eq(user_id))
                    .filter(records::id.eq(id))
                    .set(records::status.eq(status))
                    .get_result(conn)
                    .map_err(Into::into)
            }
        })
    }

    /// Delete record.
    pub fn delete(&self, id: i32) -> Result<Records> {
        self.db.run(|conn| {
            diesel::delete(records::table.filter(records::id.eq(id)))
                .get_result::<Records>(conn)
                .map_err(Into::into)
        })
    }
}

impl DatabaseRepository<PgConnection> for RecordsRepo {
    fn new(db: database::Database<PgConnection>) -> Self {
        Self { db }
    }
}
