use csml_engine::get_client_messages;
use csml_interpreter::data::Client;

use chrono::prelude::*;

fn main() {
    let client = Client {
        user_id: "alexis".to_owned(),
        bot_id: "botid".to_owned(),
        channel_id: "some-channel-id".to_owned(),
    };

    let messages = get_client_messages(&client, None, None, None, None).unwrap();

    println!(
        "msg nbr => {}",
        messages["messages"].as_array().unwrap().len()
    );

    // DateTime::format_with_items(&self, items)
    let start = Utc.ymd(2022, 3, 23).and_hms(14, 19, 50); // `2014-07-08T09:10:11Z`
                                                          // dt.timestamp_millis()

    let messages = get_client_messages(&client, None, None, Some(start.timestamp()), None).unwrap();

    println!("=> {:#?}", messages["messages"]);

    println!(
        "msg nbr => {}",
        messages["messages"].as_array().unwrap().len()
    );
}
