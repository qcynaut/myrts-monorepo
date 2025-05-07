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

//! # database
//!
//! This crate provides the database interface for `myrts`.
//! Main purpose of this crate is to make any database operations have the same interface
//! across different databases.

pub use diesel;
use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection, R2D2Connection},
    Connection,
};
pub use diesel_migrations;

/// DatabaseError.
/// This is the error type that can occur when interacting with database.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DatabaseError {
    #[error("Error: connecting to database")]
    Connection,
    #[error("Error: resource not found")]
    NotFound,
    #[error("Error: conflict")]
    Conflict,
    #[error("Error: expired")]
    Expired,
    #[error("Error: {0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, DatabaseError>;

impl From<diesel::r2d2::PoolError> for DatabaseError {
    fn from(_: diesel::r2d2::PoolError) -> Self {
        DatabaseError::Connection
    }
}

impl From<diesel::result::Error> for DatabaseError {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::DatabaseError(kind, _) => match kind {
                diesel::result::DatabaseErrorKind::UniqueViolation => DatabaseError::Conflict,
                diesel::result::DatabaseErrorKind::ClosedConnection => DatabaseError::Connection,
                _ => DatabaseError::Other(err.to_string()),
            },
            diesel::result::Error::NotFound => DatabaseError::NotFound,
            _ => DatabaseError::Other(err.to_string()),
        }
    }
}

/// Database.
/// This is the database object that can be used to store the connection.
#[derive(Debug)]
pub struct Database<T: Connection + R2D2Connection + 'static>(Pool<ConnectionManager<T>>);

impl<T> Clone for Database<T>
where
    T: Connection + R2D2Connection + 'static,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Database<T>
where
    T: Connection + R2D2Connection + 'static,
{
    /// Creates a new database.
    pub fn new(db_url: &str) -> Result<Self> {
        let manager = ConnectionManager::<T>::new(db_url);
        let pool = Pool::builder().build(manager)?;
        Ok(Self(pool))
    }

    /// Runs a closure with a connection to the database.
    pub fn run<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&mut PooledConnection<ConnectionManager<T>>) -> Result<R>,
    {
        let mut conn = self.0.get()?;
        f(&mut conn)
    }

    /// Runs a transaction closure with a connection to the database.
    pub fn run_transaction<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&mut PooledConnection<ConnectionManager<T>>) -> Result<R>,
    {
        let mut conn = self.0.get()?;
        conn.transaction(f)
    }

    /// Creates a new repository.
    pub fn repository<R>(&self) -> R
    where
        R: DatabaseRepository<T>,
    {
        R::new(self.clone())
    }
}

/// DatabaseRepository.
pub trait DatabaseRepository<T>
where
    T: Connection + R2D2Connection + 'static,
{
    /// Create a new database repository.
    fn new(db: Database<T>) -> Self;
}
