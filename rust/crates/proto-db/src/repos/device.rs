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

use crate::models::Device;
use crate::schema::*;
use crate::ProtoDatabase;
use database::DatabaseError;
use database::{DatabaseRepository, Result};
use diesel::prelude::*;

/// DeviceRepo.
/// Repository for `device` table.
#[derive(Clone)]
pub struct DeviceRepo {
    db: ProtoDatabase,
}

impl DeviceRepo {
    /// Check if the device exists.
    pub fn exists(&self) -> Result<bool> {
        self.db.run(|conn| {
            let res = device::table.load::<Device>(conn)?;
            Ok(!res.is_empty())
        })
    }

    /// Create a new device.
    pub fn create(&self, uid: &str, description: &str, address: &str) -> Result<()> {
        self.db.run(|conn| {
            diesel::insert_into(device::table)
                .values((
                    device::uid.eq(uid),
                    device::description.eq(description),
                    device::address.eq(address),
                ))
                .execute(conn)?;
            Ok(())
        })
    }

    /// Get device.
    pub fn get(&self) -> Result<Device> {
        self.db.run(|conn| {
            let res = device::table.load::<Device>(conn)?;
            match res.first() {
                Some(v) => Ok(v.clone()),
                None => Err(DatabaseError::NotFound),
            }
        })
    }
}

impl DatabaseRepository<SqliteConnection> for DeviceRepo {
    fn new(db: database::Database<SqliteConnection>) -> Self {
        Self { db }
    }
}
