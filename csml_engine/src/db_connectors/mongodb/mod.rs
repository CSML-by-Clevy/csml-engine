pub mod bot;
pub mod conversations;
pub mod memories;
pub mod messages;
pub mod state;

use crate::{Database, EngineError, MongoDbClient};
use bson::{doc, Document};
use core::time::Duration as CoreDuration;
use mongodb::{options::IndexOptions, IndexModel};

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

pub fn init() -> Result<Database<'static>, EngineError> {
    let dbname = match std::env::var("MONGODB_DATABASE") {
        Ok(var) => var,
        _ => {
            return Err(EngineError::Manager(format!(
                "Missing MONGODB_DATABASE in env"
            )))
        }
    };

    let uri = match std::env::var("MONGODB_URI") {
        Ok(var) => var,
        _ => create_mongodb_uri()?,
    };

    let client = mongodb::sync::Client::with_uri_str(&uri)?;
    let mongodb_client = MongoDbClient::new(client.database(&dbname));
    create_ttl_indexes(&mongodb_client);
    create_client_indexes(&mongodb_client);

    let db = Database::Mongo(mongodb_client);

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

fn create_ttl_indexes(db: &MongoDbClient) {
    // create index expires_at for conversation
    let conversation = db.client.collection::<Document>("conversation");
    let index: IndexModel = IndexModel::builder()
        .keys(doc! {
            "expires_at": 1
        })
        .options(Some(
            IndexOptions::builder()
                .expire_after(CoreDuration::new(0, 0))
                .build(),
        ))
        .build();
    conversation.create_index(index, None).ok();

    // create index expires_at for memory
    let memory = db.client.collection::<Document>("memory");
    let index: IndexModel = IndexModel::builder()
        .keys(doc! {
            "expires_at": 1
        })
        .options(Some(
            IndexOptions::builder()
                .expire_after(CoreDuration::new(0, 0))
                .build(),
        ))
        .build();

    memory.create_index(index, None).ok();

    // create index expires_at for message
    let message = db.client.collection::<Document>("message");
    let index: IndexModel = IndexModel::builder()
        .keys(doc! {
            "expires_at": 1
        })
        .options(Some(
            IndexOptions::builder()
                .expire_after(CoreDuration::new(0, 0))
                .build(),
        ))
        .build();
    message.create_index(index, None).ok();

    // create index expires_at for state
    let state = db.client.collection::<Document>("state");
    let index: IndexModel = IndexModel::builder()
        .keys(doc! {
            "expires_at": 1
        })
        .options(Some(
            IndexOptions::builder()
                .expire_after(CoreDuration::new(0, 0))
                .build(),
        ))
        .build();
    state.create_index(index, None).ok();
}

fn create_client_indexes(db: &MongoDbClient) {
    // create compound client index for conversation
    let conversation = db.client.collection::<Document>("conversation");
    let index: IndexModel = IndexModel::builder()
        .keys(doc! {
            "client.bot_id": 1,
            "client.channel_id": 1,
            "client.user_id": 1
        })
        .build();
    conversation.create_index(index, None).ok();

    // create compound client index for memory
    let memory = db.client.collection::<Document>("memory");
    let index: IndexModel = IndexModel::builder()
        .keys(doc! {
            "client.bot_id": 1,
            "client.channel_id": 1,
            "client.user_id": 1
        })
        .build();
    memory.create_index(index, None).ok();

    // create compound client index for message
    let message = db.client.collection::<Document>("message");
    let index: IndexModel = IndexModel::builder()
        .keys(doc! {
            "client.bot_id": 1,
            "client.channel_id": 1,
            "client.user_id": 1
        })
        .build();
    message.create_index(index, None).ok();

    // create compound client index for state
    let state = db.client.collection::<Document>("state");
    let index: IndexModel = IndexModel::builder()
        .keys(doc! {
            "client.bot_id": 1,
            "client.channel_id": 1,
            "client.user_id": 1
        })
        .build();
    state.create_index(index, None).ok();
}
