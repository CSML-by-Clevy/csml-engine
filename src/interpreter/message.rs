use std::ops::Add;
use serde_json::{Value, json, map::Map};
use crate::parser::ast::*;
use crate::error_format::data::ErrorInfo;

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
            Literal::StringLiteral{interval, ..} => Message {
                content_type: "text".to_owned(),
                content: Literal::lit_to_obj(literal, "text".to_owned(), interval),
            },
            Literal::IntLiteral{interval,..} => Message {
                content_type: "text".to_owned(),
                content: Literal::lit_to_obj(Literal::string(literal.to_string(), literal.get_interval()), "text".to_owned(), interval),
            },
            Literal::FloatLiteral{interval, ..} => Message {
                content_type: "text".to_owned(),
                content: Literal::lit_to_obj(Literal::string(literal.to_string(), literal.get_interval()) , "text".to_owned(), interval),
            },
            Literal::BoolLiteral{interval, ..} => Message {
                content_type: "text".to_owned(),
                content: Literal::lit_to_obj(Literal::string(literal.to_string(), literal.get_interval()), "text".to_owned(), interval),
            },
            Literal::ArrayLiteral{..} => Message {
                content_type: "array".to_owned(),
                content: literal,
            },
            Literal::ObjectLiteral{properties: value, interval} => {
                Message {
                    content_type: "object".to_owned(),
                    content: Literal::object(value, interval),
                }
            },
            Literal::FunctionLiteral{name, value, interval} => {
                Message {
                    content_type: name.to_owned(),
                    content: Literal::name_object(name.to_owned(), &value, interval),
                }
            },
            Literal::Null{..} => Message {
                content_type: literal.type_to_string(),
                content: literal,
            },
        }
    }

    pub fn add_to_message(root: MessageData, action: MessageType) -> MessageData {
        match action {
            MessageType::Msg(msg) => root.add_message(msg),
            MessageType::Empty => root,
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
            Literal::ObjectLiteral{properties, ..} => {
                let mut map: Map<String, Value> = Map::new();
                for (k, v) in properties.to_owned().drain() {
                    map.insert(k.to_owned(), Message::lit_to_json(v));
                }
                Value::Object(map)
            },
            Literal::FunctionLiteral{name, value, ..} => {
                let mut map: Map<String, Value> = Map::new();
                let val = (*value).clone();
                map.insert(name.to_owned(), Message::lit_to_json(val));
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
        Value::Object(map)
    }
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
        map.insert("value".to_owned(), Message::lit_to_json(self.value));
        Value::Object(map)
    }
}

#[derive(Debug, Clone)]
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
                vec.push(Memories{key: key.clone(), value});
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
    
    pub fn error_to_message(resutl: Result<MessageData, ErrorInfo>) -> Self {
        match resutl {
            Ok(v) => v,
            Err(ErrorInfo{message, interval}) => {
                MessageData {
                    memories: None,
                    messages: vec!(
                        Message {
                            content_type: "text".to_owned(),
                            content: Literal::lit_to_obj(
                                Literal::string(message, interval.clone()),
                                "Error".to_owned(),
                                interval
                            )
                        }
                    ),
                    next_flow: None,
                    next_step: None,
                }
            },
        }
    }
}
