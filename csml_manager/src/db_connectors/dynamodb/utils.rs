use crate::{Client, ManagerError};

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
pub fn get_table_name() -> Result<String, ManagerError> {
    match std::env::var("AWS_DYNAMODB_TABLE") {
        Ok(val) => return Ok(val),
        _ => {
            return Err(ManagerError::Manager(
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
