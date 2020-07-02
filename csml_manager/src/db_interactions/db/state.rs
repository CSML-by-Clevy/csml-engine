use crate::{ConversationInfo, Database, ManagerError};
use csmlinterpreter::data::Client;

pub fn delete_state_key(
    _client: &Client,
    _type: &str,
    _key: &str,
    _db: &Database,
) -> Result<(), ManagerError> {
    #[cfg(feature = "mongo")]
    if cfg!(feature = "mongo") {
        use crate::db_interactions::db_interactions_mongo::get_db;
        use crate::db_interactions::db_interactions_mongo::state::delete_state_key as delete;

        let db: &mongodb::Database = get_db(_db)?;

        return delete(_client, _type, _key, db);
    }

    #[cfg(feature = "http")]
    if cfg!(feature = "http") {
        use crate::db_interactions::db_interactions_http_db::get_db;
        use crate::db_interactions::db_interactions_http_db::state::delete_state_key as delete;

        let db: &http_db::apis::client::APIClient = get_db(_db)?;

        return delete(_client, _type, _key, db);
    }

    Err(ManagerError::Manager("db is not init correctly".to_owned()))
}

// pub fn get_state_type(
//     client: &Client,
//     _type: &str,
//     db: &mongodb::Database,
// ) -> Result<mongodb::Cursor, Error> {
//     let state = db.collection("state");

//     let filter = doc! {
//         "client": bson::to_bson(client)?,
//         "type": _type,
//     };
//     let cursor = state.find(filter, None)?;

//     Ok(cursor)
// }

pub fn get_state_key(
    _client: &Client,
    _type: &str,
    _key: &str,
    _db: &Database,
) -> Result<Option<serde_json::Value>, ManagerError> {
    #[cfg(feature = "mongo")]
    if cfg!(feature = "mongo") {
        use crate::db_interactions::db_interactions_mongo::get_db;
        use crate::db_interactions::db_interactions_mongo::state::get_state_key;

        let db: &mongodb::Database = get_db(_db)?;

        return get_state_key(_client, _type, _key, db);
    }

    #[cfg(feature = "http")]
    if cfg!(feature = "http") {
        use crate::db_interactions::db_interactions_http_db::get_db;
        use crate::db_interactions::db_interactions_http_db::state::get_state_key;

        let db: &http_db::apis::client::APIClient = get_db(_db)?;

        return get_state_key(_client, _type, _key, db);
    }

    Err(ManagerError::Manager("db is not init correctly".to_owned()))
}

pub fn set_state_items(
    _data: &mut ConversationInfo,
    _type: &str,
    _interaction_order: i32,
    _keys_values: Vec<(&str, &serde_json::Value)>,
) -> Result<(), ManagerError> {
    // Document
    #[cfg(feature = "mongo")]
    if cfg!(feature = "mongo") {
        use crate::db_interactions::db_interactions_mongo::state::set_state_items as set_items;

        return set_items(_data, _type, _keys_values);
    }

    #[cfg(feature = "http")]
    if cfg!(feature = "http") {
        use crate::db_interactions::db_interactions_http_db::state::format_state_body;
        use crate::db_interactions::db_interactions_http_db::state::set_state_items as set_items;

        use crate::db_interactions::db_interactions_http_db::get_db;

        let state_body = format_state_body(_data, _type, _interaction_order, _keys_values);
        let db: &http_db::apis::client::APIClient = get_db(&_data.db)?;

        return set_items(&_data.client, state_body, db);
    }

    Err(ManagerError::Manager("db is not init correctly".to_owned()))
}
