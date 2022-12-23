use diesel::{Queryable, Identifiable, Insertable, Associations, backend};

use uuid;
use std::io::prelude::*;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::{Binary};
use diesel::sqlite::Sqlite;
use diesel::backend::Backend;
use std::fmt::{Display, Formatter};
use std::fmt;

use chrono::NaiveDateTime;
use super::schema::*;

#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[diesel(table_name = cmsl_bot_versions)]
pub struct Bot {
    pub id: UUID,

    pub bot_id: String,
    pub bot: String,
    pub engine_version: String,

    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Insertable, Associations, PartialEq, Debug)]
#[diesel(table_name = cmsl_bot_versions, belongs_to(Bot))]
pub struct NewBot<'a> {
    pub id: UUID,
    pub bot_id: &'a str,
    pub bot: &'a str,
    pub engine_version: &'a str,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[diesel(table_name = csml_conversations, belongs_to(Bot))]
pub struct Conversation {
    pub id: UUID,

    pub bot_id: String,
    pub channel_id: String,
    pub user_id: String,

    pub flow_id: String,
    pub step_id: String,
    pub status: String,

    pub last_interaction_at: NaiveDateTime,

    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub expires_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Queryable, Associations, PartialEq, Debug)]
#[diesel(table_name = csml_conversations, belongs_to(Bot))]
pub struct NewConversation<'a> {
    pub id: UUID,
    pub bot_id: &'a str,
    pub channel_id: &'a str,
    pub user_id: &'a str,

    pub flow_id: &'a str,
    pub step_id: &'a str,
    pub status: &'a str,

    pub expires_at: Option<NaiveDateTime>,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[diesel(table_name = csml_memories, belongs_to(Bot))]
pub struct Memory {
    pub id: UUID,
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
#[diesel(table_name = csml_memories, belongs_to(Bot))]
pub struct NewMemory<'a> {
    pub id: UUID,
    pub bot_id: &'a str,
    pub channel_id: &'a str,
    pub user_id: &'a str,

    pub key: &'a str,
    pub value: String, //serde_json::Value,

    pub expires_at: Option<NaiveDateTime>,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[diesel(table_name = csml_messages, belongs_to(Conversation))]
pub struct Message {
    pub id: UUID,
    pub conversation_id: UUID,

    pub flow_id: String,
    pub step_id: String,
    pub direction: String,
    pub payload: String,
    pub content_type: String,

    pub message_order: i32,
    pub interaction_order: i32,

    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,

    pub expires_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Queryable, Associations, PartialEq, Debug)]
#[diesel(table_name = csml_messages, belongs_to(Conversation))]
pub struct NewMessages<'a> {
    pub id: UUID,
    pub conversation_id: UUID,

    pub flow_id: &'a str,
    pub step_id: &'a str,
    pub direction: &'a str,
    pub payload: String,
    pub content_type: &'a str,

    pub message_order: i32,
    pub interaction_order: i32,

    pub expires_at: Option<NaiveDateTime>,
}

#[derive(Identifiable, Insertable, Queryable, Associations, PartialEq, Debug)]
#[diesel(table_name = csml_states, belongs_to(Bot))]
pub struct State {
    pub id: UUID,

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
#[diesel(table_name = csml_states, belongs_to(Bot))]
pub struct NewState<'a> {
    pub id: UUID,
    pub bot_id: &'a str,
    pub channel_id: &'a str,
    pub user_id: &'a str,

    pub type_: &'a str,
    pub key: &'a str,
    pub value: String,

    pub expires_at: Option<NaiveDateTime>,
}


#[derive(Debug, Clone, Copy, FromSqlRow, AsExpression, Hash, Eq, PartialEq)]
#[diesel(sql_type = Binary)]
pub struct UUID(pub uuid::Uuid);

impl UUID {
    pub fn new_v4() -> Self {
        Self(uuid::Uuid::new_v4())
    }

    pub fn parse_str(str_uuid: &str) -> Result<Self, uuid::Error> {
        Ok(Self(uuid::Uuid::parse_str(str_uuid)?))
    }

    pub fn get_uuid(self) -> uuid::Uuid {
        self.0
    }
}

impl From<UUID> for uuid::Uuid {
    fn from(s: UUID) -> Self {
        s.0
    }
}

impl Display for UUID {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromSql<Binary, Sqlite> for UUID {
    fn from_sql(value: backend::RawValue<'_, Sqlite>) -> deserialize::Result<Self> {
        let bytes = value.as_bytes();
        // let bytes = not_none!(bytes);
        let bytes = match bytes { //TODO better error message for not null error
            Some(bytes) => bytes,
            None => return Err(Box::new(diesel::NotFound)),
        };
        uuid::Uuid::from_slice(bytes.read_blob()).map(UUID).map_err(|e| e.into())
    }
}

impl ToSql<Binary, Sqlite> for UUID {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        out.write_all(self.0.as_bytes())
            .map(|_| IsNull::No)
            .map_err(Into::into)
    }
}