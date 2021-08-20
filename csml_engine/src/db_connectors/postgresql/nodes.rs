use std::env;

use diesel::{RunQueryDsl, ExpressionMethods, QueryDsl};
use diesel::{insert_into};

use serde_json::Value;

use crate::{
    db_connectors::postgresql::get_db,
    encrypt::encrypt_data, EngineError, PostgresqlClient, ConversationInfo
};

use super::{
    models::{NewNode, Node},
    schema::nodes
};


pub fn create_node(
    data: &mut ConversationInfo,
    nextflow: Option<String>,
    nextstep: Option<String>,
) -> Result<(), EngineError> {

    let db = get_db(&data.db)?;


    let next_flow = match nextflow {
        Some(ref nextflow) => Some(nextflow.as_str()),
        None => None,
    };

    let next_step = match nextstep {
        Some(ref nextstep) => Some(nextstep.as_str()),
        None => None,
    };

    let new_node = NewNode {
        client_id: 42, // client_id
        interaction_id: 42, // &data.interaction_id,
        conversation_id: 42, // &data.conversation_id,
        flow_id: &data.context.flow,
        step_id: &data.context.step,
        next_flow,
        next_step,
    };

    let instruction: Node = diesel::insert_into(nodes::table)
    .values(&new_node)
    .get_result(&db.client)
    .expect("Error creating node");

    Ok(())
}

pub fn delete_conversation_nodes(
    // client: &Client,
    client_id: i32,
    db: &PostgresqlClient
) -> Result<(), EngineError> {

    diesel::delete(nodes::table.filter(
        nodes::client_id.eq(client_id))
    ).get_result::<Node>(&db.client);

    Ok(())
}


