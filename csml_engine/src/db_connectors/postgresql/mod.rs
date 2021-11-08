pub mod bot;
pub mod conversations;
pub mod memories;
pub mod messages;
pub mod state;

pub mod pagination;

pub mod schema;
pub mod models;

pub mod expired_data;

use crate::{Database, EngineError, PostgresqlClient};

use diesel::prelude::*;

embed_migrations!("migrations/postgresql");

pub fn init() -> Result<Database, EngineError> {

    let uri = match std::env::var("POSTGRESQL_URL") {
        Ok(var) => var,
        _ => "".to_owned(),
    };

    let pg_connection = PgConnection::establish(&uri)
        .unwrap_or_else(|_| panic!("Error connecting to {}", uri));

    let db = Database::Postgresql(
        PostgresqlClient::new(pg_connection)
    );
    Ok(db)
}

pub fn make_migrations() -> Result<(), EngineError> {
    let uri = match std::env::var("POSTGRESQL_URL") {
        Ok(var) => var,
        _ => "".to_owned(),
    };

    let pg_connection = PgConnection::establish(&uri)
        .unwrap_or_else(|_| panic!("Error connecting to {}", uri));

    embedded_migrations::run_with_output(&pg_connection, &mut std::io::stdout())?;

    Ok(())
}

pub fn get_db<'a>(db: &'a Database) -> Result<&'a PostgresqlClient, EngineError> {
    match db {
        Database::Postgresql(db) => Ok(db),
        _ => Err(EngineError::Manager(
            "Postgresql connector is not setup correctly".to_owned(),
        )),
    }
}
