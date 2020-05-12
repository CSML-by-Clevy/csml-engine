use crate::data::message_data::MessageData;
use crate::data::Literal;
use crate::error_format::*;
use crate::data::position::Position;

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
    pub fn new(literal: Literal) -> Result<Self, ErrorInfo> {
        if literal.primitive.to_string().len() >= MAX_PAYLOAD_SIZE {
            return Err(gen_error_info(
                Position::new(literal.interval),
                ERROR_PAYLOAD_EXCEED_MAX_SIZE.to_owned(),
            ));
        }

        Ok(literal.primitive.to_msg(literal.content_type))
    }

    pub fn add_to_message(root: MessageData, action: MessageType) -> MessageData {
        match action {
            MessageType::Msg(msg) => root.add_message(msg),
            MessageType::Empty => root,
        }
    }

    pub fn message_to_json(&mut self) -> Value {
        let mut map: Map<String, Value> = Map::new();

        map.insert("content_type".to_owned(), json!(self.content_type));
        map.insert("content".to_owned(), self.content.to_owned());
        Value::Object(map)
    }
}
