use crate::parser::ast::*;
use serde::{Deserialize, Serialize};
use std::ops::Add;

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

impl Message {
    pub fn new(expr: Literal, name: String) -> Self {
        match expr {
            Literal::IntLiteral(..) => Message {
                content_type: name.to_lowercase(),
                content: expr,
            },
            Literal::FloatLiteral(..) => Message {
                content_type: name.to_lowercase(),
                content: expr,
            },
            Literal::StringLiteral(..) => Message {
                content_type: name.to_lowercase(),
                content: expr,
            },
            Literal::BoolLiteral(..) => Message {
                content_type: name.to_lowercase(),
                content: expr,
            },
            Literal::ArrayLiteral(..) => Message {
                content_type: name.to_lowercase(),
                content: expr,
            },
            Literal::ObjectLiteral { ref name, .. } => Message {
                content_type: name.to_lowercase().to_owned(),
                content: expr,
            },
            Literal::Null => Message {
                content_type: expr.type_to_string(),
                content: expr,
            },
        }
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
            vec.push(Memories { key, value })
        } else {
            self.memories = Some(vec![Memories { key, value }]);
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
