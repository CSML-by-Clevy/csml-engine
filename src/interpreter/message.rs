use crate::parser::ast::*;
use serde::{Deserialize, Serialize};
use std::ops::Add;
use std::collections::HashMap;

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
pub enum Content {
    #[serde(rename = "text")]
    Text(String),
    #[serde(rename = "int")]
    Int(i64),
    #[serde(rename = "float")]
    Float(f64),
    #[serde(rename = "array")]
    Array(Vec<Content>),
    #[serde(rename = "object")]
    Object { 
        name: String,
        value: HashMap<String, Content>
    },
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
    pub content: Content,
}

// return Result<Message>
impl Message {
    pub fn new(expr: &Expr, string: String) -> Self {
        match expr {
            Expr::LitExpr{lit: Literal::IntLiteral(val)}     => {Message { content_type: string.to_lowercase(), content: Content::Int(*val) }},
            Expr::LitExpr{lit: Literal::FloatLiteral(val)}   => {Message { content_type: string.to_lowercase(), content: Content::Float(*val) }},
            Expr::LitExpr{lit: Literal::StringLiteral(val)}  => {Message { content_type: string.to_lowercase(), content: Content::Text(val.to_string()) }},
            Expr::LitExpr{lit: Literal::BoolLiteral(val)}    => {Message { content_type: string.to_lowercase(), content: Content::Text(val.to_string()) }},
            _                                                => {Message { content_type: "text".to_string(), content: Content::Text("Error in message creation".to_string()) } },
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