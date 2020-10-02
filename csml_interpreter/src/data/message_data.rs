use crate::data::error_info::ErrorInfo;
use crate::data::{Hold, Literal, Memory, Message, MSG};
use crate::parser::ExitCondition;

use core::ops::Add;
use std::sync::mpsc;

////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURE
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct MessageData {
    pub memories: Option<Vec<Memory>>,
    pub messages: Vec<Message>,
    pub hold: Option<Hold>,
    pub exit_condition: Option<ExitCondition>,
}

////////////////////////////////////////////////////////////////////////////////
// TRAIT FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl Default for MessageData {
    fn default() -> Self {
        Self {
            memories: None,
            messages: Vec::new(),
            hold: None,
            exit_condition: None,
        }
    }
}

impl Add<MessageData> for MessageData {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            memories: match (self.memories, other.memories) {
                (Some(memory), None) => Some(memory),
                (None, Some(new_memory)) => Some(new_memory),
                (Some(memory), Some(new_memory)) => Some([&memory[..], &new_memory[..]].concat()),
                _ => None,
            },
            messages: [&self.messages[..], &other.messages[..]].concat(),
            hold: self.hold,
            exit_condition: match (&self.exit_condition, &other.exit_condition) {
                (Some(exit_condition), None) => Some(exit_condition.to_owned()),
                (None, Some(exit_condition)) => Some(exit_condition.to_owned()),
                (Some(exit_condition), Some(_)) => Some(exit_condition.to_owned()),
                _ => None,
            },
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// STATIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl MessageData {
    pub fn error_to_message(
        result: Result<Self, ErrorInfo>,
        sender: &Option<mpsc::Sender<MSG>>,
    ) -> Self {
        match result {
            Ok(message_data) => message_data,
            Err(err) => {
                let json_msg = serde_json::json!({"error": err.format_error()});

                MSG::send(
                    sender,
                    MSG::Error(Message {
                        content_type: "error".to_owned(),
                        content: json_msg.clone(),
                    }),
                );

                Self {
                    memories: None,
                    messages: vec![Message {
                        content_type: "error".to_owned(),
                        content: json_msg,
                    }],
                    hold: None,
                    exit_condition: Some(ExitCondition::Error),
                }
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// METHOD FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl MessageData {
    pub fn add_message(mut self, message: Message) -> Self {
        self.messages.push(message);
        self
    }

    pub fn add_to_memory(&mut self, key: &str, value: Literal) {
        let content_type = &value.content_type;

        if let Some(ref mut vec) = self.memories {
            vec.push(Memory {
                key: key.to_owned(),
                value: value.primitive.format_mem(content_type, true),
            });
        } else {
            self.memories = Some(vec![Memory {
                key: key.to_owned(),
                value: value.primitive.format_mem(content_type, true),
            }])
        };
    }
}
