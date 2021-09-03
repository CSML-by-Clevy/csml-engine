#[cfg(feature = "dynamo")]
use crate::db_connectors::{dynamodb as dynamodb_connector, is_dynamodb};
#[cfg(feature = "mongo")]
use crate::db_connectors::{is_mongodb, mongodb as mongodb_connector};
#[cfg(feature = "postgresql")]
use crate::db_connectors::{is_postgresql, postgresql_connector};

use crate::error_messages::ERROR_DB_SETUP;
use crate::{ConversationInfo, EngineError};

pub fn create_node(
    _conversation: &mut ConversationInfo,
    _nextflow: Option<String>,
    _nextstep: Option<String>,
) -> Result<(), EngineError> {
    #[cfg(feature = "mongo")]
    if is_mongodb() {
        return mongodb_connector::nodes::create_node(_conversation, _nextflow, _nextstep);
    }

    #[cfg(feature = "dynamo")]
    if is_dynamodb() {
        return dynamodb_connector::nodes::create_node(_conversation, _nextflow, _nextstep);
    }

    #[cfg(feature = "postgresql")]
    if is_postgresql() {
        return postgresql_connector::nodes::create_node(_conversation, _nextflow, _nextstep);
    }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}
