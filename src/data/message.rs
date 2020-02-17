use crate::data::message_data::MessageData;
use crate::data::Literal;
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

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn question_to_json(mut value: Value) -> Value {
    if let Value::Array(ref mut array) = value["buttons"] {
        for elem in array.iter_mut() {
            if let Value::Object(ref mut map) = elem {
                map.insert("content_type".to_owned(), json!("button"));
                let title = map["title"].clone();
                let payload = map["title"].clone();
                map.insert(
                    "content".to_owned(),
                    json!({"title": title, "payload": payload}),
                );
            }
        }
    }
    value
}

fn button_to_json(name: Value, mut value: Value) -> Value {
    if let Value::Object(ref mut map) = value {
        map.insert("content_type".to_owned(), name);
        let title = map["title"].clone();
        let payload = map["title"].clone();
        map.insert(
            "content".to_owned(),
            json!({"title": title, "payload": payload}),
        );
    }
    value
}

////////////////////////////////////////////////////////////////////////////////
// METHOD FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl Message {
    pub fn new(literal: Literal) -> Self {
        literal.primitive.to_msg(literal.content_type)
    }

    pub fn add_to_message(root: MessageData, action: MessageType) -> MessageData {
        match action {
            MessageType::Msg(msg) => root.add_message(msg),
            MessageType::Empty => root,
        }
    }

    pub fn message_to_json(self) -> Value {
        let mut map: Map<String, Value> = Map::new();

        match &self.content_type {
            name if name == "button" => {
                map.insert("content_type".to_owned(), json!(name));
                map.insert(
                    "content".to_owned(),
                    button_to_json(json!(name), self.content),
                );
            }
            name if name == "question" => {
                map.insert("content_type".to_owned(), json!(name));
                map.insert("content".to_owned(), question_to_json(self.content));
            }
            name => {
                map.insert("content_type".to_owned(), json!(name));
                map.insert("content".to_owned(), self.content);
            }
        }
        Value::Object(map)
    }
}
