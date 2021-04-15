use crate::data::DynamoDbClient;
use crate::db_connectors::dynamodb::{Interaction, DynamoDbKey};
use crate::{encrypt::encrypt_data, Client, EngineError};
use rusoto_dynamodb::*;
use uuid::Uuid;
use std::collections::HashMap;

use crate::db_connectors::dynamodb::utils::*;

pub fn init_interaction(
    event: serde_json::Value,
    client: &Client,
    db: &mut DynamoDbClient,
) -> Result<String, EngineError> {
    let id = Uuid::new_v4();
    let encrypted_event = encrypt_data(&event)?;
    let interaction = Interaction::new(&id, client, &encrypted_event);

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
                s: Some(interaction.hash.to_owned()),
                ..Default::default()
            },
        ),
        (
            String::from(":rangeVal"),
            AttributeValue {
                s: Some(interaction.range.to_owned()),
                ..Default::default()
            },
        ),
    ]
    .iter()
    .cloned()
    .collect();

    let input = PutItemInput {
        table_name: get_table_name()?,
        item: serde_dynamodb::to_hashmap(&interaction)?,
        condition_expression: Some("#hashKey <> :hashVal AND #rangeKey <> :rangeVal".to_owned()),
        expression_attribute_names: Some(expr_attr_names),
        expression_attribute_values: Some(expr_attr_values),
        ..Default::default()
    };

    let future = db.client.put_item(input);
    db.runtime.block_on(future)?;

    Ok(id.to_string())
}

pub fn update_interaction(
    interaction_id: &str,
    success: bool,
    client: &Client,
    db: &mut DynamoDbClient,
) -> Result<(), EngineError> {
    let key = Interaction::get_key(client, interaction_id);

    let expr_attr_names = [
        (String::from("#hashKey"), String::from("hash")),
        (String::from("#rangeKey"), String::from("range")),
        (String::from("#successKey"), String::from("success")),
        (String::from("#updatedAtKey"), String::from("updated_at")),
    ]
    .iter()
    .cloned()
    .collect();

    let expr_attr_values = [
        (
            String::from(":hashVal"),
            AttributeValue {
                s: Some(key.hash.to_owned()),
                ..Default::default()
            },
        ),
        (
            String::from(":rangeVal"),
            AttributeValue {
                s: Some(key.range.to_owned()),
                ..Default::default()
            },
        ),
        (
            String::from(":successVal"),
            AttributeValue {
                bool: Some(success),
                ..Default::default()
            },
        ),
        (
            String::from(":updatedAtVal"),
            AttributeValue {
                s: Some(get_date_time()),
                ..Default::default()
            },
        ),
    ]
    .iter()
    .cloned()
    .collect();

    // make sure that if the item does not already exist, it is NOT created automatically
    let condition_expr = "#hashKey = :hashVal AND #rangeKey = :rangeVal".to_string();
    let update_expr = "SET #updatedAtKey = :updatedAtVal, #successKey = :successVal".to_string();

    let input = UpdateItemInput {
        table_name: get_table_name()?,
        key: serde_dynamodb::to_hashmap(&key)?,
        condition_expression: Some(condition_expr),
        update_expression: Some(update_expr),
        expression_attribute_names: Some(expr_attr_names),
        expression_attribute_values: Some(expr_attr_values),
        ..Default::default()
    };

    let future = db.client.update_item(input);
    db.runtime.block_on(future)?;

    Ok(())
}

fn query_interactions(
    client: &Client,
    db: &mut DynamoDbClient,
    limit: i64,
    pagination_key: Option<HashMap<String, AttributeValue>>,
) -> Result<QueryOutput, EngineError> {
    let hash = Interaction::get_hash(client);

    let expr_attr_names = [
        ("#hashKey".to_string(), "hash".to_string()),
        ("#rangeKey".to_string(), "range".to_string()),
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
                s: Some(String::from("interaction#")),
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

fn get_interactions_batches_to_delete(
    client: &Client,
    db: &mut DynamoDbClient,
) -> Result<Vec<Vec<WriteRequest>>, EngineError> {
    let mut batches = vec![];
    let mut pagination_key = None;

    // retrieve all memories from dynamodb
    loop {
        let data = query_interactions(client, db, 25, pagination_key)?;

        // The query returns an array of items (max 10, based on the limit param above).
        // If 0 item is returned it means that there is no open conversation, so simply return None
        // , "last_key": :
        let items = match data.items {
            None => return Ok(batches),
            Some(items) if items.len() == 0 => return Ok(batches),
            Some(items) => items.clone(),
        };

        let mut write_requests = vec![];

        for item in items {
            let interaction: Interaction = serde_dynamodb::from_hashmap(item.to_owned())?;

            let key = serde_dynamodb::to_hashmap(&DynamoDbKey {
                hash: Interaction::get_hash(client),
                range: Interaction::get_range(&interaction.id),
            })?;

            write_requests.push(WriteRequest {
                delete_request: Some(DeleteRequest {key}),
                put_request: None,
            });
        }

        batches.push(write_requests);

        pagination_key = match data.last_evaluated_key {
            Some(pagination_key) => Some(pagination_key),
            None => return Ok(batches),
        };
    }
}

pub fn delete_user_interactions(client: &Client, db: &mut DynamoDbClient) -> Result<(), EngineError> {
    let batches = get_interactions_batches_to_delete(client, db)?;

    for write_requests in batches {
        let request_items = [(get_table_name()?, write_requests)]
            .iter()
            .cloned()
            .collect();

        let input = BatchWriteItemInput {
            request_items,
            ..Default::default()
        };

        let future = db.client.batch_write_item(input);
        db.runtime.block_on(future)?;
    }
    Ok(())
}