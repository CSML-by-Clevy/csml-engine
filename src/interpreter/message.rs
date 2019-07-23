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
    pub fn new(literal: Literal) -> Self {
        match literal {
            Literal::IntLiteral{..} => Message {
                content_type: "text".to_owned(),
                content: literal.set_name("text".to_owned()),
            },
            Literal::FloatLiteral{..} => Message {
                content_type: "text".to_owned(),
                content: literal.set_name("text".to_owned()),
            },
            Literal::StringLiteral{..} => Message {
                content_type: "text".to_owned(),
                content: literal.set_name("text".to_owned()),
            },
            Literal::BoolLiteral{..} => Message {
                content_type: "text".to_owned(),
                content: literal.set_name("text".to_owned()),
            },
            Literal::ArrayLiteral{..} => Message {
                content_type: "array".to_owned(),
                content: literal,
            },
            Literal::ObjectLiteral{name: Some(ref name), properties: ref value, ..} if name == "url" => Message {
                content_type: name.to_owned(),
                content: value[0].clone(),
            },
            Literal::ObjectLiteral{name: Some(ref name), properties: ref value, ..} if name == "wait" => Message {
                content_type: name.to_owned(),
                content: value[0].clone(),
            },
            Literal::ObjectLiteral{name: Some(ref name), properties: ref value, ..} if name == "typing" => Message {
                content_type: name.to_owned(),
                content: value[0].clone(),
            },
            Literal::ObjectLiteral{name: Some(ref name), properties: ref value, ..} if name == "image" => Message {
                content_type: name.to_owned(),
                content: value[0].clone(),
            },
            Literal::ObjectLiteral{name: Some(ref name), properties: ref value, ..} if name == "text" => Message {
                content_type: name.to_owned(),
                content: value[0].clone(),
            },
            Literal::ObjectLiteral{..} => Message {
                content_type: literal.get_name().unwrap(),
                content: literal,
            },
            Literal::Null{..} => Message{
                content_type: literal.type_to_string(),
                content: literal,
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
            if let Literal::ObjectLiteral{..} = &value{
                vec.push(Memories{key: key.clone(), value: value});
            } else {
                vec.push(Memories{key: key.clone(), value: value.set_name(key)});
            }
        } else {
            match &value {
                Literal::ObjectLiteral{..} => self.memories = Some(vec![Memories{key: key.clone(), value: value}]),
                _                          => self.memories = Some(vec![Memories{key: key.clone(), value: value.set_name(key) }])
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
