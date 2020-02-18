use crate::data::primitive::{PrimitiveObject, PrimitiveString};
use crate::data::{send_msg, Literal, Memories, Message, MSG};
use crate::error_format::ErrorInfo;
use crate::parser::ExitCondition;

use core::ops::Add;
use nom::lib::std::collections::HashMap;
use std::sync::mpsc;

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
