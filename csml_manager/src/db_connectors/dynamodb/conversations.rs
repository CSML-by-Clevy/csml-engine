use crate::{Client, ManagerError, encrypt::{encrypt_data, decrypt_data}};
use crate::db_connectors::DbConversation;
use crate::db_connectors::dynamodb::{Conversation, DynamoDbKey};
use rusoto_dynamodb::*;
use crate::data::DynamoDbClient;
use std::collections::HashMap;

use crate::db_connectors::dynamodb::utils::*;

pub fn create_conversation(
    flow_id: &str,
    step_id: &str,
    client: &Client,
    metadata: serde_json::Value,
    db: &DynamoDbClient,
) -> Result<String, ManagerError> {

    let data = Conversation::new(client, &encrypt_data(&metadata)?, flow_id, step_id);
    let input = PutItemInput {
        item: serde_dynamodb::to_hashmap(&data)?,
        table_name: get_table_name()?,
        ..Default::default()
    };

    let client = db.client.to_owned();
    let future = client.put_item(input);
    let mut runtime = db.get_runtime()?;

    runtime.block_on(future)?;
    Ok(data.id.to_owned())
}

/**
 * Retrieve a conversation then set its status to the requested value.
 * For simplicity's sake, we first retrieve the item then must rewrite it
 * entirely. This is not great but necessary because STATUS is embedded
 * in range key (ideally, we would use a secondary index instead).
 * FIXME: This should be improved on at some point.
 */
pub fn close_conversation(
    id: &str,
    client: &Client,
    status: &str,
    db: &DynamoDbClient,
) -> Result<(), ManagerError> {

    let hash = Conversation::get_hash(client);
    let range = Conversation::get_range(id);
    let key = DynamoDbKey::new(&hash, &range);

    // get conv with ID
    let get_input = GetItemInput {
        table_name: get_table_name()?,
        key: serde_dynamodb::to_hashmap(&key)?,
        ..Default::default()
    };

    let future = db.client.get_item(get_input);
    let mut runtime = db.get_runtime()?;
    let res = runtime.block_on(future)?;

    if let None = res.item {
        return Ok(());
    }
    let item = match res.item {
        None => return Ok(()),
        Some(data) => data,
    };

    let new_conv: Conversation = serde_dynamodb::from_hashmap(item)?;
    let metadata = serde_json::from_str(&new_conv.metadata)?;

    let new_conv = DbConversation {
        id: new_conv.id.to_string(),
        client: client.to_owned(),
        flow_id: new_conv.flow_id.to_string(),
        step_id: new_conv.step_id.to_string(),
        metadata: decrypt_data(metadata)?,
        status: new_conv.status.to_string(),
        last_interaction_at: new_conv.last_interaction_at.to_string(),
        updated_at: new_conv.updated_at.to_string(),
        created_at: new_conv.created_at.to_string(),
    };

    let mut new_conv = Conversation::from(&new_conv);
    let now = get_date_time();
    new_conv.status = status.to_owned();
    new_conv.last_interaction_at = now.to_owned();
    new_conv.updated_at = now.to_owned();
    new_conv.range_time = make_range(&["interaction", status, &now, &id]);
    let key = Conversation::get_key(&client, &id);

    let new_conv = serde_dynamodb::to_hashmap(&new_conv)?;

    replace_conversation(&key, new_conv, db)?;

    Ok(())
}

/**
 * There should not be many open conversations for any given client.
 * In a normal scenario, there should be either 1, or none. If for some reason,
 * there is more than one, there should not be many.
 * For this reason it should be ok to just get them all one by one like this.
 */
fn get_all_open_conversations(client: &Client, db: &DynamoDbClient) -> Vec<DbConversation> {
    let mut res = vec![];

    while let Some(conv) = match get_latest_open(client, db) {
        Ok(val) => val,
        _ => None,
    } {
        res.push(conv);
    }

    res
}

/**
 * To close a conversation, we must replace the given conversation,
 * ideally in a transaction to make sure that we don't lose a conversation in the process.
 */
fn replace_conversation(
    key: &DynamoDbKey,
    new_conversation: HashMap<String, AttributeValue>,
    db: &DynamoDbClient
) -> Result<(), ManagerError> {

    let to_remove = TransactWriteItem {
        put: Some(Put {
            table_name: get_table_name()?,
            item: new_conversation,
            ..Default::default()
        }),
        ..Default::default()
    };
    let to_insert = TransactWriteItem  {
        delete: Some(Delete {
            table_name: get_table_name()?,
            key: serde_dynamodb::to_hashmap(key.to_owned())?,
            ..Default::default()
        }),
        ..Default::default()
    };

    let mut runtime = db.get_runtime()?;
    let input = TransactWriteItemsInput {
        transact_items: vec![to_remove, to_insert],
        ..Default::default()
    };

    let future = db.client.transact_write_items(input);
    runtime.block_on(future)?;

    Ok(())
}

