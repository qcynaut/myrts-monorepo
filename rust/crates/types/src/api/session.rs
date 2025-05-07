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
#[cfg(feature = "db")]
use diesel::prelude::*;
use types_rt::ty;

/// WebSession.
/// The web_session data.
#[ty(db(kind: Query, table: web_session))]
pub struct WebSession {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
}

/// NewWebSession.
/// The new_web_session data.
#[ty(db(kind: Insert, table: web_session))]
pub struct NewWebSession {
    pub user_id: i32,
    pub token: String,
}

/// MobileSession.
/// The mobile_session data.
#[ty(db(kind: Query, table: mobile_session))]
pub struct MobileSession {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
}

/// NewMobileSession.
/// The new_mobile_session data.
#[ty(db(kind: Insert, table: mobile_session))]
pub struct NewMobileSession {
    pub user_id: i32,
    pub token: String,
}

/// PendingWebSession.
/// The pending_web_session data.
#[ty(db(kind: Query, table: web_session_pending))]
pub struct PendingWebSession {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
}

/// NewPendingWebSession.
/// The new_pending_web_session data.
#[ty(db(kind: Insert, table: web_session_pending))]
pub struct NewPendingWebSession {
    pub user_id: i32,
    pub token: String,
}

/// PendingMobileSession.
/// The pending_mobile_session data.
#[ty(db(kind: Query, table: mobile_session_pending))]
pub struct PendingMobileSession {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
}

/// NewPendingMobileSession.
/// The new_pending_mobile_session data.
#[ty(db(kind: Insert, table: mobile_session_pending))]
pub struct NewPendingMobileSession {
    pub user_id: i32,
    pub token: String,
}
