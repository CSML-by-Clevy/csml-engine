use crate::{db_connectors::mongodb::get_db, Client, ConversationInfo, EngineError, MongoDbClient};
use bson::{doc, Bson};

pub fn create_node(
    data: &mut ConversationInfo,
    nextflow: Option<String>,
    nextstep: Option<String>,
) -> Result<(), EngineError> {
    let time = Bson::DateTime(chrono::Utc::now());

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
    let path = db.client.collection("path");

    path.insert_one(node, None)?;

    Ok(())
}

pub fn delete_conversation_nodes(client: &Client, db: &MongoDbClient) -> Result<(), EngineError> {
    let collection = db.client.collection("path");

    let filter = doc! {
        "client": bson::to_bson(&client)?,
    };

    collection.delete_many(filter, None)?;

    Ok(())
}
