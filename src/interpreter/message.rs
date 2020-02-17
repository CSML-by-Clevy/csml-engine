use crate::error_format::data::ErrorInfo;
use crate::interpreter::ast_interpreter::send_msg;
use crate::parser::literal::Literal;
use crate::primitive::{object::PrimitiveObject, string::PrimitiveString};
use serde_json::{json, map::Map, Value};
use std::collections::HashMap;
use std::ops::Add;
use std::sync::mpsc;

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

#[derive(Debug, Clone)]
pub struct Memories {
    pub key: String,
    pub value: Value,
}

impl Memories {
    pub fn new(key: String, value: Literal) -> Self {
        Self {
            key,
            value: value.primitive.to_json(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum MSG {
    Memorie(Memories),
    Message(Message),
    Hold {
        instruction_index: usize,
        step_vars: serde_json::Value,
    },
    NextFlow(String),
    NextStep(String),
    Error(Message),
}

pub fn step_vars_to_json(map: HashMap<String, Literal>) -> Value {
    let mut json_map = Map::new();
    for (key, val) in map.iter() {
        json_map.insert(key.to_owned(), val.primitive.to_json());
    }
    serde_json::json!(json_map)
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExitCondition {
    Goto,
    Error,
    Break,
    Hold,
}

#[derive(Debug, Clone)]
pub struct MessageData {
    pub memories: Option<Vec<Memories>>,
    pub messages: Vec<Message>,
    pub step_vars: Option<HashMap<String, Literal>>,
    pub instruction_index: usize,
    pub next_flow: Option<String>,
    pub next_step: Option<String>,
    pub exit_condition: Option<ExitCondition>,
}

impl Default for MessageData {
    fn default() -> Self {
        Self {
            memories: None,
            messages: Vec::new(),
            step_vars: None,
            instruction_index: 0,
            next_flow: None,
            next_step: None,
            exit_condition: None,
        }
    }
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
            instruction_index: self.instruction_index,
            exit_condition: match (&self.exit_condition, &other.exit_condition) {
                (Some(exit_condition), None) => Some(exit_condition.to_owned()),
                (None, Some(exit_condition)) => Some(exit_condition.to_owned()),
                (Some(exit_condition), Some(_)) => Some(exit_condition.to_owned()),
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

    pub fn add_to_memory(&mut self, key: &str, value: Literal) {
        if let Some(ref mut vec) = self.memories {
            vec.push(Memories {
                key: key.to_owned(),
                value: value.primitive.to_json(),
            });
        } else {
            self.memories = Some(vec![Memories {
                key: key.to_owned(),
                value: value.primitive.to_json(),
            }])
        };
    }

    pub fn add_next_step(mut self, next_step: &str) -> Self {
        self.next_step = Some(next_step.to_string());
        self
    }

    pub fn add_next_flow(mut self, next_step: &str) -> Self {
        self.next_flow = Some(next_step.to_string());
        self
    }

    pub fn error_to_message(
        result: Result<Self, ErrorInfo>,
        sender: &Option<mpsc::Sender<MSG>>,
    ) -> Self {
        match result {
            Ok(v) => v,
            Err(ErrorInfo { message, interval }) => {
                let msg = PrimitiveString::get_literal(
                    "string",
                    &format!(
                        "{} at line {}, column {}",
                        message, interval.line, interval.column
                    ),
                    interval,
                );

                let mut hashmap = HashMap::new();

                hashmap.insert("error".to_owned(), msg);

                let literal = PrimitiveObject::get_literal("error", &hashmap, interval);

                send_msg(
                    sender,
                    MSG::Error(Message {
                        content_type: "error".to_owned(),
                        content: literal.primitive.to_json(),
                    }),
                );

                Self {
                    memories: None,
                    messages: vec![Message {
                        content_type: "error".to_owned(),
                        content: literal.primitive.to_json(),
                    }],
                    step_vars: None,
                    next_flow: None,
                    next_step: None,
                    instruction_index: 0,
                    exit_condition: None,
                }
            }
        }
    }
}
