pub mod conversations;
pub mod interactions;
pub mod memories;
pub mod messages;
pub mod nodes;
pub mod state;
pub mod bot;

use crate::{Database, EngineError};

fn init_mongo_credentials() -> Option<mongodb::options::auth::Credential> {
    let username = match std::env::var("MONGODB_USERNAME") {
        Ok(var) if var.len() > 0 => Some(var),
        _ => None,
    };
    let password = match std::env::var("MONGODB_PASSWORD") {
        Ok(var) if var.len() > 0 => Some(var),
        _ => None,
    };

    if let (&None, &None) = (&username, &password) {
        return None;
    }

    let credentials = mongodb::options::auth::Credential::builder()
        .password(password)
        .username(username)
        .build();

    Some(credentials)
}

pub fn init() -> Result<Database, EngineError> {
    let hostname = match std::env::var("MONGODB_HOST") {
        Ok(var) => var,
        _ => panic!("Missing MONGODB_HOST in env"),
    };

    let dbname = match std::env::var("MONGODB_DATABASE") {
        Ok(var) => var,
        _ => panic!("Missing MONGODB_DATABASE in env"),
    };

    let port: Option<u16> = match std::env::var("MONGODB_PORT") {
        Ok(var) => match var.parse::<u16>() {
            Ok(port) => Some(port),
            Err(err) => return Err(EngineError::Manager(err.to_string())),
        },
        _ => None,
    };

    let credentials = init_mongo_credentials();

    let options = mongodb::options::ClientOptions::builder()
        .hosts(vec![mongodb::options::StreamAddress {
            hostname: hostname.into(),
            port,
        }])
        .credential(credentials)
        .build();

    let client = mongodb::Client::with_options(options)?;
    let db = Database::Mongo(client.database(&dbname));
    Ok(db)
}

pub fn get_db<'a>(db: &'a Database) -> Result<&'a mongodb::Database, EngineError> {
    match db {
        Database::Mongo(db) => Ok(db),
        _ => Err(EngineError::Manager(
            "MongoDB connector is not setup correctly".to_owned(),
        )),
    }
}
