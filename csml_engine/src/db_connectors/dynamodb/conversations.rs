use crate::data::DynamoDbClient;
use crate::db_connectors::dynamodb::{Conversation, ConversationKeys, DynamoDbKey};
use crate::db_connectors::DbConversation;
use crate::{Client, EngineError};
use rusoto_dynamodb::*;
use std::collections::HashMap;

use crate::db_connectors::dynamodb::utils::*;

pub fn create_conversation(
    flow_id: &str,
    step_id: &str,
    client: &Client,
    expires_at: Option<i64>,
    db: &mut DynamoDbClient,
) -> Result<String, EngineError> {
    let data = Conversation::new(client, flow_id, step_id, expires_at);
    let input = PutItemInput {
        item: serde_dynamodb::to_hashmap(&data)?,
        table_name: get_table_name()?,
        ..Default::default()
    };

    let client = db.client.to_owned();
    let future = client.put_item(input);

    db.runtime.block_on(future)?;

    Ok(data.id.to_owned())
}

/**
 * Retrieve a conversation then set its status to the requested value.
 * For simplicity's sake, we first retrieve the item then must rewrite it
 * entirely. This is not great but necessary because STATUS is embedded
 * in range key (ideally, we would use a secondary index instead).
 */
pub fn close_conversation(
    id: &str,
    client: &Client,
    status: &str,
    db: &mut DynamoDbClient,
) -> Result<(), EngineError> {
    // retrieve the old conversation, which at this stage must still be open
    let hash = Conversation::get_hash(client);
    let range = Conversation::get_range("OPEN", id);
    let old_key = DynamoDbKey::new(&hash, &range);

    // get conv with ID
    let get_input = GetItemInput {
        table_name: get_table_name()?,
        key: serde_dynamodb::to_hashmap(&old_key)?,
        ..Default::default()
    };

    let future = db.client.get_item(get_input);

    let res = db.runtime.block_on(future)?;

    // If no conversation matches the request, we assume it's already closed and move on
    let item = match res.item {
        None => return Ok(()),
        Some(data) => data,
    };

    // Update the conversation with the new status and closed state
    let mut new_conv: Conversation = serde_dynamodb::from_hashmap(item)?;

    let now = get_date_time();
    new_conv.status = status.to_owned();
    new_conv.last_interaction_at = now.to_owned();
    new_conv.updated_at = now.to_owned();
    new_conv.range_time = make_range(&["interaction", "CLOSED", &now, &id]);
    new_conv.range = Conversation::get_range("CLOSED", &id);

    let new_item = serde_dynamodb::to_hashmap(&new_conv)?;

    replace_conversation(&old_key, new_item, db)?;

    Ok(())
}

/**
 * To close a conversation, we must replace the given conversation,
 * ideally in a transaction to make sure that we don't lose a conversation in the process.
 */
fn replace_conversation(
    old_key: &DynamoDbKey,
    new_item: HashMap<String, AttributeValue>,
    db: &mut DynamoDbClient,
) -> Result<(), EngineError> {
    let put = Put {
        table_name: get_table_name()?,
        item: new_item,
        ..Default::default()
    };

    let del = Delete {
        table_name: get_table_name()?,
        key: serde_dynamodb::to_hashmap(old_key.to_owned())?,
        ..Default::default()
    };

    let to_insert = TransactWriteItem {
        put: Some(put),
        ..Default::default()
    };
    let to_remove = TransactWriteItem {
        delete: Some(del),
        ..Default::default()
    };

    let input = TransactWriteItemsInput {
        transact_items: vec![to_remove, to_insert],
        ..Default::default()
    };

    let future = db.client.transact_write_items(input);
    db.runtime.block_on(future)?;

    Ok(())
}

