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

use super::schema::*;
use diesel::prelude::*;

/// Device.
/// The device type.
#[derive(Clone, Selectable, Queryable, Identifiable)]
#[diesel(table_name = device)]
pub struct Device {
    pub id: i32,
    pub uid: String,
    pub description: String,
    pub address: String,
}

/// Schedule.
/// The schedule type.
#[derive(Clone, Selectable, Queryable, Identifiable)]
#[diesel(table_name = schedules)]
pub struct Schedule {
    pub id: i32,
    pub sid: i32,
    pub name: String,
    /// Vec<Option<i32>>
    pub days: String,
    pub record_url: String,
    pub kind: i32,
    /// Vec<Option<i32>>
    pub weeks: String,
    /// Vec<Option<i32>>
    pub dates: String,
    /// Vec<Option<String>>
    pub times: String,
    pub month: Option<i32>,
    pub year: Option<i32>,
    pub volume: Option<f64>,
}

/// NewSchedule.
/// The new schedule type.
#[derive(Clone, Insertable)]
#[diesel(table_name = schedules)]
pub struct NewSchedule {
    pub sid: i32,
    pub name: String,
    /// Vec<Option<i32>>
    pub days: String,
    pub record_url: String,
    pub kind: i32,
    /// Vec<Option<i32>>
    pub weeks: String,
    /// Vec<Option<i32>>
    pub dates: String,
    /// Vec<Option<String>>
    pub times: String,
    pub month: Option<i32>,
    pub year: Option<i32>,
    pub volume: Option<f64>,
}
