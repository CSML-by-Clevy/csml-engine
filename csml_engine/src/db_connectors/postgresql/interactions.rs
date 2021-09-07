use diesel::{RunQueryDsl, ExpressionMethods, QueryDsl};

use serde_json::Value;

use crate::{
    Client, encrypt::encrypt_data,
    EngineError, PostgresqlClient
};

use super::{
    models::{NewInteraction, Interaction},
    schema::csml_interactions
};

pub fn init_interaction(
    event: Value,
    client: &Client,
    db: &PostgresqlClient,
) -> Result<String, EngineError> {

    let e = encrypt_data(&event)?;

    let new_interaction = NewInteraction {
        id: uuid::Uuid::new_v4(),
        bot_id: &client.bot_id,
        channel_id: &client.channel_id,
        user_id: &client.user_id,
        success: false,
        event: &e
    };

    let interaction: Interaction = diesel::insert_into(csml_interactions::table)
        .values(&new_interaction)
        .get_result(&db.client)?;

    Ok(interaction.id.to_string())
}

pub fn update_interaction(
    interaction_id: &str,
    success: bool,
    client: &Client,
    db: &PostgresqlClient,
) -> Result<(), EngineError> {

    let id: uuid::Uuid = uuid::Uuid::parse_str(interaction_id).unwrap();

    diesel::update(
        csml_interactions::table
        .filter(csml_interactions::bot_id.eq(&client.bot_id))
        .filter(csml_interactions::channel_id.eq(&client.channel_id))
        .filter(csml_interactions::user_id.eq(&client.user_id))
        .filter(csml_interactions::id.eq(&id))
    )
    .set(csml_interactions::success.eq(&success))
    .get_result::<Interaction>(&db.client)?;

    Ok(())
}

pub fn delete_user_interactions(
    client: &Client,
    db: &PostgresqlClient
) -> Result<(), EngineError> {

    diesel::delete(
        csml_interactions::table
        .filter(csml_interactions::bot_id.eq(&client.bot_id))
        .filter(csml_interactions::channel_id.eq(&client.channel_id))
        .filter(csml_interactions::user_id.eq(&client.user_id))
    ).get_result::<Interaction>(&db.client).ok();

    Ok(())
}

pub fn delete_all_bot_data(
    bot_id: &str,
    db: &PostgresqlClient,
) -> Result<(), EngineError> {
    diesel::delete(
        csml_interactions::table
        .filter(csml_interactions::bot_id.eq(bot_id))
    ).execute(&db.client).ok();

    Ok(())
}