fn get_all_open_conversations(
    client: &Client,
    db: &mut DynamoDbClient,
) -> Result<Vec<Conversation>, EngineError> {
    let hash = Conversation::get_hash(client);

    let key_cond_expr =
        "#hashKey = :hashVal AND begins_with(#rangeTimeKey, :rangePrefix)".to_string();

    let expr_attr_names: HashMap<String, String> = [
        (String::from("#hashKey"), String::from("hash")),
        (String::from("#rangeKey"), "range".to_owned()),
        (String::from("#rangeTimeKey"), "range_time".to_owned()),
    ]
    .iter()
    .cloned()
    .collect();

    let expr_attr_values = [
        (
            String::from(":hashVal"),
            AttributeValue {
                s: Some(hash.to_string()),
                ..Default::default()
            },
        ),
        (
            String::from(":rangePrefix"),
            AttributeValue {
                s: Some(String::from("conversation#OPEN")),
                ..Default::default()
            },
        ),
    ]
    .iter()
    .cloned()
    .collect();

    // There should not be many open conversations for any given client.
    // In a normal scenario, there should be either 1, or none. If for some reason,
    // there is more than one, there should definitely not be many, and it would lead to all
    // sorts of other issues anyway.
    // For this reason it *should* be safe to limit to 50 max, and assume there are not 51+.
    let limit = Some(50);

    let input = QueryInput {
        table_name: get_table_name()?,
        index_name: Some("TimeIndex".to_owned()),
        key_condition_expression: Some(key_cond_expr),
        expression_attribute_names: Some(expr_attr_names),
        expression_attribute_values: Some(expr_attr_values),
        limit,
        projection_expression: Some("#hashKey, #rangeKey".to_owned()),
        ..Default::default()
    };

    let query = db.client.query(input);
    let data = match db.runtime.block_on(query) {
        Ok(data) => data,
        Err(e) => {
            return Err(EngineError::Manager(format!(
                "get_all_open_conversations {:?}",
                e
            )))
        }
    };

    // The query returns an array of items (max 10, based on the limit param above).
    // If 0 item is returned it means that there is no open conversation, so simply return None
    // , "last_key": :
    let items = match data.items {
        None => return Ok(vec![]),
        Some(items) if items.len() == 0 => return Ok(vec![]),
        Some(items) => items.clone(),
    };

    let mut get_requests = vec![];

    for item in items {
        let conversation: ConversationKeys = serde_dynamodb::from_hashmap(item)?;

        let key = serde_dynamodb::to_hashmap(&DynamoDbKey {
            hash: conversation.hash,
            range: conversation.range,
        })?;

        get_requests.push(key);
    }

    let request_items = [(get_table_name()?, get_requests)]
        .iter()
        .cloned()
        .map(|(name, keys)| {
            let mut attval = KeysAndAttributes::default();

            attval.keys = keys;

            (name, attval)
        })
        .collect();

    let input = BatchGetItemInput {
        request_items,
        ..Default::default()
    };

    execute_conversations_batch_get_query(db, input)
}

pub fn close_all_conversations(
    client: &Client,
    db: &mut DynamoDbClient,
) -> Result<(), EngineError> {
    let status = "CLOSED";
    let now = get_date_time();

    let mut conversations = get_all_open_conversations(client, db)?;
    for new_conv in conversations.iter_mut() {
        new_conv.status = status.to_owned();
        new_conv.last_interaction_at = now.to_owned();
        new_conv.updated_at = now.to_owned();
        new_conv.range_time = make_range(&["interaction", status, &now, &new_conv.id]);
        new_conv.range = Conversation::get_range(status, &new_conv.id);

        let old_key = Conversation::get_key(&client, "OPEN", &new_conv.id);
        let new_item = serde_dynamodb::to_hashmap(&new_conv)?;

        replace_conversation(&old_key, new_item, db)?;
    }
    Ok(())
}

pub fn get_latest_open(
    client: &Client,
    db: &mut DynamoDbClient,
) -> Result<Option<DbConversation>, EngineError> {
    let hash = Conversation::get_hash(client);

    let key_cond_expr = "#hashKey = :hashVal AND begins_with(#rangeKey, :rangePrefix)".to_string();
    let expr_attr_names = [
        (String::from("#hashKey"), String::from("hash")),
        (String::from("#rangeKey"), String::from("range_time")), // time index
    ]
    .iter()
    .cloned()
    .collect();

    let expr_attr_values = [
        (
            String::from(":hashVal"),
            AttributeValue {
                s: Some(hash.to_string()),
                ..Default::default()
            },
        ),
        (
            String::from(":rangePrefix"),
            AttributeValue {
                s: Some(String::from("conversation#OPEN")),
                ..Default::default()
            },
        ),
    ]
    .iter()
    .cloned()
    .collect();

    let input = QueryInput {
        table_name: get_table_name()?,
        index_name: Some(String::from("TimeIndex")),
        key_condition_expression: Some(key_cond_expr),
        expression_attribute_names: Some(expr_attr_names),
        expression_attribute_values: Some(expr_attr_values),
        limit: Some(1),
        // select: Some(String::from("SPECIFIC_ATTRIBUTES")),
        ..Default::default()
    };

    let query = db.client.query(input);
    let data = match db.runtime.block_on(query) {
        Ok(data) => data,
        Err(e) => return Err(EngineError::Manager(format!("get_latest_open {:?}", e))),
    };

    // The query returns an array of items (max 1, based on the limit param above).
    // If 0 item is returned it means that there is no open conversation, so simply return None
    let item = match data.items {
        None => return Ok(None),
        Some(items) if items.len() == 0 => return Ok(None),
        Some(items) => items[0].clone(),
    };

    let conversation: ConversationKeys = serde_dynamodb::from_hashmap(item)?;

    let key = serde_dynamodb::to_hashmap(&DynamoDbKey {
        hash: conversation.hash,
        range: conversation.range,
    })?;

    let input = GetItemInput {
        table_name: get_table_name()?,
        key,
        ..Default::default()
    };

    let conv = execute_conversation_get_query(db, input)?;

    Ok(Some(DbConversation {
        id: conv.id.to_string(),
        client: client.to_owned(),
        flow_id: conv.flow_id.to_string(),
        step_id: conv.step_id.to_string(),
        status: conv.status.to_string(),
        last_interaction_at: conv.last_interaction_at.to_string(),
        updated_at: conv.updated_at.to_string(),
        created_at: conv.created_at.to_string(),
    }))
}

