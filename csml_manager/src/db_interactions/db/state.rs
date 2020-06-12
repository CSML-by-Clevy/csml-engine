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

    Err(ManagerError::Manager("db is not init correctly".to_owned()))
}

pub fn set_state_items(
    _data: &mut ConversationInfo,
    _type: &str,
    _keys_values: Vec<(&str, &serde_json::Value)>,
) -> Result<(), ManagerError> {
    // Document
    #[cfg(feature = "mongo")]
    if cfg!(feature = "mongo") {
        use crate::db_interactions::db_interactions_mongo::state::set_state_items;

        return set_state_items(_data, _type, _keys_values);
    }

    Err(ManagerError::Manager("db is not init correctly".to_owned()))
}
