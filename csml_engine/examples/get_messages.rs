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
    // 2022-04-08T13:52:29.982Z
    let start = Utc.ymd(2022, 4, 8).and_hms(11, 55, 50); // `2014-07-08T09:10:11Z`
                                                         // dt.timestamp_millis()

    // let messages = get_client_messages(&client, None, None, None, None).unwrap();
    let messages =
        get_client_messages(&client, Some(1), None, Some(start.timestamp()), None).unwrap();

    println!("=> {:#?}", messages);

    println!(
        "msg nbr => {}",
        messages["messages"].as_array().unwrap().len()
    );
}
