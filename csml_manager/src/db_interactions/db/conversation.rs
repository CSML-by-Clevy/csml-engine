use crate::{Client, Conversation, ConversationInfo, Database, ManagerError};

pub fn create_conversation(
    flow_id: &str,
    step_id: &str,
    client: &Client,
    metadata: serde_json::Value,
    db: &Database,
) -> Result<String, ManagerError> {
    #[cfg(feature = "mongo")]
    if cfg!(feature = "mongo") && std::env::var("ENGINE_DB_TYPE") != Ok("http".to_owned()) {
        use crate::db_interactions::db_interactions_mongo::conversation::create_conversation as create;
        use crate::db_interactions::db_interactions_mongo::get_db;

        let db: &mongodb::Database = get_db(db)?;

        return create(flow_id, step_id, client, metadata, db);
    }

    #[cfg(feature = "http")]
    if cfg!(feature = "http") && std::env::var("ENGINE_DB_TYPE") == Ok("http".to_owned()) {
        use crate::db_interactions::db_interactions_http_db::conversation::create_conversation as create;
        use crate::db_interactions::db_interactions_http_db::get_db;

        let db: &http_db::apis::client::APIClient = get_db(db)?;

        return create(flow_id, step_id, client, metadata, db);
    }

    Err(ManagerError::Manager("db is not init correctly".to_owned()))
}

pub fn close_conversation(id: &String, client: &Client, db: &Database) -> Result<(), ManagerError> {
    #[cfg(feature = "mongo")]
    if cfg!(feature = "mongo") && std::env::var("ENGINE_DB_TYPE") != Ok("http".to_owned()) {
        use crate::db_interactions::db_interactions_mongo::conversation::close_conversation as close;
        use crate::db_interactions::db_interactions_mongo::get_db;

        let db: &mongodb::Database = get_db(db)?;

        return close(id, client, db);
    }

    #[cfg(feature = "http")]
    if cfg!(feature = "http") && std::env::var("ENGINE_DB_TYPE") == Ok("http".to_owned()) {
        use crate::db_interactions::db_interactions_http_db::conversation::close_conversation as close;
        use crate::db_interactions::db_interactions_http_db::get_db;

        let db: &http_db::apis::client::APIClient = get_db(db)?;

        return close(id, client, "CLOSED", db);
    }

    Err(ManagerError::Manager("db is not init correctly".to_owned()))
}

pub fn close_all_conversations(client: &Client, db: &Database) -> Result<(), ManagerError> {
    #[cfg(feature = "mongo")]
    if cfg!(feature = "mongo") && std::env::var("ENGINE_DB_TYPE") != Ok("http".to_owned()) {
        use crate::db_interactions::db_interactions_mongo::conversation::close_all_conversations as close_all;
        use crate::db_interactions::db_interactions_mongo::get_db;

        let db: &mongodb::Database = get_db(db)?;

        return close_all(client, db);
    }

    #[cfg(feature = "http")]
    if cfg!(feature = "http") && std::env::var("ENGINE_DB_TYPE") == Ok("http".to_owned()) {
        use crate::db_interactions::db_interactions_http_db::conversation::close_all_conversations as close_all;
        use crate::db_interactions::db_interactions_http_db::get_db;

        let db: &http_db::apis::client::APIClient = get_db(db)?;

        return close_all(client, db);
    }

    Err(ManagerError::Manager("db is not init correctly".to_owned()))
}

pub fn get_latest_open(
    client: &Client,
    db: &Database,
) -> Result<Option<Conversation>, ManagerError> {
    #[cfg(feature = "mongo")]
    if cfg!(feature = "mongo") && std::env::var("ENGINE_DB_TYPE") != Ok("http".to_owned()) {
        use crate::db_interactions::db_interactions_mongo::conversation::get_latest_open as get_latest;
        use crate::db_interactions::db_interactions_mongo::get_db;

        let db: &mongodb::Database = get_db(db)?;
        return get_latest(client, db);
    }

    #[cfg(feature = "http")]
    if cfg!(feature = "http") && std::env::var("ENGINE_DB_TYPE") == Ok("http".to_owned()) {
        use crate::db_interactions::db_interactions_http_db::conversation::get_latest_open as get_latest;
        use crate::db_interactions::db_interactions_http_db::get_db;

        let db: &http_db::apis::client::APIClient = get_db(db)?;

        return get_latest(client, db);
    }

    Err(ManagerError::Manager("db is not init correctly".to_owned()))
}

pub fn update_conversation(
    data: &ConversationInfo,
    flow_id: Option<String>,
    step_id: Option<String>,
) -> Result<(), ManagerError> {
    #[cfg(feature = "mongo")]
    if cfg!(feature = "mongo") && std::env::var("ENGINE_DB_TYPE") != Ok("http".to_owned()) {
        use crate::db_interactions::db_interactions_mongo::conversation::update_conversation as update;
        use crate::db_interactions::db_interactions_mongo::get_db;

        let db: &mongodb::Database = get_db(&data.db)?;

        return update(
            data.conversation_id.clone(),
            &data.client,
            flow_id,
            step_id,
            db,
        );
    }

    #[cfg(feature = "http")]
    if cfg!(feature = "http") && std::env::var("ENGINE_DB_TYPE") == Ok("http".to_owned()) {
        use crate::db_interactions::db_interactions_http_db::conversation::update_conversation as update;
        use crate::db_interactions::db_interactions_http_db::get_db;

        let db: &http_db::apis::client::APIClient = get_db(&data.db)?;

        return update(&data.conversation_id, &data.client, flow_id, step_id, db);
    }

    Err(ManagerError::Manager("db is not init correctly".to_owned()))
}
