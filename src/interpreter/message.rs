use crate::error_format::data::ErrorInfo;
use crate::parser::literal::Literal;
use serde_json::{json, map::Map, Value};
use std::ops::Add;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum MessageType {
    Msg(Message),
    Empty,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub content_type: String,
    pub content: Literal,
}

impl Message {
    pub fn new(literal: Literal) -> Self {
        match literal.clone() {
            Literal::StringLiteral { interval, .. } => Self {
                content_type: "text".to_owned(),
                content: Literal::name_object(
                    "text".to_lowercase(),
                    &literal,
                    interval.clone()
                ),
            },
            Literal::IntLiteral { interval, .. }
            | Literal::FloatLiteral { interval, .. }
            | Literal::BoolLiteral { interval, .. } => Self {
                content_type: "text".to_owned(),
                content: Literal::name_object(
                    "text".to_lowercase(),
                    &Literal::string(literal.to_string(), interval.clone()),
                    interval
                )
            },
            Literal::ArrayLiteral { .. } => Self {
                content_type: "array".to_owned(),
                content: literal,
            },
            Literal::ObjectLiteral {
                properties: value,
                interval,
            } => Self {
                content_type: "object".to_owned(),
                content: Literal::object(value, interval),
            },
            Literal::FunctionLiteral {
                name,
                value,
                interval: _,
            } => Self {
                content_type: name.to_owned(),
                content: *value,
            },
            Literal::Null { interval, .. } => Self {
                content_type: "text".to_owned(),
                content: Literal::name_object(
                    "text".to_lowercase(),
                    &Literal::null(interval.to_owned()),
                    interval.clone()
                ),
            },
        }
    }

    pub fn add_to_message(root: MessageData, action: MessageType) -> MessageData {
        match action {
            MessageType::Msg(msg) => root.add_message(msg),
            MessageType::Empty => root,
        }
    }

    pub fn message_to_json(self) -> Value {
        let mut map: Map<String, Value> = Map::new();
        let value = self.content.to_json();
        match &self.content_type {
            name if name == "button" => {
                return button_to_json(json!(name), value)
            },
            name if name == "question" => {
                map.insert("content_type".to_owned(), json!(name));
                map.insert("content".to_owned(), question_to_json(value));
            },
            name if name == "button" => {
                return button_to_json(json!(name), value)
            },
            name => {
                map.insert("content_type".to_owned(), json!(name));
                map.insert("content".to_owned(), value);
            }
        }
        Value::Object(map)
    }
}

fn question_to_json(value: Value) -> Value {
    let mut map: Map<String, Value> = Map::new();

    match value["title"].clone() {
        Value::Null => (),
        val => {map.insert("title".to_owned(), val);}
    };

    let buttons = match value["buttons"].clone() {
        Value::Array(array) => {
            array.iter().fold(vec!(),|mut vec, elem| {
            vec.push(button_to_json(json!("button"), elem["button"].clone()));
            vec
        })},
        _ => vec!(),
    };
    map.insert("buttons".to_owned(), json!(buttons));
    Value::Object(map)
}

fn button_to_json(name: Value, value: Value) -> Value {
    let mut map: Map<String, Value> = Map::new();
    map.insert("content_type".to_owned(), name);
    map.insert("content".to_owned(), json!({"title": value["title"].clone(), "payload": value["payload"].clone()}));
    map.insert("accepts".to_owned(), value["accept"].clone());
    Value::Object(map)
}

#[derive(Debug, Clone)]
pub struct Memories {
    pub key: String,
    pub value: Literal,
}

impl Memories {
    pub fn memorie_to_jsvalue(self) -> Value {
        let mut map: Map<String, Value> = Map::new();
        map.insert("key".to_owned(), json!(self.key));
        map.insert("value".to_owned(), self.value.to_json());
        Value::Object(map)
    }
}

#[derive(Debug, Clone, Default)]
pub struct MessageData {
    pub memories: Option<Vec<Memories>>,
    pub messages: Vec<Message>,
    pub step_vars: Option<HashMap<String, Literal>>,
    pub index: i64,
    pub next_flow: Option<String>,
    pub next_step: Option<String>,
}

impl Add for MessageData {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            memories: match (self.memories, other.memories) {
                (Some(memory), None) => Some(memory),
                (None, Some(newmemory)) => Some(newmemory),
                (Some(memory), Some(newmemory)) => Some([&memory[..], &newmemory[..]].concat()),
                _ => None,
            },
            messages: [&self.messages[..], &other.messages[..]].concat(),
            // TODO: refactor
            step_vars: self.step_vars.clone(), 
            next_flow: match (&self.next_flow, &other.next_flow) {
                (Some(flow), None) => Some(flow.to_owned()),
                (None, Some(flow)) => Some(flow.to_owned()),
                (Some(flow), Some(_)) => Some(flow.to_owned()),
                _ => None,
            },
            next_step: match (&self.next_step, &other.next_step) {
                (Some(step), None) => Some(step.to_owned()),
                (None, Some(step)) => Some(step.to_owned()),
                (Some(step), Some(_)) => Some(step.to_owned()),
                _ => None,
            },
            index: self.index,
        }
    }
}

impl MessageData {
    pub fn add_message(mut self, message: Message) -> Self {
        self.messages.push(message);
        self
    }

    pub fn add_to_memory(mut self, key: &str, value: Literal) -> Self {
        if let Some(ref mut vec) = self.memories {
            if let Literal::ObjectLiteral { .. } = &value {
                vec.push(Memories {
                    key: key.to_owned(),
                    value,
                });
            } else {
                vec.push(Memories {
                    key: key.to_owned(),
                    value,
                });
            }
        } else {
            self.memories = Some(vec![Memories {
                key: key.to_owned(),
                value: value,
            }])
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

    pub fn error_to_message(resutl: Result<Self, ErrorInfo>) -> Self {
        match resutl {
            Ok(v) => v,
            Err(ErrorInfo { message, interval }) => {
                let msg = format!(
                    "{} at line {}, column {}",
                    message, interval.line, interval.column
                );
                Self {
                    memories: None,
                    messages: vec![Message {
                        content_type: "error".to_owned(),
                        content: Literal::name_object(
                            "text".to_owned(),
                            &Literal::string(msg, interval.clone()),
                            interval,
                        ),
                    }],
                    step_vars: None,
                    next_flow: None,
                    next_step: None,
                    index: 0,
                }
            }
        }
    }
}
