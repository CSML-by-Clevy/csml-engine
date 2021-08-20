use std::env;

use diesel::{RunQueryDsl, ExpressionMethods, QueryDsl};
use diesel::{insert_into};

use serde_json::Value;

use crate::{
    encrypt::encrypt_data, EngineError, PostgresqlClient
};

use super::{
    models::{NewInteraction, Interaction},
    schema::interactions
};

pub fn init_interaction(
    event: Value,
    client_id: i32,
    db: &PostgresqlClient,
) -> Result<Interaction, EngineError> {

    let e = encrypt_data(&event)?;

    let new_interaction = NewInteraction{client_id, success: false, event: &e};

    let instruction: Interaction = diesel::insert_into(interactions::table)
        .values(&new_interaction)
        .get_result(&db.client)
        .expect("Error creating instruction");

    Ok(instruction)
}

use  diesel::pg::upsert::excluded;

pub fn update_interaction(
    interaction_id: i32,
    success: bool,
    // client: &Client,
    db: &PostgresqlClient,
) -> Result<(), EngineError> {

    diesel::update(
        interactions::table.filter(interactions::id.eq(interaction_id))
    )
    .set(interactions::success.eq(&success))
    .get_result::<Interaction>(&db.client);

    Ok(())
}

pub fn delete_user_interactions(
    client_id: i32, db: &PostgresqlClient
) -> Result<(), EngineError> {

    diesel::delete(interactions::table.filter(
        interactions::client_id.eq(client_id))
    ).get_result::<Interaction>(&db.client);

    Ok(())
}
