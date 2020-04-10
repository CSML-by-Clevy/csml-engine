use crate::data::execution_context::ExecutionContext;
use crate::data::hold::Hold;
use crate::data::message::Message;
use crate::data::Memories;

use std::sync::mpsc;

////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURE
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum MSG {
    Memory(Memories),
    Message(Message),
    Hold(Hold),
    Next(ExecutionContext),
    Error(Message),
}

////////////////////////////////////////////////////////////////////////////////
// STATIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl MSG {
    pub fn send(sender: &Option<mpsc::Sender<MSG>>, msg: MSG) {
        if let Some(sender) = sender {
            println!("[+] sending {:?}", msg);
            sender.send(msg).unwrap();
        }
    }
}
