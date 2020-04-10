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

// fn card_to_json<'a>(value: &'a mut Value) -> &'a Value {
//     if let Value::Array(ref mut array) = value["buttons"] {
//         for elem in array.iter_mut() {
//             button_to_json(elem);
//         }
//     }
//     value
// }

// fn carousel_to_json<'a>(value: &'a mut Value) -> &'a Value {
//     if let Value::Array(ref mut array) = value["cards"] {
//         for elem in array.iter_mut() {
//             card_to_json(elem);
//         }
//     }
//     value
// }

// fn question_to_json<'a>(value: &'a mut Value) -> &'a Value {
//     if let Value::Array(ref mut array) = value["buttons"] {
//         for elem in array.iter_mut() {
//             button_to_json(elem);
//         }
//     }
//     value
// }

// fn button_to_json<'a>(value: &'a mut Value) -> &'a Value {
//     if let Value::Object(ref mut map) = value {
//         map.insert("content_type".to_owned(), json!("button"));
//         let title = map["title"].clone();
//         let payload = map["title"].clone();
//         map.insert(
//             "content".to_owned(),
//             json!({"title": title, "payload": payload}),
//         );
//     }
//     value
// }

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

    pub fn message_to_json(&mut self) -> Value {
        let mut map: Map<String, Value> = Map::new();

        map.insert("content_type".to_owned(), json!(self.content_type));
        map.insert("content".to_owned(), self.content.to_owned());
        // match &self.content_type {
            // name if name == "button" => {
            //     map.insert("content_type".to_owned(), json!(name));
            //     map.insert(
            //         "content".to_owned(),
            //         button_to_json(&mut self.content).to_owned(),
            //     );
            // }
            // name if name == "question" => {
            //     map.insert("content_type".to_owned(), json!(name));
            //     map.insert(
            //         "content".to_owned(),
            //         question_to_json(&mut self.content).to_owned(),
            //     );
            // }
            // name if name == "carousel" => {
            //     map.insert("content_type".to_owned(), json!(name));
            //     map.insert(
            //         "content".to_owned(),
            //         carousel_to_json(&mut self.content).to_owned(),
            //     );
            // }
            // name if name == "card" => {
            //     map.insert("content_type".to_owned(), json!(name));
            //     map.insert(
            //         "content".to_owned(),
            //         card_to_json(&mut self.content).to_owned(),
            //     );
            // }
            // name => {
            //     map.insert("content_type".to_owned(), json!(name));
            //     map.insert("content".to_owned(), self.content.to_owned());
            // }
        // }
        Value::Object(map)
    }
}
