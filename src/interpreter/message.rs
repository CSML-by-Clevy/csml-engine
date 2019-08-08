use std::ops::Add;
use serde_json::{Value, json, map::Map};
use crate::parser::ast::*;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageType {
    Msg(Message),
    Empty,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub content_type: String,
    pub content: Literal,
}

fn obj_to_message(properties: HashMap<String, Literal>) -> (String, Literal) {
    if properties.len() > 1 {
        ("object".to_owned(), Literal::object(properties))
    } else if properties.len() == 1 {
        for (k, _) in properties.iter() {
            return (k.to_owned(), Literal::object(properties.clone()))
        }

        unreachable!()
    } else {
        unreachable!()
    }
}

impl Message {
    pub fn new(literal: Literal) -> Self {
        match literal {
            Literal::IntLiteral{..} => Message {
                content_type: "text".to_owned(),
                content: literal,
            },
            Literal::FloatLiteral{..} => Message {
                content_type: "text".to_owned(),
                content: literal,
            },
            Literal::StringLiteral{..} => Message {
                content_type: "text".to_owned(),
                content: literal,
            },
            Literal::BoolLiteral{..} => Message {
                content_type: "text".to_owned(),
                content: literal,
            },
            Literal::ArrayLiteral{..} => Message {
                content_type: "array".to_owned(),
                content: literal,
            },
            Literal::ObjectLiteral{properties: ref value, ..} => {
                let (content_type, content) = obj_to_message(value.to_owned());

                Message {
                    content_type,
                    content,
                }
            },
            Literal::Null{..} => Message{
                content_type: literal.type_to_string(),
                content: literal,
            },
        }
    }

    pub fn lit_to_json(literal: Literal) -> Value {
        match literal {
            Literal::StringLiteral{value, ..} => {
                json!(value)
            },
            Literal::IntLiteral{value, ..} => {
                json!(value)
            },
            Literal::FloatLiteral{value, ..} => {
                json!(value)
            },
            Literal::BoolLiteral{value, ..} => {
                json!(value)
            },
            Literal::ArrayLiteral{items, ..} => {
                let mut array: Vec<Value> = vec![];
                for val in items.iter() {
                    array.push(Message::lit_to_json(val.to_owned()));
                }
                Value::Array(array)
            },
            Literal::ObjectLiteral{ref properties, ..} if properties.len() < 2 => {
                let mut map: Map<String, Value> = Map::new();
                for (k, v) in properties.to_owned().drain().take(1) {
                    map.insert(k.to_owned(), Message::lit_to_json(v));
                }
                Value::Object(map)
            },
            Literal::ObjectLiteral{properties, ..} => {
                let mut map: Map<String, Value> = Map::new();
                for (k, v) in properties.to_owned().drain() {
                    map.insert(k.to_owned(), Message::lit_to_json(v));
                }
                Value::Object(map)
            },
            Literal::Null{..} => {
                json!(null)
            }
        }
    }

    pub fn message_to_json(self) -> Value {
        let mut map: Map<String, Value> = Map::new();
        let value = Message::lit_to_json(self.content);

        map.insert("content_type".to_owned(), json!(self.content_type));
        map.insert("content".to_owned(), value);
        let json = Value::Object(map);

        json
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Memories {
    pub key: String,
    pub value: Literal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageData {
    pub memories: Option<Vec<Memories>>,
    pub messages: Vec<Message>,
    pub next_flow: Option<String>,
    pub next_step: Option<String>,
}

impl Add for MessageData {
    type Output = MessageData;

    fn add(self, other: MessageData) -> MessageData {
        MessageData {
            memories: match (self.memories, other.memories) {
                (Some(memory), None) => Some(memory),
                (None, Some(newmemory)) => Some(newmemory),
                (Some(memory), Some(newmemory)) => Some([&memory[..], &newmemory[..]].concat()),
                _ => None,
            },
            messages: [&self.messages[..], &other.messages[..]].concat(),
            next_flow: None,
            next_step: match (self.next_step, other.next_step) {
                (Some(step), None) => Some(step),
                (None, Some(step)) => Some(step),
                (Some(step), Some(_)) => Some(step),
                _ => None,
            },
        }
    }
}

impl MessageData {
    pub fn add_message(mut self, message: Message) -> Self {
        self.messages.push(message);
        self
    }

    pub fn add_to_memory(mut self, key: String, value: Literal) -> Self {
        if let Some(ref mut vec) = self.memories {
            if let Literal::ObjectLiteral{..} = &value{
                vec.push(Memories{key: key.clone(), value});
            } else {
                vec.push(Memories{key: key.clone(), value: value});
            }
        } else {
            match &value {
                Literal::ObjectLiteral{..} => self.memories = Some(vec![Memories{key: key.clone(), value: value}]),
                _                          => self.memories = Some(vec![Memories{key: key.clone(), value: value}])
            };
        }
        self
    }

    pub fn add_next_step(mut self, next_step: &str) -> Self {
        self.next_step = Some(next_step.to_string());
        self
    }

    pub fn add_next_flow(mut self, next_step: &str) -> Self {
        self.next_flow = Some(next_step.to_string());
        self
    }
}
