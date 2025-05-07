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
    schema::*,
    session::{NewMobileSession, NewWebSession, PendingMobileSession, PendingWebSession},
    verify::{NewVerify, Verify},
};

/// VerifyRepo.
/// Repository for `verify` table.
#[derive(Clone)]
pub struct VerifyRepo {
    db: ApiDatabase,
}

impl VerifyRepo {
    /// Create new verify.
    pub fn create(&self, mobile: bool, session_id: i32) -> Result<String> {
        self.db.run(|conn| {
            let uuid = utils::crypto::uuid();
            let new = NewVerify {
                uuid: uuid.clone(),
                created_at: utils::time::now(),
                mobile: mobile as i32,
                session_id,
            };
            diesel::insert_into(verify::table)
                .values(&new)
                .execute(conn)?;
            Ok(uuid)
        })
    }

    /// Verify pending session.
    pub fn verify(&self, uuid: &str) -> Result<(String, bool)> {
        log::debug!("uuid: {}", uuid);
        self.db.run_transaction(|conn| {
            let ver = verify::table
                .filter(verify::uuid.eq(uuid))
                .first::<Verify>(conn)?;
            log::debug!("0");
            if !utils::time::is_greater_than(ver.created_at, utils::time::minute(5)) {
                return Err(database::DatabaseError::Expired);
            }
            if ver.mobile != 0 {
                let pending = diesel::delete(mobile_session_pending::table)
                    .filter(mobile_session_pending::id.eq(ver.session_id))
                    .get_result::<PendingMobileSession>(conn)?;
                diesel::delete(mobile_session::table)
                    .filter(mobile_session::user_id.eq(pending.user_id))
                    .execute(conn)?;
                diesel::insert_into(mobile_session::table)
                    .values(NewMobileSession {
                        user_id: pending.user_id,
                        token: pending.token.clone(),
                    })
                    .execute(conn)?;
                Ok((pending.token, true))
            } else {
                log::debug!("1");
                let pending = diesel::delete(web_session_pending::table)
                    .filter(web_session_pending::id.eq(ver.session_id))
                    .get_result::<PendingWebSession>(conn)?;
                log::debug!("2");
                diesel::delete(web_session::table)
                    .filter(web_session::user_id.eq(pending.user_id))
                    .execute(conn)?;
                log::debug!("3");
                diesel::insert_into(web_session::table)
                    .values(NewWebSession {
                        user_id: pending.user_id,
                        token: pending.token.clone(),
                    })
                    .execute(conn)?;
                Ok((pending.token, false))
            }
        })
    }
}

impl DatabaseRepository<PgConnection> for VerifyRepo {
    fn new(db: database::Database<PgConnection>) -> Self {
        Self { db }
    }
}
