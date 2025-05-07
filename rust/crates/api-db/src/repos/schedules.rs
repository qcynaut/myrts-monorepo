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
    records::Records,
    schedules::{NewSchedule, Schedules},
    schema::*,
};

/// ScheduleRepo.
/// Repository for `schedules` table.
#[derive(Clone)]
pub struct ScheduleRepo {
    db: ApiDatabase,
}

impl ScheduleRepo {
    /// Get schedule by avs.
    pub fn get_by_avs(&self, avs: i32) -> Result<Vec<Schedules>> {
        self.db.run(|conn| {
            schedules::table
                .filter(schedules::device_ids.contains(vec![avs]))
                .load::<Schedules>(conn)
                .map_err(Into::into)
        })
    }

    /// Get schedule by avs with it's records.
    pub fn get_by_avs_with_records(&self, avs: i32) -> Result<Vec<(Schedules, Records)>> {
        self.db.run(|conn| {
            schedules::table
                .inner_join(records::table)
                .filter(schedules::device_ids.contains(vec![Some(avs)]))
                .load::<(Schedules, Records)>(conn)
                .map_err(Into::into)
        })
    }

    /// Create new schedule.
    pub fn create(&self, schedule: NewSchedule) -> Result<Schedules> {
        self.db.run(|conn| {
            diesel::insert_into(schedules::table)
                .values(&schedule)
                .get_result::<Schedules>(conn)
                .map_err(Into::into)
        })
    }

    /// Get schedule by id.
    pub fn get(&self, id: i32) -> Result<Schedules> {
        self.db.run(|conn| {
            schedules::table
                .filter(schedules::id.eq(id))
                .first::<Schedules>(conn)
                .map_err(Into::into)
        })
    }

    /// Get all schedules.
    pub fn get_all(&self) -> Result<Vec<Schedules>> {
        self.db
            .run(|conn| schedules::table.load::<Schedules>(conn).map_err(Into::into))
    }

    /// Get all schedule by user id.
    pub fn get_by_user(&self, user_id: i32) -> Result<Vec<Schedules>> {
        self.db.run(|conn| {
            schedules::table
                .filter(schedules::user_id.eq(user_id))
                .load::<Schedules>(conn)
                .map_err(Into::into)
        })
    }

    /// Delete schedule.
    pub fn delete(&self, id: i32) -> Result<Schedules> {
        self.db.run(|conn| {
            diesel::delete(schedules::table.filter(schedules::id.eq(id)))
                .get_result::<Schedules>(conn)
                .map_err(Into::into)
        })
    }
}

impl DatabaseRepository<PgConnection> for ScheduleRepo {
    fn new(db: database::Database<PgConnection>) -> Self {
        Self { db }
    }
}
