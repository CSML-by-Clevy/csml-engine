pub mod bot;
pub mod conversations;
pub mod memories;
pub mod messages;
pub mod state;

pub mod pagination;

pub mod models;

pub mod schema;

pub mod expired_data;

use crate::{Database, EngineError, SqliteClient};

use diesel::prelude::*;
use diesel_migrations::{EmbeddedMigrations, HarnessWithOutput, MigrationHarness};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/sqlite");

pub fn init() -> Result<Database, EngineError> {
    let uri = match std::env::var("SQLITE_URL") {
        Ok(var) => var,
        _ => "".to_owned(),
    };

    let sqlite_connection =
        SqliteConnection::establish(&uri).unwrap_or_else(|_| panic!("Error connecting to {}", uri));

    let db = Database::SqLite(SqliteClient::new(sqlite_connection));
    Ok(db)
}

pub fn make_migrations() -> Result<(), EngineError> {
    let uri = match std::env::var("SQLITE_URL") {
        Ok(var) => var,
        _ => "".to_owned(),
    };

    let mut sqlite_connection =
        SqliteConnection::establish(&uri).unwrap_or_else(|_| panic!("Error connecting to {}", uri));

    let mut harness = HarnessWithOutput::write_to_stdout(&mut sqlite_connection);
    harness.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}

pub fn get_db<'a>(db: &'a mut Database) -> Result<&'a mut SqliteClient, EngineError> {
    match db {
        Database::SqLite(db) => Ok(db),
        _ => Err(EngineError::Manager(
            "SqLite connector is not setup correctly".to_owned(),
        )),
    }
}
