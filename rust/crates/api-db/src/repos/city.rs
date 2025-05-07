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
    city::{City, NewCity},
    schema::*,
};

/// CityRepo.
/// Repository for `city` table.
#[derive(Clone)]
pub struct CityRepo {
    db: ApiDatabase,
}

impl CityRepo {
    /// Create a new city.
    pub fn create(&self, data: NewCity) -> Result<City> {
        self.db.run(|conn| {
            let res = diesel::insert_into(city::table)
                .values(&data)
                .get_result(conn)?;
            Ok(res)
        })
    }

    /// Get all cities.
    pub fn get_all(&self) -> Result<Vec<City>> {
        self.db.run(|conn| {
            city::table
                .order(city::name.asc())
                .load::<City>(conn)
                .map_err(Into::into)
        })
    }

    /// Get all cities with pagination.
    pub fn get_all_paginated(&self, page: i64) -> Result<(Vec<City>, i64, i64)> {
        self.db.run(|conn| {
            let res = city::table
                .order(city::name.asc())
                .limit(10)
                .offset((page - 1) * 10)
                .load::<City>(conn)?;
            let total = city::table.count().first::<i64>(conn)?;
            let total_pages = (total - 1) / 10 + 1;
            Ok((res, total, total_pages))
        })
    }

    /// Get cities by it's province id.
    pub fn get_by_province_id(&self, province_id: i32) -> Result<Vec<City>> {
        self.db.run(|conn| {
            city::table
                .filter(city::province_id.eq(province_id))
                .load::<City>(conn)
                .map_err(Into::into)
        })
    }
}

impl DatabaseRepository<PgConnection> for CityRepo {
    fn new(db: database::Database<PgConnection>) -> Self {
        Self { db }
    }
}
