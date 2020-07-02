/*
 * CSML engine microservices
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;
use std::rc::Rc;

use reqwest;

use super::{configuration, Error};

pub struct MemoriesApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl MemoriesApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> MemoriesApiClient {
        MemoriesApiClient { configuration }
    }
}

pub trait MemoriesApi {
    fn get_memories(
        &self,
        bot_id: &str,
        user_id: &str,
        channel_id: &str,
    ) -> Result<Vec<crate::models::MemoryModel>, Error>;
}

impl MemoriesApi for MemoriesApiClient {
    fn get_memories(
        &self,
        bot_id: &str,
        user_id: &str,
        channel_id: &str,
    ) -> Result<Vec<crate::models::MemoryModel>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/memories", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("bot_id", &bot_id.to_string())]);
        req_builder = req_builder.query(&[("user_id", &user_id.to_string())]);
        req_builder = req_builder.query(&[("channel_id", &channel_id.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }
}
