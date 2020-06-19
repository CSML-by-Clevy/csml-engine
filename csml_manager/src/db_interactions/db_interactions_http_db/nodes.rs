use crate::{db_interactions::db_interactions_http_db::get_db, ConversationInfo, ManagerError};
use http_db::models::CreateNodeBody;
use uuid::Uuid;

pub fn new_node(
    data: &mut ConversationInfo,
    nextflow: Option<String>,
    nextstep: Option<String>,
) -> CreateNodeBody {
    CreateNodeBody {
        id: Uuid::new_v4().to_string(),
        interaction_id: data.interaction_id.to_owned(),
        flow_id: data.context.flow.to_owned(),
        next_flow: nextflow,
        step_id: data.context.step.to_owned(),
        next_step: nextstep,
    }
}

pub fn create_node(
    data: &mut ConversationInfo,
    nextflow: Option<String>,
    nextstep: Option<String>,
) -> Result<(), ManagerError> {
    let node = new_node(data, nextflow, nextstep);

    let db = get_db(&data.db)?;

    db.nodes_api().create_node(
        &data.conversation_id,
        &data.client.bot_id,
        &data.client.user_id,
        &data.client.channel_id,
        node,
    )?;

    Ok(())
}
