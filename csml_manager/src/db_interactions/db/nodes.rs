use crate::{ConversationInfo, ManagerError};

pub fn create_node(
    data: &mut ConversationInfo,
    nextflow: Option<String>,
    nextstep: Option<String>,
) -> Result<(), ManagerError> {
    #[cfg(feature = "mongo")]
    if cfg!(feature = "mongo") {
        use crate::db_interactions::db_interactions_mongo::nodes::create_node as create;

        return create(data, nextflow, nextstep);
    }

    #[cfg(feature = "dynamo")]
    if cfg!(feature = "dynamo") {
        use crate::db_interactions::db_interactions_dynamo::nodes::create_node as create;

        return create(data, nextflow, nextstep);
    }

    Err(ManagerError::Manager("db is not init correctly".to_owned()))
}
