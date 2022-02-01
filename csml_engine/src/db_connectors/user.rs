#[cfg(feature = "dynamo")]
use crate::db_connectors::{dynamodb as dynamodb_connector, is_dynamodb};
#[cfg(feature = "mongo")]
use crate::db_connectors::{is_mongodb, mongodb as mongodb_connector};
#[cfg(feature = "postgresql")]
use crate::db_connectors::{is_postgresql, postgresql_connector};

use csml_interpreter::data::csml_logs::{LogLvl, CsmlLog, csml_logger};
use crate::error_messages::ERROR_DB_SETUP;
use crate::{Client, Database, EngineError};

pub fn delete_client(client: &Client, db: &mut Database) -> Result<(), EngineError> {
    csml_logger(
        CsmlLog::new(
            None,
            None,
            None,
            format!("db call delete client")
        ),
        LogLvl::Info
    );
    csml_logger(
        CsmlLog::new(
            Some(client),
            None,
            None,
            format!("db call delete client")
        ),
        LogLvl::Debug
    );

    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let db = mongodb_connector::get_db(db)?;

        mongodb_connector::conversations::delete_user_conversations(client, db)?;
        mongodb_connector::memories::delete_client_memories(client, db)?;
        mongodb_connector::messages::delete_user_messages(client, db)?;
        mongodb_connector::state::delete_user_state(client, db)?;

        return Ok(())
    }

    #[cfg(feature = "dynamo")]
    if is_dynamodb() {
        let db = dynamodb_connector::get_db(db)?;

        dynamodb_connector::memories::delete_client_memories(client, db)?;
        dynamodb_connector::messages::delete_user_messages(client, db)?;
        dynamodb_connector::conversations::delete_user_conversations(client, db)?;
        dynamodb_connector::state::delete_user_state(client, db)?;

        return Ok(())
    }

    #[cfg(feature = "postgresql")]
    if is_postgresql() {
        let db = postgresql_connector::get_db(db)?;

        postgresql_connector::conversations::delete_user_conversations(client, db)?;
        postgresql_connector::memories::delete_client_memories(client, db)?;
        postgresql_connector::messages::delete_user_messages(client, db)?;
        postgresql_connector::state::delete_user_state(client, db)?;

        return Ok(())
    }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}
