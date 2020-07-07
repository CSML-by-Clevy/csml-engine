pub mod conversation;
pub mod interactions;
pub mod memories;
pub mod messages;
pub mod nodes;
pub mod state;

use crate::{Database, ManagerError};

pub fn get_db<'a>(db: &'a Database) -> Result<&'a http_db::apis::client::APIClient, ManagerError> {
    match db {
        Database::Httpdb(httpdb) => Ok(httpdb),
        _ => Err(ManagerError::Manager("db is not init correctly".to_owned())),
    }
}
