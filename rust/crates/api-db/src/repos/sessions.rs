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
    blacklist_token::NewBlacklistToken,
    schema::*,
    session::{
        MobileSession, NewMobileSession, NewPendingMobileSession, NewPendingWebSession,
        NewWebSession, PendingMobileSession, PendingWebSession, WebSession,
    },
};

/// SessionsRepo.
/// Repository for `sessions` table.
#[derive(Clone)]
pub struct SessionsRepo {
    db: ApiDatabase,
}

impl SessionsRepo {
    /// Check if the token is valid.
    pub fn is_valid(&self, token: &str) -> Result<bool> {
        self.db.run(|conn| {
            let web = web_session::table
                .filter(web_session::token.eq(token))
                .first::<WebSession>(conn)
                .optional()?;
            if web.is_some() {
                return Ok(true);
            }
            let mobile = mobile_session::table
                .filter(mobile_session::token.eq(token))
                .first::<MobileSession>(conn)
                .optional()?;
            Ok(mobile.is_some())
        })
    }

    /// Check if session need to be pending.
    pub fn is_need_pending(&self, user_id: i32, mobile: bool) -> Result<bool> {
        self.db.run(|conn| {
            if mobile {
                let res = mobile_session::table
                    .filter(mobile_session::user_id.eq(user_id))
                    .first::<MobileSession>(conn)
                    .optional()?;
                Ok(res.is_some())
            } else {
                let res = web_session::table
                    .filter(web_session::user_id.eq(user_id))
                    .first::<WebSession>(conn)
                    .optional()?;
                Ok(res.is_some())
            }
        })
    }

    /// Create pending session.
    pub fn create_pending(&self, user_id: i32, mobile: bool, token: &str) -> Result<i32> {
        self.db.run_transaction(|conn| {
            if mobile {
                let current_pending = mobile_session_pending::table
                    .filter(mobile_session_pending::user_id.eq(user_id))
                    .first::<PendingMobileSession>(conn)
                    .optional()?;
                if let Some(pending) = current_pending {
                    diesel::delete(
                        mobile_session_pending::table
                            .filter(mobile_session_pending::id.eq(pending.id)),
                    )
                    .execute(conn)?;
                }
                let res = diesel::insert_into(mobile_session_pending::table)
                    .values(NewPendingMobileSession {
                        user_id,
                        token: token.to_string(),
                    })
                    .get_result::<PendingMobileSession>(conn)?;
                Ok(res.id)
            } else {
                let current_pending = web_session_pending::table
                    .filter(web_session_pending::user_id.eq(user_id))
                    .first::<PendingWebSession>(conn)
                    .optional()?;
                if let Some(pending) = current_pending {
                    diesel::delete(
                        web_session_pending::table.filter(web_session_pending::id.eq(pending.id)),
                    )
                    .execute(conn)?;
                }
                let res = diesel::insert_into(web_session_pending::table)
                    .values(NewPendingWebSession {
                        user_id,
                        token: token.to_string(),
                    })
                    .get_result::<PendingWebSession>(conn)?;
                Ok(res.id)
            }
        })
    }

    /// Create session.
    pub fn create_session(&self, user_id: i32, mobile: bool, token: &str) -> Result<i32> {
        self.db.run(|conn| {
            if mobile {
                let res = diesel::insert_into(mobile_session::table)
                    .values(NewMobileSession {
                        user_id,
                        token: token.to_owned(),
                    })
                    .get_result::<MobileSession>(conn)?;
                Ok(res.id)
            } else {
                let res = diesel::insert_into(web_session::table)
                    .values(NewWebSession {
                        user_id,
                        token: token.to_owned(),
                    })
                    .get_result::<WebSession>(conn)?;
                Ok(res.id)
            }
        })
    }

    /// Logout.
    pub fn logout(&self, user_id: i32, token: &str) -> Result<()> {
        self.db.run_transaction(|conn| {
            if let Some(mobile) = mobile_session::table
                .filter(mobile_session::user_id.eq(user_id))
                .filter(mobile_session::token.eq(token))
                .first::<MobileSession>(conn)
                .optional()?
            {
                diesel::delete(mobile_session::table)
                    .filter(mobile_session::id.eq(mobile.id))
                    .execute(conn)?;
                diesel::delete(mobile_session_pending::table)
                    .filter(mobile_session_pending::user_id.eq(user_id))
                    .execute(conn)?;
            }
            if let Some(web) = web_session::table
                .filter(web_session::user_id.eq(user_id))
                .filter(web_session::token.eq(token))
                .first::<WebSession>(conn)
                .optional()?
            {
                diesel::delete(web_session::table)
                    .filter(web_session::id.eq(web.id))
                    .execute(conn)?;
                diesel::delete(web_session_pending::table)
                    .filter(web_session_pending::user_id.eq(user_id))
                    .execute(conn)?;
            }
            diesel::insert_into(blacklist_token::table)
                .values(NewBlacklistToken {
                    token: token.to_string(),
                })
                .execute(conn)?;
            Ok(())
        })
    }
}

impl DatabaseRepository<PgConnection> for SessionsRepo {
    fn new(db: database::Database<PgConnection>) -> Self {
        Self { db }
    }
}
