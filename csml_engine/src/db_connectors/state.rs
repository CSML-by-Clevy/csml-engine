#[cfg(feature = "dynamo")]
use crate::db_connectors::{dynamodb as dynamodb_connector, is_dynamodb};
#[cfg(feature = "mongo")]
use crate::db_connectors::{is_mongodb, mongodb as mongodb_connector};
use crate::error_messages::ERROR_DB_SETUP;
use crate::{Database, EngineError};
use csml_interpreter::data::Client;

pub fn delete_state_key(
    client: &Client,
    _type: &str,
    _key: &str,
    db: &mut Database,
) -> Result<(), EngineError> {
    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let db = mongodb_connector::get_db(db)?;
        return mongodb_connector::state::delete_state_key(client, _type, _key, db);
    }

    #[cfg(feature = "dynamo")]
    if is_dynamodb() {
        let db = dynamodb_connector::get_db(db)?;
        return dynamodb_connector::state::delete_state_key(client, _type, _key, db);
    }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}

pub fn get_state_key(
    client: &Client,
    _type: &str,
    _key: &str,
    db: &mut Database,
) -> Result<Option<serde_json::Value>, EngineError> {
    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let db = mongodb_connector::get_db(db)?;
        return mongodb_connector::state::get_state_key(client, _type, _key, db);
    }

    #[cfg(feature = "dynamo")]
    if is_dynamodb() {
        let db = dynamodb_connector::get_db(db)?;
        return dynamodb_connector::state::get_state_key(client, _type, _key, db);
    }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}

pub fn set_state_items(
    client: &Client,
    _type: &str,
    keys_values: Vec<(&str, &serde_json::Value)>,
    db: &mut Database,
) -> Result<(), EngineError> {
    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let db = mongodb_connector::get_db(db)?;
        return mongodb_connector::state::set_state_items(client, _type, keys_values, &db);
    }

    #[cfg(feature = "dynamo")]
    if is_dynamodb() {
        let db = dynamodb_connector::get_db(db)?;
        return dynamodb_connector::state::set_state_items(client, _type, keys_values, db);
    }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db_connectors::init_db;
    use crate::{Hold, IndexInfo};
    use core::panic;

    #[test]
    fn ok_hold() {
        let client = Client{bot_id:"bot_id".to_owned(), channel_id: "channel_id".to_owned(), user_id: "test".to_owned()};
        let mut db = init_db().unwrap();

        let hash = "Hash".to_owned();
        let index_info = Hold {
            index: IndexInfo { command_index: 42, loop_index: vec!()},
            step_vars: serde_json::json!({}),
            step_name: "step_name".to_owned(),
            flow_name: "flow_name".to_owned(),
        };

        let state_hold: serde_json::Value = serde_json::json!({
            "index": index_info.index,
            "step_vars": index_info.step_vars,
            "hash": hash
        });

        set_state_items(&client, "hold", vec![("position", &state_hold)], &mut db).unwrap();

        let hold  = get_state_key(&client, "hold", "position", &mut db).unwrap().unwrap();

        let index_result = match serde_json::from_value::<IndexInfo>(hold["index"].clone()) {
            Ok(index) => index,
            Err(_) => panic!("value not found in db")
        };

        if index_result.loop_index != index_info.index.loop_index && index_result.command_index != index_info.index.command_index {
            panic!("db get hodl got the wrong value")
        }

        delete_state_key(&client, "hold", "position", &mut db).unwrap();

        match get_state_key(&client, "hold", "position", &mut db).unwrap() {
            Some(_value) => panic!("get_state_key should not have found a hold because it has deleted just before"),
            None => {},
        }
    }
}
