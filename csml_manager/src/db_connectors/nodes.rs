use crate::{ConversationInfo, ManagerError};
use crate::db_connectors::{is_mongodb, is_http};
use crate::error_messages::ERROR_DB_SETUP;
#[cfg(feature = "mongo")]
use crate::db_connectors::mongodb as mongodb_connector;
#[cfg(feature = "http")]
use crate::db_connectors::http as http_connector;

pub fn create_node(
    conversation: &mut ConversationInfo,
    nextflow: Option<String>,
    nextstep: Option<String>,
) -> Result<(), ManagerError> {
    #[cfg(feature = "mongo")]
    if is_mongodb() {
        return mongodb_connector::nodes::create_node(conversation, nextflow, nextstep);
    }

    #[cfg(feature = "http")]
    if is_http() {
        return http_connector::nodes::create_node(conversation, nextflow, nextstep);
    }

    Err(ManagerError::Manager(ERROR_DB_SETUP.to_owned()))
}
