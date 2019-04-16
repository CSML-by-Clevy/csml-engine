use crate::parser::ast::*;
use serde::{Deserialize, Serialize};
use std::ops::Add;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Content {
    Text(String),
    Int(i64),
    Button(String, Vec<String>),
}

//TMP I dont like this TODO: change it
pub enum MessageType {
    Msg(Message),
    Msgs(Vec<Message>),
    Assign{name: String, value: String},
    Empty
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    #[serde(rename = "type")]
    pub my_type: String,
    pub content: Content,
}

impl Message {
    pub fn new(expr: &Expr, string: String) -> Self {
        let mut msg = Message {
            my_type: "".to_string(),
            content: Content::Text("".to_string())
        };

        match expr {
            Expr::LitExpr(Literal::IntLiteral(val))     => {msg.my_type = string; msg.content = Content::Int(*val); msg},
            Expr::LitExpr(Literal::StringLiteral(val))  => {msg.my_type = string; msg.content = Content::Text(val.to_string()); msg},
            Expr::LitExpr(Literal::BoolLiteral(val))    => {msg.my_type = string; msg.content = Content::Text(val.to_string()); msg},
            _                                           => {msg},
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RootInterface {
    pub remember: Option<Vec <(String, String)> >,
    pub messages: Vec<Message>,
    pub next_flow: Option<String>,
    pub next_step: Option<String>,
}

impl Add for RootInterface {
    type Output = RootInterface;

    // return Result<struct, error>
    fn add(self, other: RootInterface) -> RootInterface {
        RootInterface {
            remember: match (self.remember, other.remember) {
                (Some(memory), None)            => Some(memory),
                (None, Some(newmemory))         => Some(newmemory),
                (Some(memory), Some(newmemory)) => Some([&memory[..], &newmemory[..]].concat()),
                _                               => None,
            },
            messages: [&self.messages[..], &other.messages[..]].concat(),
            next_flow: None,
            next_step: match (self.next_step, other.next_step) {
                (Some(t), None)        => Some(t),
                (None, Some(t))        => Some(t),
                (Some(step1), Some(_)) => Some(step1), // should never happened
                _                      => None,
            },
        }
    }
}

impl RootInterface {
    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn add_to_memory(&mut self, name: String, value: String) {
        if let Some(ref mut vec) = self.remember {
            vec.push((name, value))
        } else {
            self.remember = Some(vec![(name, value)]);
        }
    }

    pub fn add_next_step(&mut self, next_step: &str) {
        self.next_step = Some(next_step.to_string());
    }
}