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

pub use database::DatabaseError;
use database::{
    diesel_migrations::{self, embed_migrations, EmbeddedMigrations, MigrationHarness},
    Database, Result,
};
use diesel::{Connection, SqliteConnection};

pub mod models;
pub mod repos;
mod schema;

/// MIGRATIONS.
/// The database migrations code.
const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

/// ProtoDatabase.
/// Contains some database functionalities for the proto.
pub type ProtoDatabase = Database<SqliteConnection>;

/// Create a new database.
pub fn new_proto_database(db_url: &str) -> Result<ProtoDatabase> {
    let path = std::path::Path::new(db_url);
    if !path.exists() {
        let dir = path.parent().unwrap();
        std::fs::create_dir_all(dir).unwrap();
        std::fs::File::create(db_url).unwrap();
    }
    let mut conn = SqliteConnection::establish(db_url).map_err(|_| DatabaseError::Connection)?;
    conn.run_pending_migrations(MIGRATIONS)
        .map_err(|e| DatabaseError::Other(e.to_string()))?;
    Database::new(db_url)
}
