use std::rc::Rc;

use super::configuration::Configuration;

pub struct APIClient {
    conversations_api: Box<dyn crate::apis::ConversationsApi>,
    interactions_api: Box<dyn crate::apis::InteractionsApi>,
    memories_api: Box<dyn crate::apis::MemoriesApi>,
    messages_api: Box<dyn crate::apis::MessagesApi>,
    nodes_api: Box<dyn crate::apis::NodesApi>,
    state_api: Box<dyn crate::apis::StateApi>,
}

impl APIClient {
    pub fn new(configuration: Configuration) -> APIClient {
        let rc = Rc::new(configuration);

        APIClient {
            conversations_api: Box::new(crate::apis::ConversationsApiClient::new(rc.clone())),
            interactions_api: Box::new(crate::apis::InteractionsApiClient::new(rc.clone())),
            memories_api: Box::new(crate::apis::MemoriesApiClient::new(rc.clone())),
            messages_api: Box::new(crate::apis::MessagesApiClient::new(rc.clone())),
            nodes_api: Box::new(crate::apis::NodesApiClient::new(rc.clone())),
            state_api: Box::new(crate::apis::StateApiClient::new(rc.clone())),
        }
    }

    pub fn conversations_api(&self) -> &dyn crate::apis::ConversationsApi {
        self.conversations_api.as_ref()
    }

    pub fn interactions_api(&self) -> &dyn crate::apis::InteractionsApi {
        self.interactions_api.as_ref()
    }

    pub fn memories_api(&self) -> &dyn crate::apis::MemoriesApi {
        self.memories_api.as_ref()
    }

    pub fn messages_api(&self) -> &dyn crate::apis::MessagesApi {
        self.messages_api.as_ref()
    }

    pub fn nodes_api(&self) -> &dyn crate::apis::NodesApi {
        self.nodes_api.as_ref()
    }

    pub fn state_api(&self) -> &dyn crate::apis::StateApi {
        self.state_api.as_ref()
    }
}
