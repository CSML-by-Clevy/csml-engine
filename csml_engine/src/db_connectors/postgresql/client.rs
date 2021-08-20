use std::env;

use diesel::{RunQueryDsl, ExpressionMethods, QueryDsl};
use diesel::{insert_into};

use crate::PostgresqlClient;


use super::{
    models::{NewClient, Client},
    schema::client
};

pub fn create_client<'a>(
    bot_id: &'a str,
    channel_id: &'a str,
    user_id: &'a str,
    db: &PostgresqlClient,
) -> Client {
    let new_client = NewClient{bot_id, channel_id, user_id};

    diesel::insert_into(client::table)
        .values(&new_client)
        .get_result(&db.client)
        .expect("Error creating client")
}

pub fn find_client<'a>(
    bot_id: &'a str,
    channel_id: &'a str,
    user_id: &'a str,
    db: &PostgresqlClient,
) {

    let client = client::table
    .filter(client::bot_id.eq(bot_id))
    .filter(client::channel_id.eq(channel_id))
    .filter(client::user_id.eq(user_id));
}