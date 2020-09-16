use crate::{ConversationInfo, ManagerError, encrypt::encrypt_data};
use crate::db_connectors::dynamodb::{Message, get_db};
use rusoto_dynamodb::*;
use std::collections::HashMap;

use crate::db_connectors::dynamodb::utils::*;

fn format_messages(
    data: &ConversationInfo,
    messages: &[serde_json::Value],
    interaction_order: i32,
    direction: &str,
) -> Result<Vec<Message>, ManagerError> {

    let mut res = vec![];

    for (i, message) in messages.iter().enumerate() {
        res.push(Message::new(
            &data.client,
            &data.conversation_id,
            &data.interaction_id,
            &data.context.flow,
            &data.context.step,
            direction,
            interaction_order,
            i as i32,
            &encrypt_data(&message["payload"])?,
            &message["payload"]["content_type"].to_string(),
        ));
    }

    Ok(res)
}

pub fn add_messages_bulk(
    data: &ConversationInfo,
    messages: &[serde_json::Value],
    interaction_order: i32,
    direction: &str,
) -> Result<(), ManagerError> {

    if messages.len() == 0 {
        return Ok(());
    }

    let messages = format_messages(data, messages, interaction_order, direction)?;

    // We can only use BatchWriteItem on up to 25 items at once,
    // so we need to split the messages to write into chunks of max
    // 25 items.
    for chunk in messages.chunks(25) {

        let mut request_items = HashMap::new();

        let mut items_to_write = vec![];
        for data in chunk {
            items_to_write.push(WriteRequest {
                put_request: Some(PutRequest {
                    item: serde_dynamodb::to_hashmap(&data)?,
                }),
                ..Default::default()
            });
        };

        request_items.insert(
            get_table_name()?,
            items_to_write,
        );

        let input = BatchWriteItemInput {
            request_items,
            ..Default::default()
        };

        let db = get_db(&data.db)?;
        let mut runtime = db.get_runtime()?;
        let future = db.client.batch_write_item(input);

        runtime.block_on(future)?;
    }

    Ok(())
}
