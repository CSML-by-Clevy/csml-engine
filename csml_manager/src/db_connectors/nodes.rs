#[cfg(feature = "dynamo")]
use crate::db_connectors::{dynamodb as dynamodb_connector, is_dynamodb};
#[cfg(feature = "mongo")]
use crate::db_connectors::{is_mongodb, mongodb as mongodb_connector};
use crate::error_messages::ERROR_DB_SETUP;
use crate::{ConversationInfo, ManagerError};

pub fn create_node(
    conversation: &mut ConversationInfo,
    nextflow: Option<String>,
    nextstep: Option<String>,
) -> Result<(), ManagerError> {
    #[cfg(feature = "mongo")]
    if is_mongodb() {
        return mongodb_connector::nodes::create_node(conversation, nextflow, nextstep);
    }

    #[cfg(feature = "dynamo")]
    if is_dynamodb() {
        return dynamodb_connector::nodes::create_node(conversation, nextflow, nextstep);
    }

    Err(ManagerError::Manager(ERROR_DB_SETUP.to_owned()))
}
