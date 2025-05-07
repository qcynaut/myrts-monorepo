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
    avs::{Avs, NewAvs, PartialUpdateAvs, UpdateAvsInfo},
    schema::*,
};

/// AvsRepo.
/// Repository for `avs` table.
#[derive(Clone)]
pub struct AvsRepo {
    db: ApiDatabase,
}

impl AvsRepo {
    /// Get all avs.
    pub fn get_all(&self) -> Result<Vec<Avs>> {
        self.db
            .run(|conn| avs::table.load::<Avs>(conn).map_err(Into::into))
    }

    /// Update slots.
    pub fn update_slots(&self, id: i32, slots: &str) -> Result<()> {
        self.db.run(|conn| {
            diesel::update(avs::table.filter(avs::id.eq(id)))
                .set(avs::slots.eq(slots))
                .execute(conn)?;
            Ok(())
        })
    }

    /// Get many avs.
    pub fn get_many(&self, ids: &[i32]) -> Result<Vec<Avs>> {
        self.db.run(|conn| {
            avs::table
                .filter(avs::id.eq_any(ids))
                .load::<Avs>(conn)
                .map_err(Into::into)
        })
    }

    /// Create new avs.
    pub fn create(&self, avs: NewAvs) -> Result<Avs> {
        self.db.run(|conn| {
            diesel::insert_into(avs::table)
                .values(avs)
                .get_result(conn)
                .map_err(Into::into)
        })
    }

    /// Get by unique id.
    pub fn get_unique(&self, unique_id: &str) -> Result<Option<Avs>> {
        self.db.run(|conn| {
            avs::table
                .filter(avs::unique_id.eq(unique_id))
                .first::<Avs>(conn)
                .optional()
                .map_err(Into::into)
        })
    }

    /// Disconnect avs.
    pub fn disconnect(&self, unique_id: &str) -> Result<()> {
        self.db.run(|conn| {
            let _ = diesel::update(avs::table.filter(avs::unique_id.eq(unique_id)))
                .set(avs::status.eq(2))
                .execute(conn)?;
            Ok(())
        })
    }

    /// Disconnect all avs.
    pub fn disconnect_all(&self) -> Result<()> {
        self.db.run(|conn| {
            let _ = diesel::update(avs::table)
                .set(avs::status.eq(2))
                .execute(conn)?;
            Ok(())
        })
    }

    /// Connect avs.
    pub fn connect(&self, unique_id: &str) -> Result<()> {
        self.db.run(|conn| {
            let _ = diesel::update(avs::table.filter(avs::unique_id.eq(unique_id)))
                .set(avs::status.eq(1))
                .execute(conn)?;
            Ok(())
        })
    }

    /// Accept pending avs.
    pub fn accept(&self, id: i32) -> Result<Avs> {
        self.db.run(|conn| {
            diesel::update(avs::table.filter(avs::id.eq(id)))
                .set(avs::pending.eq(0))
                .get_result(conn)
                .map_err(Into::into)
        })
    }

    /// Update avs information.
    pub fn update_info(&self, avs: PartialUpdateAvs, id: i32) -> Result<Avs> {
        self.db.run(|conn| {
            diesel::update(avs::table.filter(avs::id.eq(id)))
                .set(avs)
                .get_result(conn)
                .map_err(Into::into)
        })
    }

    /// Update avs os specific info.
    pub fn update_os_info(&self, avs: UpdateAvsInfo, id: i32) -> Result<Avs> {
        self.db.run(|conn| {
            diesel::update(avs::table.filter(avs::id.eq(id)))
                .set(avs)
                .get_result(conn)
                .map_err(Into::into)
        })
    }

    /// Delete avs.
    pub fn delete(&self, id: i32) -> Result<()> {
        self.db.run(|conn| {
            diesel::delete(avs::table.filter(avs::id.eq(id))).execute(conn)?;
            Ok(())
        })
    }
}

impl DatabaseRepository<PgConnection> for AvsRepo {
    fn new(db: database::Database<PgConnection>) -> Self {
        Self { db }
    }
}
