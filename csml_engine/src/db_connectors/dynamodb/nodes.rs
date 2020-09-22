use crate::db_connectors::dynamodb::{get_db, Node};
use crate::{ConversationInfo, EngineError};
use rusoto_dynamodb::*;

use crate::db_connectors::dynamodb::utils::*;

pub fn create_node(
    data: &mut ConversationInfo,
    nextflow: Option<String>,
    nextstep: Option<String>,
) -> Result<(), EngineError> {
    let node = Node::new(
        &data.client,
        &data.conversation_id,
        &data.interaction_id,
        &data.context.flow,
        &data.context.step,
        nextflow,
        nextstep,
    );

    let item = serde_dynamodb::to_hashmap(&node)?;

    let expr_attr_names = [
        (String::from("#hashKey"), String::from("hash")),
        (String::from("#rangeKey"), String::from("range")),
    ]
    .iter()
    .cloned()
    .collect();

    let expr_attr_values = [
        (
            String::from(":hashVal"),
            AttributeValue {
                s: Some(node.hash.to_owned()),
                ..Default::default()
            },
        ),
        (
            String::from(":rangeVal"),
            AttributeValue {
                s: Some(node.range.to_owned()),
                ..Default::default()
            },
        ),
    ]
    .iter()
    .cloned()
    .collect();

    let input = PutItemInput {
        table_name: get_table_name()?,
        item,
        condition_expression: Some("#hashKey <> :hashVal AND #rangeKey <> :rangeVal".to_owned()),
        expression_attribute_names: Some(expr_attr_names),
        expression_attribute_values: Some(expr_attr_values),
        ..Default::default()
    };

    let db = get_db(&mut data.db)?;
    let future = db.client.put_item(input);
    db.runtime.block_on(future)?;

    Ok(())
}
