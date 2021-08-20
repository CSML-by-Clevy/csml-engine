// use serde::{Deserialize, Serialize};
// , Serialize, Deserialize

use diesel::{Queryable, Identifiable, Associations};

// use diesel::sql_types::Datetime;
use chrono::NaiveDateTime;
use std::convert::From;
// use chrono::DateTime;

use super::schema::*;

#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[table_name = "cmsl_bot"]
pub struct Bot {
    // pub id: i32, ??
    pub id: String, // uuid

    pub version_id: String,
    pub bot: String,
    pub engine_version: String,

    pub created_at: NaiveDateTime,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Bot)]
#[table_name = "client"]
pub struct Client {
    pub id: i32, // uuid ?
    pub bot_id: String, // uuid ?
    pub channel_id: String,
    pub user_id: String,
}

#[derive(Insertable)]
#[table_name = "client"]
pub struct NewClient<'a> {
    pub bot_id: &'a str,
    pub channel_id: &'a str,
    pub user_id: &'a str,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Client)]
#[table_name = "interactions"]
pub struct Interaction {
    pub id: i32, // uuid ?
    pub client_id: i32, // uuid ?
    pub success: bool,
    pub event: String, //serde_json::Value,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "interactions"]
pub struct NewInteraction<'a> {
    pub client_id: i32, // uuid ?
    pub success: bool,
    pub event: &'a str, //serde_json::Value,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Client)]
#[table_name = "conversations"]
pub struct Conversation {
    pub id: i32, // uuid ?
    pub client_id: i32, // uuid ?

    pub flow_id: String,
    pub step_id: String,
    pub status: String,

    pub last_interaction_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "conversations"]
pub struct NewConversation<'a> {
    pub client_id: i32, // uuid ?

    pub flow_id: &'a str,
    pub step_id: &'a str,
    pub status: &'a str,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Client)]
#[table_name = "memories"]
pub struct Memory {
    pub id: i32, // uuid ?
    pub client_id: i32, // uuid ?

    pub key: String,
    pub value: String,

    pub expires_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "memories"]
pub struct NewMemory<'a> {
    pub client_id: i32, // uuid ?

    pub key: &'a str, 
    pub value: String, //serde_json::Value,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Client, Interaction, Conversation)]
#[table_name = "messages"]
pub struct Message {
    pub id: i32, // uuid ?
    pub client_id: i32, // uuid ?
    pub interaction_id: i32, // uuid ?
    pub conversation_id: i32, // uuid ?

    pub flow_id: String,
    pub step_id: String,
    pub direction: String,
    pub payload: String,
    pub content_type: String,

    pub message_order: i32,
    pub interaction_order: i32,

    pub created_at: NaiveDateTime,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Client, Interaction, Conversation)]
#[table_name = "nodes"]
pub struct Node {
    pub id: i32,
    pub client_id: i32,
    pub conversation_id: i32,
    pub interaction_id: i32,

    pub flow_id: String,
    pub step_id: String,
    pub next_flow: Option<String>,
    pub next_step: Option<String>,

    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "nodes"]
pub struct NewNode<'a> {
    pub client_id: i32, // uuid ?
    pub conversation_id: i32,
    pub interaction_id: i32,

    pub flow_id: &'a str,
    pub step_id: &'a str,
    pub next_flow: Option<&'a str>,
    pub next_step: Option<&'a str>,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Client)]
#[table_name = "states"]
pub struct State {
    pub id: i32,
    pub client_id: i32,

    pub type_: String,
    pub key: String,
    pub value: String,

    pub expires_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "states"]
pub struct NewState<'a> {
    pub client_id: i32, // uuid ?

    pub type_: &'a str,
    pub key: &'a str,
    pub value: String,
}
