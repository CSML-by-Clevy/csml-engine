pub mod bot;
pub mod conversations;
pub mod memories;
pub mod messages;
pub mod state;

pub mod pagination;

pub mod models;
pub mod schema;

pub mod expired_data;

use crate::{Database, EngineError, PostgresqlClient};

use diesel::prelude::{Connection, PgConnection};
use diesel_migrations::{EmbeddedMigrations, HarnessWithOutput, MigrationHarness};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/postgresql");

pub fn init() -> Result<Database<'static>, EngineError> {
    let uri = match std::env::var("POSTGRESQL_URL") {
        Ok(var) => var,
        _ => "".to_owned(),
    };

    let pg_connection =
        PgConnection::establish(&uri).unwrap_or_else(|_| panic!("Error connecting to {}", uri));

    let db = Database::Postgresql(PostgresqlClient::new(pg_connection));
    Ok(db)
}

pub fn make_migrations() -> Result<(), EngineError> {
    let uri = match std::env::var("POSTGRESQL_URL") {
        Ok(var) => var,
        _ => "".to_owned(),
    };

    let mut pg_connection =
        PgConnection::establish(&uri).unwrap_or_else(|_| panic!("Error connecting to {}", uri));

    let mut harness = HarnessWithOutput::write_to_stdout(&mut pg_connection);
    harness.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}

pub fn get_db<'a, 'b>(db: &'a mut Database<'b>) -> Result<&'a mut PostgresqlClient<'b>, EngineError> {
    match db {
        Database::Postgresql(db) => Ok(db),
        _ => Err(EngineError::Manager(
            "Postgresql connector is not setup correctly".to_owned(),
        )),
    }
}
