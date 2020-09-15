use crate::{ConversationInfo, ManagerError};
use crate::db_connectors::dynamodb::{Node, get_db};
use rusoto_dynamodb::*;

use crate::db_connectors::dynamodb::utils::*;

pub fn create_node(
    data: &mut ConversationInfo,
    nextflow: Option<String>,
    nextstep: Option<String>,
) -> Result<(), ManagerError> {
    let node = Node::new(
        &data.client,
        &data.conversation_id,
        &data.interaction_id,
        &data.context.flow,
        &data.context.step,
        nextflow,
        nextstep
    );

    let item = to_attribute_value_map(&node)?;

    let expr_attr_names = [
        (String::from("#hashKey"), String::from("hash")),
        (String::from("#rangeKey"), String::from("range_time"))
    ].iter().cloned().collect();

    let expr_attr_values = [
        (String::from(":hashVal"), AttributeValue { s: Some(node.hash.to_owned()), ..Default::default() }),
        (String::from(":rangeVal"), AttributeValue { s: Some(node.range.to_owned()), ..Default::default() }),
    ].iter().cloned().collect();

    let input = PutItemInput {
        item,
        condition_expression: Some("#hashKey <> :hashVal AND #rangeKey <> :rangeVal".to_owned()),
        expression_attribute_names: Some(expr_attr_names),
        expression_attribute_values: Some(expr_attr_values),
    ..Default::default()
    };

    let db = get_db(&data.db)?;
    let mut runtime = db.get_runtime()?;

    let future = db.client.put_item(input);
    runtime.block_on(future)?;

    Ok(())
}
