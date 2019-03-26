use crate::parser::ast::*;
use serde::{Deserialize, Serialize};
use std::ops::Add;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Content {
    Text(String),
    Int(i64),
    Button(String, Vec<String>)
}

//TMP
pub enum MessageType {
    Msg(Message),
    Msgs(Vec<Message>),
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
    pub remember: Option<Vec<String>>,
    pub message: Vec<Message>,
    pub next_flow: Option<String>,
    pub next_step: Option<String>,
}


impl Add for RootInterface {
    type Output = RootInterface;

    // return Result<struct, error>
    fn add(self, other: RootInterface) -> RootInterface {
        RootInterface {
            remember: None,
            message: [&self.message[..], &other.message[..]].concat(),
            next_flow: None,
            next_step: match (self.next_step, other.next_step) {
                (None, None)    => None,
                (None, t)       => t,
                (t, None)       => t,
                (_, _)          => panic!("ERROR bad paring can't have too goto at same time"),
            },
        }
    }
}

impl RootInterface {
    // fn add_remeber(){}
    pub fn add_message(&mut self, message: Message) {
        self.message.push(message);
    }

    pub fn add_next_step(&mut self, next_step: &str) {
        self.next_step = Some(next_step.to_string());
    }
}