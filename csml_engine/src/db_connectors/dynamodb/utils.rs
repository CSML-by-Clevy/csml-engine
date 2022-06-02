use crate::db_connectors::dynamodb::{Bot, Conversation, Memory, Message};
use crate::{
    data::{DynamoBot, DynamoBotBincode, DynamoDbClient},
    encrypt::decrypt_data,
    Client, EngineError,
};

use rusoto_core::RusotoError;
use rusoto_dynamodb::{
    BatchGetItemError, BatchGetItemInput, BatchWriteItemError, BatchWriteItemInput, DynamoDb,
    GetItemError, GetItemInput,
};
use std::{thread, time};

use rand::Rng;

// The maximum back off time in milliseconds (0.5 seconds).
const RETRY_BASE: u64 = 500;
// The maximum back off time in milliseconds (1 minute).
const MAX_INTERVAL_LIMIT: u64 = 60_000;
// The default maximum elapsed time in milliseconds (10 minutes).
const MAX_ELAPSED_TIME_MILLIS: u64 = 600_000;

/**
 * Return the current datetime formatted as YYYY-MM-DDTHH:mm:ss.SSS[Z].
 * For example: 2020-03-12T12:33:42.123Z
 */
pub fn get_date_time() -> String {
    return chrono::Utc::now()
        .format("%Y-%m-%dT%H:%M:%S.%3fZ")
        .to_string();
}

/**
 * Return the table's name
 */
pub fn get_table_name() -> Result<String, EngineError> {
    match std::env::var("AWS_DYNAMODB_TABLE") {
        Ok(val) => return Ok(val),
        _ => {
            return Err(EngineError::Manager(
                "Missing AWS_DYNAMODB_TABLE env var".to_owned(),
            ))
        }
    }
}

/**
 * Create a hash key from the client info
 */
pub fn make_hash(client: &Client) -> String {
    format!(
        "bot_id:{}#channel_id:{}#user_id:{}",
        client.bot_id, client.channel_id, client.user_id
    )
}

/**
 * Create a serialized range key from given arguments
 */
pub fn make_range(args: &[&str]) -> String {
    let mut res = "".to_owned();
    for arg in args.iter() {
        if res.len() > 0 {
            res = res + "#";
        }
        res = res + arg.to_owned();
    }
    res.to_owned()
}

/**
 * Batch write query wrapper with exponential backoff in case of exceeded throughput
 */
pub fn execute_batch_write_query(
    db: &mut DynamoDbClient,
    input: BatchWriteItemInput,
) -> Result<(), RusotoError<BatchWriteItemError>> {
    let mut retry_times = 1;

    let mut rng = rand::thread_rng();
    let now = time::Instant::now();
    loop {
        match db
            .runtime
            .block_on(db.client.batch_write_item(input.clone()))
        {
            Ok(_) => return Ok(()),
            // request rate is too high, reduce the frequency of requests and use exponential backoff. "https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Programming.Errors.html#Programming.Errors.RetryAndBackoff"
            Err(RusotoError::Service(BatchWriteItemError::ProvisionedThroughputExceeded(err))) => {
                let interval = std::cmp::min(MAX_INTERVAL_LIMIT, RETRY_BASE * 2 * retry_times);
                let interval_jitter = rng.gen_range(0..interval);
                let duration = time::Duration::from_millis(interval_jitter);

                thread::sleep(duration);

                if now.elapsed() >= time::Duration::from_millis(MAX_ELAPSED_TIME_MILLIS) {
                    // if time elapsed reach the MAX_ELAPSED_TIME_MILLIS return error
                    return Err(RusotoError::Service(
                        BatchWriteItemError::ProvisionedThroughputExceeded(err),
                    ));
                }
            }
            Err(err) => return Err(err),
        }
        retry_times += 1;
    }
}

/**
 * Batch get query wrapper with exponential backoff in case of exceeded throughput
 */
