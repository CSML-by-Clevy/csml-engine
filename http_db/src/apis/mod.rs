use reqwest;
use serde_json;

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    Interpreter(String),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

mod conversations_api;
pub use self::conversations_api::{ ConversationsApi, ConversationsApiClient };
mod interactions_api;
pub use self::interactions_api::{ InteractionsApi, InteractionsApiClient };
mod memories_api;
pub use self::memories_api::{ MemoriesApi, MemoriesApiClient };
mod messages_api;
pub use self::messages_api::{ MessagesApi, MessagesApiClient };
mod nodes_api;
pub use self::nodes_api::{ NodesApi, NodesApiClient };
mod state_api;
pub use self::state_api::{ StateApi, StateApiClient };

pub mod configuration;
pub mod client;
