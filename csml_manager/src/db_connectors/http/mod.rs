pub mod conversation;
pub mod interactions;
pub mod memories;
pub mod messages;
pub mod nodes;
pub mod state;

use crate::{Database, ManagerError};

#[cfg(feature = "http")]
use http_db::apis::{client::APIClient, configuration::Configuration};

pub fn init() -> Result<Database, ManagerError> {
    let conf = Configuration::new();
    let db = Database::Httpdb(APIClient::new(conf));

    Ok(db)
}

pub fn get_db<'a>(db: &'a Database) -> Result<&'a http_db::apis::client::APIClient, ManagerError> {
    match db {
        Database::Httpdb(httpdb) => Ok(httpdb),
        _ => Err(ManagerError::Manager("HTTP DB connector is not setup correctly".to_owned())),
    }
}