pub fn execute_bot_version_batch_get_query(
    db: &mut DynamoDbClient,
    input: BatchGetItemInput,
) -> Result<Vec<serde_json::Value>, EngineError> {
    let mut retry_times = 1;

    let mut rng = rand::thread_rng();
    let now = time::Instant::now();
    loop {
        match db.runtime.block_on(db.client.batch_get_item(input.clone())) {
            Ok(output) => {
                let items = match output.responses {
                    None => return Ok(vec![]),
                    Some(items) if items.len() == 0 => return Ok(vec![]),
                    Some(items) => items.clone(),
                };
                let mut bots = vec![];

                for (_, item) in items {
                    for item in item {
                        let data: Bot = serde_dynamodb::from_hashmap(item.to_owned())?;

                        let csml_bot: DynamoBot = match base64::decode(&data.bot) {
                            Ok(base64decoded) => {
                                match bincode::deserialize::<DynamoBotBincode>(&base64decoded[..]) {
                                    Ok(bot) => bot.to_bot(),
                                    Err(_) => serde_json::from_str(&data.bot).unwrap(),
                                }
                            }
                            Err(_) => serde_json::from_str(&data.bot).unwrap(),
                        };

                        let mut json = serde_json::json!({
                            "version_id": data.version_id,
                            "id": data.id,
                            "name": csml_bot.name,
                            "default_flow": csml_bot.default_flow,
                            "engine_version": data.engine_version,
                            "created_at": data.created_at
                        });

                        if let Some(custom_components) = csml_bot.custom_components {
                            json["custom_components"] = serde_json::json!(custom_components);
                        }

                        bots.push(json);
                    }
                }

                return Ok(bots);
            }
            // request rate is too high, reduce the frequency of requests and use exponential backoff. "https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Programming.Errors.html#Programming.Errors.RetryAndBackoff"
            Err(RusotoError::Service(BatchGetItemError::ProvisionedThroughputExceeded(err))) => {
                let interval = std::cmp::min(MAX_INTERVAL_LIMIT, RETRY_BASE * 2 * retry_times);
                let interval_jitter = rng.gen_range(0..interval);
                let duration = time::Duration::from_millis(interval_jitter);

                thread::sleep(duration);

                if now.elapsed() >= time::Duration::from_millis(MAX_ELAPSED_TIME_MILLIS) {
                    // if time elapsed reach the MAX_ELAPSED_TIME_MILLIS return error
                    return Err(RusotoError::Service(
                        BatchGetItemError::ProvisionedThroughputExceeded(err),
                    )
                    .into());
                }
            }
            Err(err) => return Err(err.into()),
        }
        retry_times += 1;
    }
}

/**
 * Batch get query wrapper with exponential backoff in case of exceeded throughput
 */
pub fn execute_messages_batch_get_query(
    db: &mut DynamoDbClient,
    input: BatchGetItemInput,
) -> Result<Vec<serde_json::Value>, EngineError> {
    let mut retry_times = 1;

    let mut rng = rand::thread_rng();
    let now = time::Instant::now();
    loop {
        match db.runtime.block_on(db.client.batch_get_item(input.clone())) {
            Ok(output) => {
                let items = match output.responses {
                    None => return Ok(vec![]),
                    Some(items) if items.len() == 0 => return Ok(vec![]),
                    Some(items) => items.clone(),
                };
                let mut messages = vec![];

                for (_, item) in items {
                    for item in item {
                        let message: Message = serde_dynamodb::from_hashmap(item)?;

                        let json = serde_json::json!({
                            "client": message.client,
                            "conversation_id": message.conversation_id,
                            "flow_id": message.flow_id,
                            "step_id": message.step_id,
                            "message_order": message.message_order,
                            "interaction_order": message.interaction_order,
                            "direction": message.direction,
                            "payload": decrypt_data(message.payload)?,
                            "created_at": message.created_at
                        });

                        messages.push(json)
                    }
                }

                return Ok(messages);
            }
            // request rate is too high, reduce the frequency of requests and use exponential backoff. "https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Programming.Errors.html#Programming.Errors.RetryAndBackoff"
            Err(RusotoError::Service(BatchGetItemError::ProvisionedThroughputExceeded(err))) => {
                let interval = std::cmp::min(MAX_INTERVAL_LIMIT, RETRY_BASE * 2 * retry_times);
                let interval_jitter = rng.gen_range(0..interval);
                let duration = time::Duration::from_millis(interval_jitter);

                thread::sleep(duration);

                if now.elapsed() >= time::Duration::from_millis(MAX_ELAPSED_TIME_MILLIS) {
                    // if time elapsed reach the MAX_ELAPSED_TIME_MILLIS return error
                    return Err(RusotoError::Service(
                        BatchGetItemError::ProvisionedThroughputExceeded(err),
                    )
                    .into());
                }
            }
            Err(err) => return Err(err.into()),
        }
        retry_times += 1;
    }
}

/**
 * Batch get query wrapper with exponential backoff in case of exceeded throughput
 */
