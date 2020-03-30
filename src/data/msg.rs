use crate::data::message::Message;
use crate::data::Memories;

use std::sync::mpsc;

#[derive(Debug, Clone)]
pub enum MSG {
    Memory(Memories),
    Message(Message),
    Hold {
        instruction_index: usize,
        step_vars: serde_json::Value,
    },
    NextFlow(String),
    NextStep(String),
    Error(Message),
}

pub fn send_msg(sender: &Option<mpsc::Sender<MSG>>, msg: MSG) {
    if let Some(sender) = sender {
        sender.send(msg).unwrap();
    }
}
