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
    city::City,
    province::{NewProvince, Province},
    schema::*,
};

/// ProvinceRepo.
/// Repository for `province` table.
#[derive(Clone)]
pub struct ProvinceRepo {
    db: ApiDatabase,
}

impl ProvinceRepo {
    /// Get all provinces.
    pub fn all(&self) -> Result<Vec<Province>> {
        self.db
            .run(|conn| province::table.load::<Province>(conn).map_err(Into::into))
    }

    /// Create new province.
    pub fn create(&self, province: NewProvince) -> Result<Province> {
        self.db.run(|conn| {
            diesel::insert_into(province::table)
                .values(&province)
                .get_result(conn)
                .map_err(Into::into)
        })
    }

    /// Get all province along with cities.
    pub fn get_all_related(&self) -> Result<Vec<(Province, Vec<City>)>> {
        self.db.run(|conn| {
            let res = province::table.load::<Province>(conn)?;
            let cities = City::belonging_to(&res).load::<City>(conn)?;
            let grouped = cities
                .grouped_by(&res)
                .into_iter()
                .zip(res)
                .map(|(c, p)| (p, c))
                .collect();
            Ok(grouped)
        })
    }
}

impl DatabaseRepository<PgConnection> for ProvinceRepo {
    fn new(db: database::Database<PgConnection>) -> Self {
        Self { db }
    }
}
