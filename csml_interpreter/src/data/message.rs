use crate::data::message_data::MessageData;
use crate::data::position::Position;
use crate::data::Client;
use crate::data::Literal;
use crate::error_format::*;

use serde_json::{json, map::Map, Value};

////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURES
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub enum MessageType {
    Msg(Message),
    Empty,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub content_type: String,
    pub content: serde_json::Value,
}
const MAX_PAYLOAD_SIZE: usize = 16000;

////////////////////////////////////////////////////////////////////////////////
// METHOD FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl Message {
    pub fn new(literal: Literal, flow_name: &str) -> Result<Self, ErrorInfo> {
        if literal.primitive.to_string().len() >= MAX_PAYLOAD_SIZE {
            return Err(gen_error_info(
                Position::new(literal.interval, flow_name),
                ERROR_PAYLOAD_EXCEED_MAX_SIZE.to_owned(),
            ));
        }

        Ok(literal.primitive.to_msg(literal.content_type))
    }

    pub fn add_to_message(msg_data: MessageData, action: MessageType) -> MessageData {
        match action {
            MessageType::Msg(msg) => msg_data.add_message(msg),
            MessageType::Empty => msg_data,
        }
    }

    pub fn switch_bot_message(bot_id: &str, client: &Client) -> Self {
        Self {
            content_type: "switch_bot".to_owned(),
            content: json!({ "bot_id": bot_id, "client": client }),
        }
    }

    pub fn message_to_json(&mut self) -> Value {
        let mut map: Map<String, Value> = Map::new();

        map.insert("content_type".to_owned(), json!(self.content_type));
        map.insert("content".to_owned(), self.content.to_owned());
        Value::Object(map)
    }
}