pub fn update_conversation(
    conversation_id: &str,
    client: &Client,
    flow_id: Option<String>,
    step_id: Option<String>,
    db: &mut DynamoDbClient,
) -> Result<(), EngineError> {
    let hash = Conversation::get_hash(client);
    let range = Conversation::get_range("OPEN", conversation_id);

    // make sure that if the item does not already exist, it is NOT created automatically
    let condition_expr = "#hashKey = :hashVal AND #rangeKey = :rangeVal".to_string();
    let expr_attr_names: HashMap<String, String> = [
        ("#hashKey".to_string(), "hash".to_string()),
        ("#rangeKey".to_string(), "range".to_string()),
    ]
    .iter()
    .cloned()
    .collect();
    let mut expr_attr_values: HashMap<String, AttributeValue> = [
        (
            String::from(":hashVal"),
            AttributeValue {
                s: Some(hash.to_string()),
                ..Default::default()
            },
        ),
        (
            String::from(":rangeVal"),
            AttributeValue {
                s: Some(range.to_string()),
                ..Default::default()
            },
        ),
    ]
    .iter()
    .cloned()
    .collect();

    let mut update_expr = "SET last_interaction_at = :lastInteractionAtVal".to_string();

    // all items get the last_interaction_at date updated
    let now = get_date_time();
    expr_attr_values.insert(
        ":lastInteractionAtVal".to_string(),
        AttributeValue {
            s: Some(now),
            ..Default::default()
        },
    );

    // only update the flow_id if there is a need for that
    if let Some(flow_id) = flow_id {
        update_expr = format!("{}, flow_id = :flowIdVal", update_expr);
        expr_attr_values.insert(
            ":flowIdVal".to_string(),
            AttributeValue {
                s: Some(flow_id),
                ..Default::default()
            },
        );
    }

    // only update the step_id if there is a need for that
    if let Some(step_id) = step_id {
        update_expr = format!("{}, step_id = :stepIdVal", update_expr);
        expr_attr_values.insert(
            ":stepIdVal".to_string(),
            AttributeValue {
                s: Some(step_id),
                ..Default::default()
            },
        );
    }

    let input = UpdateItemInput {
        table_name: get_table_name()?,
        key: serde_dynamodb::to_hashmap(&DynamoDbKey::new(&hash, &range))?,
        condition_expression: Some(condition_expr),
        update_expression: Some(update_expr.to_string()),
        expression_attribute_names: Some(expr_attr_names),
        expression_attribute_values: Some(expr_attr_values),
        ..Default::default()
    };

    let future = db.client.update_item(input);
    match db.runtime.block_on(future) {
        Ok(_) => Ok(()),
        Err(e) => return Err(EngineError::Manager(format!("update_conversation {:?}", e))),
    }
}

fn query_conversation(
    client: &Client,
    db: &mut DynamoDbClient,
    limit: i64,
    pagination_key: Option<HashMap<String, AttributeValue>>,
    projection_expression: Option<String>,
    key_condition_expression: Option<String>,
    expression_attribute_names: Option<HashMap<String, String>>,
) -> Result<QueryOutput, EngineError> {
    let hash = Conversation::get_hash(client);

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
                s: Some(String::from("conversation#")),
                ..Default::default()
            },
        ),
    ]
    .iter()
    .cloned()
    .collect();

    let input = QueryInput {
        table_name: get_table_name()?,
        index_name: Some("TimeIndex".to_owned()),
        key_condition_expression,
        expression_attribute_names,
        expression_attribute_values: Some(expr_attr_values),
        limit: Some(limit),
        exclusive_start_key: pagination_key,
        scan_index_forward: Some(false),
        projection_expression,
        select: Some(String::from("SPECIFIC_ATTRIBUTES")),
        ..Default::default()
    };

    let future = db.client.query(input);
    let data = match db.runtime.block_on(future) {
        Ok(data) => data,
        Err(e) => return Err(EngineError::Manager(format!("query_conversation {:?}", e))),
    };

    Ok(data)
}

