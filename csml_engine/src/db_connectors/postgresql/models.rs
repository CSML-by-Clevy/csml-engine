use diesel::{Queryable, Identifiable, Insertable, Associations,};

use chrono::NaiveDateTime;
use uuid::Uuid;

use super::schema::*;

#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[table_name = "cmsl_bot_versions"]
pub struct Bot {
    pub id: Uuid,

    pub bot_id: String,
    pub bot: String,
    pub engine_version: String,

    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Insertable, Associations, PartialEq, Debug)]
#[table_name = "cmsl_bot_versions"]
pub struct NewBot<'a> {
    pub id: Uuid,
    pub bot_id: &'a str,
    pub bot: &'a str,
    pub engine_version: &'a str,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[table_name = "csml_conversations"]
pub struct Conversation {
    pub id: Uuid,

    pub bot_id: String,
    pub channel_id: String,
    pub user_id: String,

    pub flow_id: String,
    pub step_id: String,
    pub status: String,

    pub last_interaction_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Queryable, Associations, PartialEq, Debug)]
#[table_name = "csml_conversations"]
pub struct NewConversation<'a> {
    pub id: Uuid,
    pub bot_id: &'a str,
    pub channel_id: &'a str,
    pub user_id: &'a str,

    pub flow_id: &'a str,
    pub step_id: &'a str,
    pub status: &'a str,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[table_name = "csml_memories"]
pub struct Memory {
    pub id: Uuid,
    pub bot_id: String,
    pub channel_id: String,
    pub user_id: String,

    pub key: String,
    pub value: String,

    pub expires_at: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Queryable, Associations, PartialEq, Debug)]
#[table_name = "csml_memories"]
pub struct NewMemory<'a> {
    pub id: Uuid,
    pub bot_id: &'a str,
    pub channel_id: &'a str,
    pub user_id: &'a str,

    pub key: &'a str,
    pub value: String, //serde_json::Value,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Conversation)]
#[table_name = "csml_messages"]
pub struct Message {
    pub id: Uuid,
    pub conversation_id: Uuid,

    pub flow_id: String,
    pub step_id: String,
    pub direction: String,
    pub payload: String,
    pub content_type: String,

    pub message_order: i32,
    pub interaction_order: i32,

    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Queryable, Associations, PartialEq, Debug)]
#[table_name = "csml_messages"]
pub struct NewMessages<'a> {
    pub id: Uuid,
    pub conversation_id: Uuid,

    pub flow_id: &'a str,
    pub step_id: &'a str,
    pub direction: &'a str,
    pub payload: String,
    pub content_type: &'a str,

    pub message_order: i32,
    pub interaction_order: i32,
}

#[derive(Identifiable, Insertable, Queryable, Associations, PartialEq, Debug)]
#[table_name = "csml_states"]
pub struct State {
    pub id: Uuid,

    pub bot_id: String,
    pub channel_id: String,
    pub user_id: String,

    pub type_: String,
    pub key: String,
    pub value: String,

    pub expires_at: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Queryable, Associations, PartialEq, Debug)]
#[table_name = "csml_states"]
pub struct NewState<'a> {
    pub id: Uuid,
    pub bot_id: &'a str,
    pub channel_id: &'a str,
    pub user_id: &'a str,

    pub type_: &'a str,
    pub key: &'a str,
    pub value: String,
}
