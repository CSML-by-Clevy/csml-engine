// use crate::{ConversationInfo, Memories};
// use csmlenginems::{
//     apis::{client::APIClient, Error},
//     models::{CreateMemoryBody, MemoryModel},
// };
// use csmlinterpreter::interpreter::data::Client;
// // use std::collections::HashSet;
// use uuid::Uuid;

// pub fn format_memories(
//     data: &mut ConversationInfo,
//     memories: &[Memories],
//     interaction_order: i32,
// ) -> Vec<CreateMemoryBody> {
//     return memories
//         .iter()
//         .enumerate()
//         .map(|(i, var)| {
//             CreateMemoryBody::new(
//                 Uuid::new_v4().to_string(),
//                 data.interaction_id.to_owned(),
//                 i as i32,
//                 interaction_order,
//                 data.flow_info.flow.id.to_owned(),
//                 data.flow_info.step_id.to_owned(),
//                 "serde json".to_owned(),
//                 var.key.clone(),
//                 var.value.clone(),
//                 true,
//                 None,
//             )
//         })
//         .collect::<Vec<_>>();
// }

// // pub fn add_memories_bulk(
// //     conversation_id: &str,
// //     bot_id: &str,
// //     user_id: &str,
// //     channel_id: &str,
// //     create_memory_body: Vec<crate::models::CreateMemoryBody>
// // )
// pub fn add_memories_bulk(
//     data: &mut ConversationInfo,
//     memories: Vec<CreateMemoryBody>,
// ) -> Result<(), Error> {
//     data.api_client.memories_api().add_memories_bulk(
//         &data.conversation_id,
//         &data.context.client.bot_id,
//         &data.context.client.user_id,
//         &data.context.client.channel_id,
//         memories,
//     )
// }

// // pub fn add_memory(
// //     conversation_id: &str,
// //     bot_id: &str,
// //     user_id: &str,
// //     channel_id: &str,
// //      create_memory_body: crate::models::CreateMemoryBody
// // )

// // pub fn add_memory(
// //     core: &mut Core,
// //     api_client: &APIClient,
// //     client: &Client,
// //     conversation_id: &str,
// //     mem:CreateMemoryBody,
// // ) -> Result<(), Error<Value>>  {
// //     let future = api_client.memories_api().add_memory(
// //         conversation_id,
// //         &client.bot_id,
// //         &client.user_id,
// //         &client.channel_id,
// //         mem);

// //     core.run(future)
// // }

// // pub fn get_current_memories(
// //     conversation_id: &str,
// //     bot_id: &str,
// //     user_id: &str,
// //     channel_id: &str
// // )

// pub fn get_current_memories(
//     api_client: &APIClient,
//     client: &Client,
//     conversation_id: &str,
// ) -> Result<Vec<MemoryModel>, Error> {
//     api_client.memories_api().get_current_memories(
//         conversation_id,
//         &client.bot_id,
//         &client.user_id,
//         &client.channel_id,
//     )
// }

// // pub fn get_past_memories(
// //     conversation_id: &str,
// //     bot_id: &str,
// //     user_id: &str,
// //     channel_id: &str
// // )
// pub fn get_past_memories(
//     api_client: &APIClient,
//     client: &Client,
//     conversation_id: &str,
// ) -> Result<Vec<MemoryModel>, Error> {
//     api_client.memories_api().get_past_memories(
//         conversation_id,
//         &client.bot_id,
//         &client.user_id,
//         &client.channel_id,
//     )
// }
