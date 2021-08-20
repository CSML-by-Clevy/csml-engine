use chrono::SecondsFormat;

use diesel::{RunQueryDsl, ExpressionMethods, QueryDsl};
use diesel::{insert_into};

use serde_json::Value;

use crate::{
    db_connectors::postgresql::get_db,
    encrypt::{decrypt_data, encrypt_data},
    EngineError, PostgresqlClient,
    ConversationInfo, Memory, Client
};

use super::{
    models,
    schema::conversations
};

use std::collections::HashMap;
use std::env;

pub fn create_conversation(
    flow_id: &str,
    step_id: &str,
    client: &Client,
    db: &PostgresqlClient,
) -> Result<String, EngineError> {
    let new_conversation = models::NewConversation {
        client_id: 42, // client_id
        flow_id,
        step_id,
        status: "OPEN"
    };

    let conversation: models::Conversation = diesel::insert_into(conversations::table)
    .values(&new_conversation)
    .get_result(&db.client)
    .expect("Error creating memory"); 

    Ok(conversation.id.to_string())
}

pub fn close_conversation(
    id: &str,
    client: &Client,
    status: &str,
    db: &PostgresqlClient,
) -> Result<(), EngineError> {
    diesel::update(
        conversations::table
        .filter(conversations::id.eq(42))
        .filter(conversations::client_id.eq(42))
    )
    .set(conversations::status.eq(status))
    .execute(&db.client);

    Ok(())
}

pub fn close_all_conversations(client: &Client, db: &PostgresqlClient) -> Result<(), EngineError> {
    diesel::update(
        conversations::table
        .filter(conversations::client_id.eq(42))
    )
    .set(conversations::status.eq("CLOSED"))
    .execute(&db.client);

    Ok(())
}

// pub fn get_latest_open(
//     client: &Client,
//     db: &PostgresqlClient,
// ) -> Result<Option<DbConversation>, EngineError> {
//     let conversations: models::Conversation = conversations::table
//         .filter(conversations::client_id.eq(42))
//         .filter(conversations::status.eq("OPEN"))
//         .order_by(conversations::updated_at.asc())
//         // .filter(memories::bot_id.eq("Sean"))
//         // .filter(memories::channel_id.eq("Sean"))
//         // .filter(memories::user_id.eq("Sean"))
//         .limit(1)
//         .load(&db.client)
//         .expect("Error getting memory");

//     // match result {
//     //     Some(conv) => {
//     //         let conversation = format_conversation_struct(conv)?;
//     //         Ok(Some(conversation))
//     //     }
//     //     None => Ok(None),
//     // }

//     Ok(None)

//     unimplemented!()
// }

pub fn update_conversation(
    conversation_id: &str,
    client: &Client,
    flow_id: Option<String>,
    step_id: Option<String>,
    db: &PostgresqlClient,
) -> Result<(), EngineError> {

    match (flow_id, step_id) {
        (Some(flow_id), Some(step_id)) => {
           diesel::update(
                conversations::table
                .filter(conversations::id.eq(42))
                .filter(conversations::client_id.eq(42))
            )
            .set((
                conversations::flow_id.eq(flow_id.as_str()),
                conversations::step_id.eq(step_id.as_str())
            ))
            .execute(&db.client);
        }
        (Some(flow_id), _) => {
            diesel::update(
                conversations::table
                .filter(conversations::id.eq(42))
                .filter(conversations::client_id.eq(42))
            )
            .set(conversations::flow_id.eq(flow_id.as_str()))
            .get_result::<models::Conversation>(&db.client);
        }
        (_, Some(step_id)) => {
            diesel::update(
                conversations::table
                .filter(conversations::id.eq(42))
                .filter(conversations::client_id.eq(42))
            )
            .set(conversations::step_id.eq(step_id.as_str()))
            .get_result::<models::Conversation>(&db.client);
        }
        _ => return Ok(())
    };

    Ok(())
}

pub fn delete_user_conversations(client: &Client, db: &PostgresqlClient) -> Result<(), EngineError> {
    diesel::delete(conversations::table
        .filter(conversations::client_id.eq(42))
    ).execute(&db.client);

    Ok(())
}

// pub fn get_client_conversations(
//     client: &Client,
//     db: &PostgresqlClient,
//     limit: Option<i64>,
//     pagination_key: Option<String>,
// ) -> Result<serde_json::Value, EngineError> {

//     unimplemented!()

    //     let collection = db.client.collection("conversation");

//     let limit = match limit {
//         Some(limit) if limit >= 1 => limit + 1,
//         Some(_limit) => 21,
//         None => 21,
//     };

//     let filter = match pagination_key {
//         Some(key) => {
//             doc! {
//                 "client": bson::to_bson(&client)?,
//                 "_id": {"$gt": bson::oid::ObjectId::with_string(&key).unwrap() }
//             }
//         }
//         None => doc! {"client": bson::to_bson(&client)?},
//     };

//     let find_options = mongodb::options::FindOptions::builder()
//         .sort(doc! { "$natural": -1 })
//         .batch_size(30)
//         .limit(limit)
//         .build();
//     let cursor = collection.find(filter, find_options)?;

//     let mut conversations = vec![];
//     for doc in cursor {
//         match doc {
//             Ok(conv) => {
//                 let conversation = format_conversation_struct(conv)?;

//                 let json = serde_json::json!({
//                     "client": conversation.client,
//                     "flow_id": conversation.flow_id,
//                     "step_id": conversation.step_id,
//                     "status": conversation.status,
//                     "last_interaction_at": conversation.last_interaction_at,
//                     "updated_at": conversation.updated_at,
//                     "created_at": conversation.created_at
//                 });

//                 conversations.push(json);
//             }
//             Err(_) => (),
//         };
//     }

//     match conversations.len() == limit as usize {
//         true => {
//             conversations.pop();
//             match conversations.last() {
//                 Some(last) => {
//                     let pagination_key = base64::encode(last["version_id"].clone().to_string());

//                     Ok(
//                         serde_json::json!({"conversations": conversations, "pagination_key": pagination_key}),
//                     )
//                 }
//                 None => Ok(serde_json::json!({ "conversations": conversations })),
//             }
//         }
//         false => Ok(serde_json::json!({ "conversations": conversations })),
//     }
// }