pub fn execute_memory_batch_get_query(
    db: &mut DynamoDbClient,
    input: BatchGetItemInput,
) -> Result<Vec<serde_json::Value>, EngineError> {
    let mut retry_times = 1;

    let mut rng = rand::thread_rng();
    let now = time::Instant::now();
    loop {
        match db.runtime.block_on(db.client.batch_get_item(input.clone())) {
            Ok(output) => {
                let items = match output.responses {
                    None => return Ok(vec![]),
                    Some(items) if items.len() == 0 => return Ok(vec![]),
                    Some(items) => items.clone(),
                };
                let mut memories = vec![];

                for (_, item) in items {
                    for item in item {
                        let memory: Memory = serde_dynamodb::from_hashmap(item)?;

                        let json = serde_json::json!({
                            "key": memory.key,
                            "value": decrypt_data(memory.value.unwrap())?,
                            "created_at": memory.created_at,
                        });

                        memories.push(json)
                    }
                }

                return Ok(memories);
            }
            // request rate is too high, reduce the frequency of requests and use exponential backoff. "https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Programming.Errors.html#Programming.Errors.RetryAndBackoff"
            Err(RusotoError::Service(BatchGetItemError::ProvisionedThroughputExceeded(err))) => {
                let interval = std::cmp::min(MAX_INTERVAL_LIMIT, RETRY_BASE * 2 * retry_times);
                let interval_jitter = rng.gen_range(0..interval);
                let duration = time::Duration::from_millis(interval_jitter);

                thread::sleep(duration);

                if now.elapsed() >= time::Duration::from_millis(MAX_ELAPSED_TIME_MILLIS) {
                    // if time elapsed reach the MAX_ELAPSED_TIME_MILLIS return error
                    return Err(RusotoError::Service(
                        BatchGetItemError::ProvisionedThroughputExceeded(err),
                    )
                    .into());
                }
            }
            Err(err) => return Err(err.into()),
        }
        retry_times += 1;
    }
}

/**
 * Batch get query wrapper with exponential backoff in case of exceeded throughput
 */
pub fn execute_conversations_batch_get_query(
    db: &mut DynamoDbClient,
    input: BatchGetItemInput,
) -> Result<Vec<Conversation>, EngineError> {
    let mut retry_times = 1;

    let mut rng = rand::thread_rng();
    let now = time::Instant::now();
    loop {
        match db.runtime.block_on(db.client.batch_get_item(input.clone())) {
            Ok(output) => {
                let items = match output.responses {
                    None => return Ok(vec![]),
                    Some(items) if items.len() == 0 => return Ok(vec![]),
                    Some(items) => items.clone(),
                };
                let mut conversations = vec![];

                for (_, item) in items {
                    for item in item {
                        let conversation: Conversation = serde_dynamodb::from_hashmap(item)?;

                        conversations.push(conversation)
                    }
                }

                return Ok(conversations);
            }
            // request rate is too high, reduce the frequency of requests and use exponential backoff. "https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Programming.Errors.html#Programming.Errors.RetryAndBackoff"
            Err(RusotoError::Service(BatchGetItemError::ProvisionedThroughputExceeded(err))) => {
                let interval = std::cmp::min(MAX_INTERVAL_LIMIT, RETRY_BASE * 2 * retry_times);
                let interval_jitter = rng.gen_range(0..interval);
                let duration = time::Duration::from_millis(interval_jitter);

                thread::sleep(duration);

                if now.elapsed() >= time::Duration::from_millis(MAX_ELAPSED_TIME_MILLIS) {
                    // if time elapsed reach the MAX_ELAPSED_TIME_MILLIS return error
                    return Err(RusotoError::Service(
                        BatchGetItemError::ProvisionedThroughputExceeded(err),
                    )
                    .into());
                }
            }
            Err(err) => return Err(err.into()),
        }
        retry_times += 1;
    }
}

/**
 * Batch get query wrapper with exponential backoff in case of exceeded throughput
 */
pub fn execute_conversation_get_query(
    db: &mut DynamoDbClient,
    input: GetItemInput,
) -> Result<Conversation, EngineError> {
    let mut retry_times = 1;

    let mut rng = rand::thread_rng();
    let now = time::Instant::now();
    loop {
        match db.runtime.block_on(db.client.get_item(input.clone())) {
            Ok(item) => {
                let conversation: Conversation = serde_dynamodb::from_hashmap(item.item.unwrap())?;

                return Ok(conversation);
            }
            // request rate is too high, reduce the frequency of requests and use exponential backoff. "https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Programming.Errors.html#Programming.Errors.RetryAndBackoff"
            Err(RusotoError::Service(GetItemError::ProvisionedThroughputExceeded(err))) => {
                let interval = std::cmp::min(MAX_INTERVAL_LIMIT, RETRY_BASE * 2 * retry_times);
                let interval_jitter = rng.gen_range(0..interval);
                let duration = time::Duration::from_millis(interval_jitter);

                thread::sleep(duration);

                if now.elapsed() >= time::Duration::from_millis(MAX_ELAPSED_TIME_MILLIS) {
                    // if time elapsed reach the MAX_ELAPSED_TIME_MILLIS return error
                    return Err(RusotoError::Service(
                        BatchGetItemError::ProvisionedThroughputExceeded(err),
                    )
                    .into());
                }
            }
            Err(err) => return Err(err.into()),
        }
        retry_times += 1;
    }
}
