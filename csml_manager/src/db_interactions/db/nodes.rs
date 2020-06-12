use crate::{ConversationInfo, ManagerError};

pub fn create_node(data: &mut ConversationInfo) -> Result<(), ManagerError> {
    #[cfg(feature = "mongo")]
    if cfg!(feature = "mongo") {
        use crate::db_interactions::db_interactions_mongo::nodes::create_node as create;

        return create(data);
    }

    Err(ManagerError::Manager("db is not init correctly".to_owned()))
}
