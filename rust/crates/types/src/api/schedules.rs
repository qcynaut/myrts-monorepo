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
use super::{records::Records, user::User};
#[cfg(feature = "db")]
use diesel::prelude::*;
use types_rt::ty;

/// Schedules.
/// The schedules data.
#[derive(Clone)]
#[ty(db(kind: Query, table: schedules, relations: [Records, User]), web(Response))]
pub struct Schedules {
    pub id: i32,
    pub name: String,
    pub days: Vec<Option<i32>>,
    pub records_id: i32,
    pub device_ids: Vec<Option<i32>>,
    pub kind: i32,
    pub weeks: Vec<Option<i32>>,
    pub dates: Vec<Option<i32>>,
    pub times: Vec<Option<String>>,
    pub user_id: i32,
    pub month: Option<i32>,
    pub year: Option<i32>,
    pub volumes: Vec<Option<String>>,
}

/// Create new Schedule.
#[ty(db(kind: Insert, table: schedules))]
pub struct NewSchedule {
    pub name: String,
    pub days: Vec<Option<i32>>,
    pub records_id: i32,
    pub device_ids: Vec<Option<i32>>,
    pub kind: i32,
    pub weeks: Vec<Option<i32>>,
    pub dates: Vec<Option<i32>>,
    pub times: Vec<Option<String>>,
    pub user_id: i32,
    pub month: Option<i32>,
    pub year: Option<i32>,
    pub volumes: Vec<Option<String>>,
}

/// NewScheduleRequest.
#[ty(web(Request))]
pub struct NewScheduleReq {
    pub name: String,
    pub days: Vec<Option<i32>>,
    pub records_id: i32,
    pub device_ids: Vec<Option<i32>>,
    pub kind: i32,
    pub weeks: Vec<Option<i32>>,
    pub dates: Vec<Option<i32>>,
    pub times: Vec<Option<String>>,
    pub month: Option<i32>,
    pub year: Option<i32>,
    pub volumes: Vec<Option<String>>,
}

api_rt::schemas! {
    SchedulesResponse
    NewScheduleReq
}
