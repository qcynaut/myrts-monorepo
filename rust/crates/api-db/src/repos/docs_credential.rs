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

use database::{DatabaseRepository, Result};
use diesel::prelude::*;
use types::api::{docs_credential::DocCredential, schema::*};

use crate::ApiDatabase;

/// DocCredentialRepo.
/// Repository for `docs_credential` table.
#[derive(Clone)]
pub struct DocCredentialRepo {
    db: ApiDatabase,
}

impl DocCredentialRepo {
    /// Get all credentials.
    pub fn get_all(&self) -> Result<Vec<DocCredential>> {
        self.db.run(|conn| {
            let res = docs_credentials::table.load::<DocCredential>(conn)?;
            Ok(res)
        })
    }

    /// Get credentials by the given username.
    pub fn get(&self, username: &str) -> Result<DocCredential> {
        self.db.run(|conn| {
            let res = docs_credentials::table
                .filter(docs_credentials::username.eq(username))
                .first::<DocCredential>(conn)?;
            Ok(res)
        })
    }
}

impl DatabaseRepository<PgConnection> for DocCredentialRepo {
    fn new(db: database::Database<PgConnection>) -> Self {
        Self { db }
    }
}
