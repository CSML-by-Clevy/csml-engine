use crate::data::message::Message;
use crate::data::Memories;
use crate::data::hold::Hold;
use crate::error_format::ErrorInfo;

use std::sync::mpsc;

////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURE
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub enum MSG {
    Memory(Memories),
    Message(Message),
    Hold(Hold),
    NextFlow(String),
    NextStep(String),
    Error(Message),
}

////////////////////////////////////////////////////////////////////////////////
// STATIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl MSG {
    pub fn send_msg(sender: &Option<mpsc::Sender<MSG>>, msg: MSG) -> Result<(), ErrorInfo> {
        if let Some(sender) = sender {
            if let Err(_) = sender.send(msg) {
                unimplemented!();
            }
        }

        Ok(())
    }
}
