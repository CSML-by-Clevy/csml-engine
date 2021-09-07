use diesel::{RunQueryDsl};

use crate::{
    db_connectors::postgresql::get_db,
    EngineError, ConversationInfo
};

use super::{
    models::{NewNode, Node},
    schema::csml_nodes
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

    let interaction_id = uuid::Uuid::parse_str(&data.interaction_id).unwrap();
    let conversation_id = uuid::Uuid::parse_str(&data.conversation_id).unwrap();

    let new_node = NewNode {
        id: uuid::Uuid::new_v4(),

        interaction_id: &interaction_id,
        conversation_id: &conversation_id, 
        flow_id: &data.context.flow,
        step_id: &data.context.step,
        next_flow,
        next_step,
    };

    diesel::insert_into(csml_nodes::table)
    .values(&new_node)
    .get_result::<Node>(&db.client)?;

    Ok(())
}
