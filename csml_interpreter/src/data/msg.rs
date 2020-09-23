use crate::data::error_info::ErrorInfo;
use crate::data::primitive::PrimitiveNull;
use crate::data::{hold::Hold, message::Message, Literal, Memories, MessageData};

use std::sync::mpsc;

////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURE
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum MSG {
    Memory(Memories),
    Message(Message),
    Hold(Hold),
    Next {
        flow: Option<String>,
        step: Option<String>,
    },
    Error(Message),
}

////////////////////////////////////////////////////////////////////////////////
// STATIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl MSG {
    pub fn send(sender: &Option<mpsc::Sender<MSG>>, msg: MSG) {
        if let Some(sender) = sender {
            sender.send(msg).unwrap();
        }
    }

    pub fn send_error_msg(
        sender: &Option<mpsc::Sender<MSG>>,
        msg_data: &mut MessageData,
        value: Result<Literal, ErrorInfo>,
    ) -> Literal {
        match value {
            Ok(value) => value,
            Err(err) => {
                let message = Message {
                    content_type: "error".to_owned(),
                    content: serde_json::json!({"error": err.format_error()}),
                };
                msg_data.messages.push(message.clone());
                if let Some(sender) = sender {
                    let msg = MSG::Message(message);
                    sender.send(msg).unwrap();
                }

                PrimitiveNull::get_literal(err.position.interval)
            }
        }
    }
}
