#[cfg(feature = "dynamo")]
use crate::db_connectors::{is_dynamodb};
#[cfg(feature = "mongo")]
use crate::db_connectors::{is_mongodb};
#[cfg(feature = "postgresql")]
use crate::db_connectors::{is_postgresql, postgresql_connector};
#[cfg(feature = "sqlite")]
use crate::db_connectors::{is_sqlite, sqlite_connector};


use crate::error_messages::ERROR_DB_SETUP;
use crate::{Database, EngineError};

pub fn delete_expired_data(_db: &mut Database) -> Result<(), EngineError> {

    #[cfg(feature = "mongo")]
    if is_mongodb() {

        return Ok(())
    }

    #[cfg(feature = "dynamo")]
    if is_dynamodb() {

        return Ok(())
    }

    #[cfg(feature = "postgresql")]
    if is_postgresql() {
        let db = postgresql_connector::get_db(_db)?;

        postgresql_connector::expired_data::delete_expired_data(db)?;

        return Ok(())
    }

    #[cfg(feature = "sqlite")]
    if is_sqlite() {
        let db = sqlite_connector::get_db(_db)?;

        sqlite_connector::expired_data::delete_expired_data(db)?;

        return Ok(())
    }


    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}
