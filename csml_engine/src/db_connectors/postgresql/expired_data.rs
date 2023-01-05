use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::{EngineError, PostgresqlClient};

use super::schema::{csml_conversations, csml_memories, csml_states};

pub fn delete_expired_data(db: &mut PostgresqlClient) -> Result<(), EngineError> {
    let date_now = chrono::Utc::now().naive_utc();

    diesel::delete(csml_conversations::table.filter(csml_conversations::expires_at.lt(date_now)))
        .execute(&mut db.client)
        .ok();

    diesel::delete(csml_memories::table.filter(csml_memories::expires_at.lt(date_now)))
        .execute(&mut db.client)
        .ok();

    diesel::delete(csml_states::table.filter(csml_states::expires_at.lt(date_now)))
        .execute(&mut db.client)
        .ok();

    Ok(())
}
