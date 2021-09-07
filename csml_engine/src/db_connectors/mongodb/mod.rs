pub mod bot;
pub mod conversations;
pub mod interactions;
pub mod memories;
pub mod messages;
pub mod nodes;
pub mod state;

use crate::{Database, EngineError, MongoDbClient};

fn create_mongodb_uri() -> Result<String, EngineError> {
    let mut uri = "mongodb://".to_owned();

    match (
        std::env::var("MONGODB_USERNAME"),
        std::env::var("MONGODB_PASSWORD"),
    ) {
        (Ok(username), Ok(password)) if !username.is_empty() && !password.is_empty() => {
            uri = format!("{}{}:{}@", uri, username, password)
        }
        _ => {}
    }

    match std::env::var("MONGODB_HOST") {
        Ok(host) => uri = format!("{}{}", uri, host),
        _ => return Err(EngineError::Manager(format!("Missing MONGODB_HOST in env"))),
    }

    match std::env::var("MONGODB_PORT") {
        Ok(var) => match var.parse::<u16>() {
            Ok(port) => uri = format!("{}:{}", uri, port),
            Err(err) => return Err(EngineError::Manager(err.to_string())),
        },
        _ => {}
    };

    Ok(uri)
}

pub fn init() -> Result<Database, EngineError> {
    let dbname = match std::env::var("MONGODB_DATABASE") {
        Ok(var) => var,
        _ => return Err(EngineError::Manager(format!("Missing MONGODB_DATABASE in env"))),
    };

    let uri = match std::env::var("MONGODB_URI") {
        Ok(var) => var,
        _ => create_mongodb_uri()?,
    };

    let client = mongodb::sync::Client::with_uri_str(&uri)?;

    let db = Database::Mongo(MongoDbClient::new(client.database(&dbname)));
    Ok(db)
}

pub fn get_db<'a>(db: &'a Database) -> Result<&'a MongoDbClient, EngineError> {
    match db {
        Database::Mongo(db) => Ok(db),
        _ => Err(EngineError::Manager(
            "MongoDB connector is not setup correctly".to_owned(),
        )),
    }
}

pub fn get_pagination_key(pagination_key: Option<String>) -> Result<Option<String>, EngineError> {
    match pagination_key {
        Some(key) => {
            let base64decoded = match base64::decode(&key) {
                Ok(base64decoded) => base64decoded,
                Err(_) => return Err(EngineError::Manager(format!("Invalid pagination_key"))),
            };

            let key: String = match serde_json::from_slice(&base64decoded) {
                Ok(key) => key,
                Err(_) => return Err(EngineError::Manager(format!("Invalid pagination_key"))),
            };

            Ok(Some(key))
        }
        None => Ok(None),
    }
}