pub fn close_all_conversations(
    client: &Client,
    db: &DynamoDbClient,
) -> Result<(), ManagerError> {

    let ids = get_all_open_conversations(client, db);
    for conv in ids.iter() {
        let mut new_conv = Conversation::from(conv);
        let now = get_date_time();
        new_conv.status = "CLOSED".to_owned();
        new_conv.last_interaction_at = now.to_owned();
        new_conv.updated_at = now.to_owned();
        new_conv.range_time = make_range(&["interaction", "CLOSED", &now, &conv.id]);
        let key = Conversation::get_key(&conv.client, &conv.id);

        let new_conv = serde_dynamodb::to_hashmap(&new_conv)?;
        replace_conversation(&key, new_conv, db)?;
    }
    Ok(())
}

pub fn get_latest_open(
    client: &Client,
    db: &DynamoDbClient,
) -> Result<Option<DbConversation>, ManagerError> {

    let hash = Conversation::get_hash(client);

    let key_cond_expr = "#hashKey = :hashVal AND begins_with(#rangeKey, :rangePrefix)".to_string();
    let expr_attr_names = [
        (String::from("#hashKey"), String::from("hash")),
        (String::from("#rangeKey"), String::from("range_time")), // time index
    ].iter().cloned().collect();

    let expr_attr_values = [
        (String::from(":hashVal"), AttributeValue { s: Some(hash.to_string()), ..Default::default() }),
        (String::from(":rangePrefix"), AttributeValue { s: Some(String::from("conversation#OPEN")), ..Default::default() }),
    ].iter().cloned().collect();

    let input = QueryInput {
        table_name: get_table_name()?,
        index_name: Some(String::from("TimeIndex")),
        key_condition_expression: Some(key_cond_expr),
        expression_attribute_names: Some(expr_attr_names),
        expression_attribute_values: Some(expr_attr_values),
        limit: Some(1),
        select: Some(String::from("ALL_ATTRIBUTES")),
        ..Default::default()
    };

    let mut runtime = db.get_runtime()?;
    let query = db.client.query(input);
    let data = runtime.block_on(query)?;

    // The query returns an array of items (max 1, based on the limit param above).
    // If 0 item is returned it means that there is no open conversation, so simply return None
    let item = match data.items {
        None => return Ok(None),
        Some(items) if items.len() == 0 => return Ok(None),
        Some(items) => items[0].clone(),
    };

    let conv: Conversation = serde_dynamodb::from_hashmap(item)?;

    Ok(Some(DbConversation {
        id: conv.id.to_string(),
        client: client.to_owned(),
        flow_id: conv.flow_id.to_string(),
        step_id: conv.step_id.to_string(),
        metadata: decrypt_data(conv.metadata)?,
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
    db: &DynamoDbClient,
) -> Result<(), ManagerError> {

    let hash = Conversation::get_hash(client);
    let range = Conversation::get_range(conversation_id);

    // make sure that if the item does not already exist, it is NOT created automatically
    let condition_expr = "#hashKey = :hashVal AND #rangeKey = :rangeVal".to_string();
    let expr_attr_names = [
        ("#hashKey".to_string(), "hash".to_string()),
        ("#rangeKey".to_string(), "range".to_string())
    ].iter().cloned().collect();
    let mut expr_attr_values: HashMap<String, AttributeValue> = [
        (
            String::from(":hashVal"), AttributeValue { s: Some(hash.to_string()), ..Default::default() },
        ),
        (
            String::from(":rangeVal"), AttributeValue { s: Some(range.to_string()), ..Default::default() },
        ),
    ].iter().cloned().collect();

    let mut update_expr = "SET last_interaction_at = :lastInteractionAtVal".to_string();

    // all items get the last_interaction_at date updated
    let now = get_date_time();
    expr_attr_values.insert(
        ":lastInteractionAtVal".to_string(),
        AttributeValue { s: Some(now), ..Default::default() },
    );

    // only update the flow_id if there is a need for that
    if let Some(flow_id) = flow_id {
        update_expr = format!("{}, flow_id = :flowIdVal", update_expr);
        expr_attr_values.insert(
            ":flowIdVal".to_string(),
            AttributeValue { s: Some(flow_id), ..Default::default() },
        );
    }

    // only update the step_id if there is a need for that
    if let Some(step_id) = step_id {
        update_expr = format!("{}, step_id = :stepIdVal", update_expr);
        expr_attr_values.insert(
            ":stepIdVal".to_string(),
            AttributeValue { s: Some(step_id), ..Default::default() },
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
    let mut runtime = db.get_runtime()?;
    runtime.block_on(future)?;

    Ok(())
}
