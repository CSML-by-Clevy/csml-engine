use crate::{Client, EngineError, data::DynamoDbClient};
use rusoto_core::RusotoError;
use rusoto_dynamodb::{BatchWriteItemError, BatchWriteItemInput, DynamoDb};
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
pub fn execute_batch_write_query(db: &mut DynamoDbClient, input: BatchWriteItemInput) -> Result<(), RusotoError<BatchWriteItemError>> {
    let mut retry_times = 1;

    let mut rng = rand::thread_rng();
    let now = time::Instant::now();
    loop {
        
        match db.runtime.block_on(db.client.batch_write_item(input.clone())) {
            Ok(_) => return Ok(()),
            // request rate is too high, reduce the frequency of requests and use exponential backoff. "https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Programming.Errors.html#Programming.Errors.RetryAndBackoff"
            Err(RusotoError::Service(BatchWriteItemError::ProvisionedThroughputExceeded(err))) => {

                let interval = std::cmp::min(MAX_INTERVAL_LIMIT,RETRY_BASE * 2 * retry_times);
                let interval_jitter = rng.gen_range(0,interval);
                let duration = time::Duration::from_millis(interval_jitter);

                thread::sleep(duration);

                if now.elapsed() >= time::Duration::from_millis(MAX_ELAPSED_TIME_MILLIS) {
                    // if time elapsed reach the MAX_ELAPSED_TIME_MILLIS return error
                    return Err(RusotoError::Service(BatchWriteItemError::ProvisionedThroughputExceeded(err)))
                }
            },
            Err(err) => return Err(err)
        }
        retry_times += 1;
    }
}
