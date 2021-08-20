pub mod client; // <-- rm ??

// pub mod bot;
pub mod conversations;
pub mod interactions;
pub mod memories;
// pub mod messages;
pub mod nodes;
pub mod state;

// pub mod pagination;

pub mod schema;
pub mod models;

use crate::{Database, EngineError, PostgresqlClient};

use diesel::prelude::*;
use std::env;

pub fn init() -> Result<Database, EngineError> {

    let uri = match std::env::var("POSTGRESQL_URL") {
        Ok(var) => var,
        _ => "".to_owned(),
    };

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pg_connection = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    let db = Database::Postgresql(
        PostgresqlClient::new(pg_connection)
    );
    Ok(db)
}

pub fn get_db<'a>(db: &'a Database) -> Result<&'a PostgresqlClient, EngineError> {
    match db {
        Database::Postgresql(db) => Ok(db),
        _ => Err(EngineError::Manager(
            "Postgresql connector is not setup correctly".to_owned(),
        )),
    }
}
