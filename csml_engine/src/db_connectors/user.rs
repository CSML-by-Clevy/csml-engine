#[cfg(feature = "dynamo")]
use crate::db_connectors::{dynamodb as dynamodb_connector, is_dynamodb};
#[cfg(feature = "mongo")]
use crate::db_connectors::{is_mongodb, mongodb as mongodb_connector};
use crate::error_messages::ERROR_DB_SETUP;
use crate::{Client, Database, EngineError};

pub fn delete_user(client: &Client, db: &mut Database) -> Result<(), EngineError> {
    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let db = mongodb_connector::get_db(db)?;

        mongodb_connector::conversations::delete_user_conversations(client, db)?;
        mongodb_connector::interactions::delete_user_interactions(client, db)?;
        mongodb_connector::memories::delete_user_memories(client, db)?;
        mongodb_connector::messages::delete_user_messages(client, db)?;
        mongodb_connector::state::delete_user_state(client, db)?;
        mongodb_connector::nodes::delete_conversation_nodes(client, db)?;

        return Ok(())
    }

    #[cfg(feature = "dynamo")]
    if is_dynamodb() {
        let db = dynamodb_connector::get_db(db)?;

        dynamodb_connector::memories::delete_user_memories(client, db)?;
        dynamodb_connector::messages::delete_user_messages(client, db)?;
        dynamodb_connector::interactions::delete_user_interactions(client, db)?;
        dynamodb_connector::conversations::delete_user_conversations(client, db)?;
        dynamodb_connector::state::delete_user_state(client, db)?;

        return Ok(())
    }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}
