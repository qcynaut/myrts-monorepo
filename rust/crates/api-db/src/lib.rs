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

//! # api-db
//!
//! The database for myrts REST API.

pub use database::DatabaseError;
use database::{
    diesel_migrations::{self, embed_migrations, EmbeddedMigrations, MigrationHarness},
    Database, Result,
};
use diesel::{
    pg::Pg,
    prelude::*,
    query_builder::{QueryFragment, QueryId},
};
use types::api::{city::NewCity, province::NewProvince};
use utils::crypto::Bcrypt;

pub mod repos;

/// MIGRATIONS.
/// The database migrations code.
const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

/// ApiDatabase.
/// Contains some database functionalities for the api.
pub type ApiDatabase = Database<PgConnection>;

/// CreateDbStmt.
/// The sql statement to create database.
struct CreateDbStmt {
    database: String,
}
impl CreateDbStmt {
    /// Create new database.
    fn new(database: String) -> Self {
        Self { database }
    }
}
impl QueryFragment<Pg> for CreateDbStmt {
    fn walk_ast<'b>(
        &'b self,
        mut pass: diesel::query_builder::AstPass<'_, 'b, Pg>,
    ) -> QueryResult<()> {
        pass.push_sql("CREATE DATABASE ");
        pass.push_identifier(&self.database)?;
        Ok(())
    }
}
impl RunQueryDsl<PgConnection> for CreateDbStmt {}
impl QueryId for CreateDbStmt {
    type QueryId = ();

    const HAS_STATIC_QUERY_ID: bool = false;
}
fn change_database_of_url(database_url: &str, default_database: &str) -> (String, String) {
    let base = ::url::Url::parse(database_url).unwrap();
    let database = base.path_segments().unwrap().last().unwrap().to_owned();
    let mut new_url = base.join(default_database).unwrap();
    new_url.set_query(base.query());
    (database, new_url.into())
}

/// Setup province data if needed.
pub async fn setup_province_data(db: &ApiDatabase) -> std::result::Result<(), String> {
    let p_repo = db.repository::<repos::ProvinceRepo>();
    let c_repo = db.repository::<repos::CityRepo>();
    let res = p_repo.all().map_err(|e| e.to_string())?;
    if res.is_empty() {
        let data = utils::data::get_province_data()
            .await
            .map_err(|e| e.to_string())?;
        for province in data {
            let p = p_repo
                .create(NewProvince {
                    name: province.name.clone(),
                })
                .map_err(|e| e.to_string())?;
            for city in province.cities {
                c_repo
                    .create(NewCity {
                        name: city.name.clone(),
                        province_id: p.id,
                    })
                    .map_err(|e| e.to_string())?;
            }
        }
    }
    Ok(())
}

/// Setup initial data if needed.
pub fn setup_initial_data(db: &ApiDatabase) -> Result<()> {
    use types::api::{
        docs_credential::{DocCredential, NewDocCredential},
        schema::*,
        user::{NewUser, User},
    };

    db.run_transaction(|conn| {
        let creds = docs_credentials::table.load::<DocCredential>(conn)?;
        if creds.len() == 0 {
            diesel::insert_into(docs_credentials::table)
                .values(&NewDocCredential {
                    username: "brandio".to_string(),
                    password: "brandio".to_string(),
                })
                .execute(conn)?;
        }
        let user = users::table.load::<User>(conn)?;
        if user.len() == 0 {
            #[cfg(debug_assertions)]
            let (name, email, password) = (
                "Ade".to_owned(),
                "qcynaut@gmail.com".to_owned(),
                Bcrypt::hash("abc12345").unwrap_or_default(),
            );
            // #[cfg(not(debug_assertions))]
            let (name, email, password) = (
                "TSI".to_owned(),
                "tsi@sansathan.io".to_owned(),
                Bcrypt::hash(&utils::crypto::random_string(8)).unwrap_or_default(),
            );
            diesel::insert_into(users::table)
                .values(&NewUser {
                    name,
                    email,
                    password,
                    role_id: 1,
                    image_url: None,
                    city_id: None,
                    user_group_ids: vec![],
                    device_ids: vec![],
                })
                .execute(conn)?;
        }
        Ok(())
    })
}

/// Create a new database.
pub fn new_api_database(db_url: &str) -> Result<ApiDatabase> {
    {
        if PgConnection::establish(db_url).is_err() {
            let (database, new_url) = change_database_of_url(db_url, "postgres");
            let mut conn = PgConnection::establish(&new_url).unwrap();
            CreateDbStmt::new(database).execute(&mut conn).unwrap();
        }
        let mut conn = PgConnection::establish(db_url).map_err(|_| DatabaseError::Connection)?;
        conn.run_pending_migrations(MIGRATIONS)
            .map_err(|e| DatabaseError::Other(e.to_string()))?;
    }
    Database::new(db_url)
}
