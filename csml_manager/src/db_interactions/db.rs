use std::env;
use crate::{
    data::{ManagerError, Database},
};

pub mod conversation;
pub mod interactions;
pub mod memories;
pub mod messages;
pub mod nodes;
pub mod state;

pub fn init_db() -> Result<Database, ManagerError> {

    #[cfg(feature = "mongo")]
    {
        let uri = match env::var("MONGODB_URI") {
            Ok(var) => var,
            _ => panic!("error no MONGODB_URI en env"),
        };

        let client = mongodb::Client::with_uri_str(&uri)?;
        let db = Database::Mongo(client.database("csml"));

        return Ok(db);
    }

    //dynamo {}

    Ok(Database::None)
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
