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

use crate::{
    models::{NewSchedule, Schedule},
    schema::*,
    ProtoDatabase,
};
use database::{DatabaseRepository, Result};
use diesel::prelude::*;

/// ScheduleRepo.
/// Repository for `schedules` table.
#[derive(Clone)]
pub struct ScheduleRepo {
    db: ProtoDatabase,
}

impl ScheduleRepo {
    /// Get all schedules.
    pub fn get(&self) -> Result<Vec<Schedule>> {
        self.db
            .run(|conn| schedules::table.load::<Schedule>(conn).map_err(Into::into))
    }

    /// Get sids.
    pub fn get_sids(&self) -> Result<Vec<i32>> {
        self.db.run(|conn| {
            schedules::table
                .select(schedules::sid)
                .load::<i32>(conn)
                .map_err(Into::into)
        })
    }

    /// Create new schedule.
    pub fn create(&self, schedule: NewSchedule) -> Result<()> {
        self.db.run(|conn| {
            diesel::insert_into(schedules::table)
                .values(&schedule)
                .execute(conn)?;
            Ok(())
        })
    }

    /// Remove multiple schedules by sid.
    pub fn remove(&self, sids: Vec<i32>) -> Result<()> {
        self.db.run(|conn| {
            let _ = diesel::delete(schedules::table)
                .filter(schedules::sid.eq_any(sids))
                .execute(conn)?;
            Ok(())
        })
    }
}

impl DatabaseRepository<SqliteConnection> for ScheduleRepo {
    fn new(db: database::Database<SqliteConnection>) -> Self {
        ScheduleRepo { db }
    }
}