pub fn delete_user_conversations(
    client: &Client,
    db: &mut DynamoDbClient,
) -> Result<(), EngineError> {
    let mut pagination_key = None;

    let key_condition_expression =
        "#hashKey = :hashVal AND begins_with(#rangeTimeKey, :rangePrefix)".to_owned();

    let expr_attr_names: HashMap<String, String> = [
        ("#hashKey".to_string(), "hash".to_string()),
        ("#rangeKey".to_string(), "range".to_string()),
        ("#rangeTimeKey".to_string(), "range_time".to_string()),
    ]
    .iter()
    .cloned()
    .collect();

    // retrieve all memories from dynamodb
    loop {
        let data = query_conversation(
            client,
            db,
            25,
            pagination_key,
            Some("#hashKey, #rangeKey".to_owned()),
            Some(key_condition_expression.clone()),
            Some(expr_attr_names.clone()),
        )?;

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
            let conversation: ConversationKeys = serde_dynamodb::from_hashmap(item.to_owned())?;

            let key = serde_dynamodb::to_hashmap(&DynamoDbKey {
                hash: conversation.hash,
                range: conversation.range,
            })?;

            write_requests.push(WriteRequest {
                delete_request: Some(DeleteRequest { key }),
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
            return Ok(());
        }
    }
}

pub fn get_client_conversations(
    client: &Client,
    db: &mut DynamoDbClient,
    limit: Option<i64>,
    pagination_key: Option<HashMap<String, AttributeValue>>,
) -> Result<serde_json::Value, EngineError> {
    let mut conversations = vec![];
    let limit = match limit {
        Some(limit) if limit >= 1 => limit,
        Some(_limit) => 20,
        None => 20,
    };

    let key_condition_expression =
        "#hashKey = :hashVal AND begins_with(#rangeTimeKey, :rangePrefix)".to_owned();

    let expr_attr_names: HashMap<String, String> = [
        ("#hashKey".to_string(), "hash".to_string()),
        ("#rangeKey".to_string(), "range".to_string()),
        ("#rangeTimeKey".to_string(), "range_time".to_string()),
    ]
    .iter()
    .cloned()
    .collect();

    let data = query_conversation(
        client,
        db,
        limit,
        pagination_key,
        Some("#hashKey, #rangeKey".to_owned()),
        Some(key_condition_expression),
        Some(expr_attr_names.clone()),
    )?;

    // The query returns an array of items (max 10, based on the limit param above).
    // If 0 item is returned it means that there is no open conversation, so simply return None
    // , "last_key": :
    let items = match data.items {
        None => return Ok(serde_json::json!({"conversations": []})),
        Some(items) if items.len() == 0 => return Ok(serde_json::json!({"conversations": []})),
        Some(items) => items.clone(),
    };

    let mut get_requests = vec![];

    for item in items {
        let conversation: ConversationKeys = serde_dynamodb::from_hashmap(item)?;

        let key = serde_dynamodb::to_hashmap(&DynamoDbKey {
            hash: conversation.hash,
            range: conversation.range,
        })?;

        get_requests.push(key);
    }

    let request_items = [(get_table_name()?, get_requests)]
        .iter()
        .cloned()
        .map(|(name, keys)| {
            let mut attval = KeysAndAttributes::default();

            attval.keys = keys;

            (name, attval)
        })
        .collect();

    let input = BatchGetItemInput {
        request_items,
        ..Default::default()
    };

    let get_conversations = execute_conversations_batch_get_query(db, input)?;

    for conversation in get_conversations {
        conversations.push(DbConversation {
            id: conversation.id.to_string(),
            client: client.to_owned(),
            flow_id: conversation.flow_id.to_string(),
            step_id: conversation.step_id.to_string(),
            status: conversation.status.to_string(),
            last_interaction_at: conversation.last_interaction_at.to_string(),
            updated_at: conversation.updated_at.to_string(),
            created_at: conversation.created_at.to_string(),
        })
    }

    match data.last_evaluated_key {
        Some(pagination_key) => {
            let pagination_key = base64::engine::general_purpose::STANDARD.encode(serde_json::json!(pagination_key).to_string());

            Ok(
                serde_json::json!({"conversations": conversations, "pagination_key": pagination_key}),
            )
        }
        None => Ok(serde_json::json!({ "conversations": conversations })),
    }
}
