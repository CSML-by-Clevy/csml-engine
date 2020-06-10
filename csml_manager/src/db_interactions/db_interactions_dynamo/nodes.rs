use crate::ConversationInfo;
use dynamodb::{apis::Error, models::CreateNodeBody};
// use serde_json::Value;
use uuid::Uuid;

pub fn new_node(
    data: &mut ConversationInfo,
    // next_flow: Option<String>,
    // next_step: Option<String>,
) -> CreateNodeBody {
    CreateNodeBody {
        id: Uuid::new_v4().to_string(),
        interaction_id: data.interaction_id.to_owned(),
        flow_id: data.flow_info.flow.id.to_owned(),
        next_flow: Some(data.flow_info.flow.id.to_owned()),
        step_id: data.flow_info.step_id.to_owned(),
        next_step: Some(data.flow_info.step_id.to_owned()),
    }
}

// pub fn create_node(
//     conversation_id: &str,
//     bot_id: &str,
//     user_id: &str,
//     channel_id: &str,
//     create_node_body: crate::models::CreateNodeBody
// )
pub fn create_node(data: &mut ConversationInfo, node: CreateNodeBody) -> Result<(), Error> {
    data.api_client.nodes_api().create_node(
        &data.conversation_id,
        &data.client.bot_id,
        &data.client.user_id,
        &data.client.channel_id,
        node,
    )
}

// pub fn get_conversation_nodes(
//     conversation_id: &str,
//     bot_id: &str,
//     user_id: &str,
//     channel_id: &str
// )

// pub fn get_conversation_nodes(
//     core: &mut Core,
//     api_client: &APIClient,
//     conversation_id: &str,
//     client: &Client,
// ) -> Result<Vec<NodeModel>, Error<Value>> {
//     let future = api_client.nodes_api().get_conversation_nodes(
//         conversation_id,
//         &client.bot_id,
//         &client.user_id,
//         &client.channel_id,
//     );

//     core.run(future)
// }
