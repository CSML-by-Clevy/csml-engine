use crate::parser::ast::*;
use serde::{Deserialize, Serialize};
use std::ops::Add;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Button {
    pub title: String,
    pub buttton_type: String,
    pub accepts: Vec<String>,
    pub key: String,
    pub value: String,
    pub payload: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Question {
    pub title: String,
    pub accepts: Vec<String>,
    pub buttons: Vec<Button>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageType {
    Msg(Message),
    Assign{name: String, value: String},
    Empty
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub content_type: String,
    pub content: Literal,
}

// return Result<Message>
impl Message {
    pub fn new(expr: &Literal, string: String) -> Self {
        match expr {
            Literal::IntLiteral(..)               => {Message { content_type: string.to_lowercase(), content: expr.clone() }},
            Literal::FloatLiteral(..)             => {Message { content_type: string.to_lowercase(), content: expr.clone() }},
            Literal::StringLiteral(..)            => {Message { content_type: string.to_lowercase(), content: expr.clone() }},
            Literal::BoolLiteral(..)              => {Message { content_type: string.to_lowercase(), content: expr.clone() }},
            Literal::ArrayLiteral(..)             => {Message { content_type: "Array".to_lowercase(), content: expr.clone() }},
            Literal::ObjectLiteral{name, ..}      => {Message { content_type: name.to_lowercase(), content: expr.clone() }},
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Memories {
    pub key: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RootInterface {
    pub memories: Option< Vec<Memories> >,
    pub messages: Vec<Message>,
    pub next_flow: Option<String>,
    pub next_step: Option<String>,
}

impl Add for RootInterface {
    type Output = RootInterface;

    fn add(self, other: RootInterface) -> RootInterface {
        RootInterface {
            memories: match (self.memories, other.memories) {
                (Some(memory), None)            => Some(memory),
                (None, Some(newmemory))         => Some(newmemory),
                (Some(memory), Some(newmemory)) => Some([&memory[..], &newmemory[..]].concat()),
                _                               => None,
            },
            messages: [&self.messages[..], &other.messages[..]].concat(),
            next_flow: None,
            next_step: match (self.next_step, other.next_step) {
                (Some(step), None)        => Some(step),
                (None, Some(step))        => Some(step),
                (Some(step), Some(_))     => Some(step),
                _                         => None,
            },
        }
    }
}

impl RootInterface {
    pub fn add_message(mut self, message: Message) -> Self {
        self.messages.push(message);

        self
    }

    pub fn add_to_memory(mut self, key: String, value: String) -> Self {
        if let Some(ref mut vec) = self.memories {
            vec.push(Memories{key, value})
        } else {
            self.memories = Some(vec![Memories{key, value}]);
        }

        self
    }

    pub fn add_next_step(mut self, next_step: &str) -> Self {
        self.next_step = Some(next_step.to_string());

        self
    }
}