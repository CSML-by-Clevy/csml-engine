/**
 * This module defines the interactions between the CSML Manager and the underlying
 * database engines.
 *
 * There are several engines to choose from (see module features). To use one
 * of the database options, the ENGINE_DB_TYPE env var must be set
 * to one of the accepted values:
 *
 * - `mongodb`: requires a MongoDB-compatible database and additional variables:
 *   - MONGODB_HOST
 *   - MONGODB_PORT
 *   - MONGODB_DATABASE
 *   - MONGODB_USERNAME
 *   - MONGODB_PASSWORD
 * - `http`: if the developer wants to use a different DB engine, they can also create a HTTP-driven
 *   microservice to accomodate the required routes. If `http` mode is set, an additional variable
 *   is required:
 *   - HTTP_DB_MS_URL
 *
 * If the ENGINE_DB_TYPE env var is not set, mongodb is used by default.
 */

use crate::data::{Database, ManagerError};

use self::mongodb as mongodb_connector;
use self::http as http_connector;
use crate::error_messages::ERROR_DB_SETUP;

pub mod conversations;
pub mod interactions;
pub mod memories;
pub mod messages;
pub mod nodes;
pub mod state;

use crate::Client;

#[cfg(feature = "mongo")]
mod mongodb;
#[cfg(feature = "http")]
mod http;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Conversation {
    #[serde(rename = "_id")] // Use MongoDB's special primary key field name when serializing
    pub id: String,
    pub client: Client,
    pub flow_id: String,
    pub step_id: String,
    pub metadata: serde_json::Value,
    pub status: String,
    pub last_interaction_at: String,
    pub updated_at: String,
    pub created_at: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Interaction {
    #[serde(rename = "_id")] // Use MongoDB's special primary key field name when serializing
    pub id: String,
    pub client: Client,
    pub success: bool,
    pub event: serde_json::Value,
    pub updated_at: String,
    pub created_at: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct DbMemories {
    #[serde(rename = "_id")] // Use MongoDB's special primary key field name when serializing
    pub id: String,
    pub client: Client,
    pub interaction_id: String,
    pub conversation_id: String,
    pub flow_id: String,
    pub step_id: String,
    pub memory_order: i32,
    pub interaction_order: i32,
    pub key: String,
    pub value: String,
    pub expires_at: Option<String>,
    pub created_at: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Messages {
    #[serde(rename = "_id")] // Use MongoDB's special primary key field name when serializing
    pub id: String,
    pub client: Client,
    pub interaction_id: String,
    pub conversation_id: String,
    pub flow_id: String,
    pub step_id: String,
    pub message_order: i32,
    pub interaction_order: i32,
    pub direction: String,
    pub payload: String,
    pub content_type: String,
    pub created_at: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Node {
    #[serde(rename = "_id")] // Use MongoDB's special primary key field name when serializing
    pub id: String,
    pub client: Client,
    pub interaction_id: String,
    pub conversation_id: String,
    pub flow_id: String,
    pub step_id: String,
    pub next_step: Option<String>,
    pub next_flow: Option<String>,
    pub created_at: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct State {
    #[serde(rename = "_id")] // Use MongoDB's special primary key field name when serializing
    pub id: String,
    pub client: Client,
    #[serde(rename = "type")]
    pub _type: String,
    pub value: String,
    pub expires_at: Option<String>,
    pub created_at: String,
}


#[cfg(feature = "mongo")]
pub fn is_mongodb() -> bool {
    // If the env var is not available, use mongodb by default
    match std::env::var("ENGINE_DB_TYPE") {
        Ok(val) => val == "mongodb".to_owned(),
        Err(_) => true
    }
}

#[cfg(feature = "http")]
pub fn is_http() -> bool {
    match std::env::var("ENGINE_DB_TYPE") {
        Ok(val) => val == "http".to_owned(),
        Err(_) => false
    }
}


pub fn init_db() -> Result<Database, ManagerError> {
    #[cfg(feature = "mongo")]
    if is_mongodb() {
        return mongodb_connector::init();
    }

    #[cfg(feature = "http")]
    if is_http() {
        return http_connector::init();
    }

    Err(ManagerError::Manager(ERROR_DB_SETUP.to_owned()))
}

// ############################## conversation
// create_conversation(&mut core, &api_client); // OK return ConversationModel
// get_conversation(&mut core, &api_client); // OK ConversationModel
// get_latest_open(&mut core, &api_client); // OK InlineResponse200
// close_all_conversations(&mut core, &api_client); // Ok return ()
// close_conversation(&mut core, &api_client); // Ok return ()
// update_conversation(&mut core, &api_client); // Ok return ()
// ##############################

// ############################## memories
// add_memories_bulk(&mut core, &api_client); // OK ()
// add_memory(&mut core, &api_client); // OK ()
// get_current_memories(&mut core, &api_client); // OK [memories]
// get_past_memories(&mut core, &api_client); // OK [memories]
// ##############################

// ############################## messages
// add_messages(&mut core, &api_client); // Ok return ()
// add_messages_bulk(&mut core, &api_client); // Ok return ()
// ##############################

// ############################## Nodes
// create_node(&mut core, &api_client); // OK ()
// get_conversation_nodes(&mut core, &api_client); // OK NodeModel
// ##############################

// ############################## Interactions
// get_interaction(&mut core, &api_client); // InteractionModel
// get_interaction_status(&mut core, &api_client); // Ok InlineResponse2001
// get_lock_status(&mut core, &api_client); // Ok InlineResponse2002
// init_interaction(&mut core, &api_client); // OK InteractionModel
// update_interaction(&mut core, &api_client); // OK ()
// ##############################

// ############################## State
// clear_full_state(bot_id: &str, user_id: &str, channel_id: &str) // OK ()
// delete_item_state(composite_key: &str, bot_id: &str, user_id: &str, channel_id: &str) // OK ()
// get_full_state(bot_id: &str, user_id: &str, channel_id: &str) // OK StateModel
// get_item_state(composite_key: &str, bot_id: &str, user_id: &str, channel_id: &str) // OK StateModel
// set_item_state(composite_key: &str, bot_id: &str, user_id: &str, channel_id: &str, set_state_body: SetStateBody) // OK StateModel
// ##############################

// delete_state_full(&self, bot_id: &str, user_id: &str, channel_id: &str) -> Result<(), Error>;
// get_state_full(&self, bot_id: &str, user_id: &str, channel_id: &str) -> Result<Vec<crate::models::StateModel>, Error>;

// delete_state_type(&self, _type: &str, bot_id: &str, user_id: &str, channel_id: &str) -> Result<(), Error>;
// get_state_type(&self, _type: &str, bot_id: &str, user_id: &str, channel_id: &str) -> Result<Vec<crate::models::StateModel>, Error>;

// delete_state_key(&self, _type: &str, key: &str, bot_id: &str, user_id: &str, channel_id: &str) -> Result<(), Error>;
// get_state_key(&self, _type: &str, key: &str, bot_id: &str, user_id: &str, channel_id: &str) -> Result<crate::models::StateModel, Error>;

// set_state_items(&self, bot_id: &str, user_id: &str, channel_id: &str, create_state_body: Vec<crate::models::CreateStateBody>) -> Result<crate::models::StateModel, Error>;
