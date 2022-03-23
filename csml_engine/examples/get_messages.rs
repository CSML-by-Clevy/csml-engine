use csml_engine::{
    data::{BotOpt, CsmlRequest},
    start_conversation, delete_expired_data, get_client_messages,
};
use csml_interpreter::{
    data::{csml_bot::CsmlBot, csml_flow::CsmlFlow, Client},
    load_components,
};
use serde_json::json;
use std::fs::File;
use std::io::prelude::*;
use std::io::stdin;

use chrono::prelude::*;
use chrono::offset::LocalResult;

// "2022-02-22T12:19:24.596Z",
// "2022-02-22T11:39:36.335Z",
// "2022-02-22T10:48:17.821Z",

fn main() {
    let client = Client {
        user_id: "alexis".to_owned(),
        bot_id: "botid".to_owned(),
        channel_id: "some-channel-id".to_owned(),
    };


    let messages = get_client_messages(
        &client,
        None,
        None,
        None,
        None
    ).unwrap();

    // println!("=> {:#?}", messages["messages"]);

    println!("msg nbr => {}", messages["messages"].as_array().unwrap().len());

    // DateTime::format_with_items(&self, items)
    let start = Utc.ymd(2022, 3, 23).and_hms(14, 19,50); // `2014-07-08T09:10:11Z`
    // dt.timestamp_millis()

    let messages = get_client_messages(
        &client,
        None,
        None,
        Some(start.timestamp()),
        None
    ).unwrap();

    println!("=> {:#?}", messages["messages"]);

    println!("msg nbr => {}", messages["messages"].as_array().unwrap().len());

    // println!()

}
