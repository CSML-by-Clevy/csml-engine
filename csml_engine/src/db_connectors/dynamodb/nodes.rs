use crate::db_connectors::dynamodb::{get_db, Node, DynamoDbKey, DynamoDbClient};
use crate::{ConversationInfo, EngineError};
use rusoto_dynamodb::*;
use std::collections::HashMap;

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

fn query_nodes(
    conversation_id: &str,
    db: &mut DynamoDbClient,
    limit: i64,
    pagination_key: Option<HashMap<String, AttributeValue>>,
) -> Result<QueryOutput, EngineError> {
    let hash = format!("conversation#{}", conversation_id);

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
                s: Some(hash),
                ..Default::default()
            },
        ),
        (
            String::from(":rangePrefix"),
            AttributeValue {
                s: Some(String::from("path#")),
                ..Default::default()
            },
        ),
    ]
    .iter()
    .cloned()
    .collect();

    let input = QueryInput {
        table_name: get_table_name()?,
        key_condition_expression: Some(
            "#hashKey = :hashVal AND begins_with(#rangeKey, :rangePrefix)".to_owned(),
        ),
        expression_attribute_names: Some(expr_attr_names),
        expression_attribute_values: Some(expr_attr_values),
        limit: Some(limit),
        exclusive_start_key: pagination_key,
        scan_index_forward: Some(false),
        select: Some(String::from("ALL_ATTRIBUTES")),
        ..Default::default()
    };

    let future = db.client.query(input);
    let data = db.runtime.block_on(future)?;

    Ok(data)
}

pub fn delete_conversation_nodes(conversation_id: &str, db: &mut DynamoDbClient) -> Result<(), EngineError> {
    let mut pagination_key = None;

    // retrieve all memories from dynamodb
    loop {
        let data = query_nodes(conversation_id, db, 25, pagination_key)?;

        // The query returns an array of items (max 10, based on the limit param above).
        // If 0 item is returned it means that there is no open conversation, so simply return None
        // , "last_key": :
        let items = match data.items {
            None => return Ok(()),
            Some(items) if items.len() == 0 => return Ok(()),
            Some(items) => items.clone(),
        };

        let mut write_requests = vec![];

        for item in items {
            let node: Node = serde_dynamodb::from_hashmap(item.to_owned())?;

            let key = serde_dynamodb::to_hashmap(&DynamoDbKey {
                hash: format!("conversation#{}", node.conversation_id),
                range: format!("path#{}", &node.id),
            })?;

            write_requests.push(WriteRequest {
                delete_request: Some(DeleteRequest {key}),
                put_request: None,
            });
        }

        let request_items = [(get_table_name()?, write_requests)]
        .iter()
        .cloned()
        .collect();

        let input = BatchWriteItemInput {
            request_items,
            ..Default::default()
        };

        execute_batch_write_query(db, input)?;

        pagination_key = data.last_evaluated_key;
        if let None = &pagination_key {
            return Ok(())
        }
    }
}
