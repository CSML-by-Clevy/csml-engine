use crate::{db_connectors::mongodb::get_db, ConversationInfo, EngineError};
use bson::{doc, Bson};

pub fn create_node(
    data: &mut ConversationInfo,
    nextflow: Option<String>,
    nextstep: Option<String>,
) -> Result<(), EngineError> {
    let time = Bson::UtcDatetime(chrono::Utc::now());

    let nextflow = match nextflow {
        Some(nextflow) => Bson::String(nextflow),
        None => Bson::Null,
    };

    let nextstep = match nextstep {
        Some(nextstep) => Bson::String(nextstep),
        None => Bson::Null,
    };

    let node = doc! {
        "client": bson::to_bson(&data.client)?,
        "interaction_id": &data.interaction_id,
        "conversation_id": &data.conversation_id,
        "flow_id": &data.context.flow,
        "step_id": &data.context.step,
        "next_flow": nextflow,
        "next_step": nextstep,
        "created_at": time
    };

    let db = get_db(&data.db)?;
    let path = db.collection("path");

    path.insert_one(node, None)?;

    Ok(())
}
