#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use csml_interpreter::data::{Message};

    use crate::{
        Client, ConversationInfo, Context,
        init_db, db_connectors::*, make_migrations
    };

    fn get_client() -> Client {
        Client {
            user_id: "alexis".to_owned(),
            bot_id: "botid".to_owned(),
            channel_id: "some-channel-id".to_owned(),
        }
    }

    fn get_context() -> Context {
        Context {
            current: HashMap::new(),
            metadata: HashMap::new(),
            api_info: None,
            hold: None,
            step: "start".to_owned(),
            flow: "Default".to_owned(),
        }
    }

    fn get_conversation_info(
        messages: Vec<Message>,
        conversation_id: String,
        db: Database
    ) -> ConversationInfo {
        ConversationInfo {
            request_id: "1234".to_owned(),
            conversation_id,
            callback_url: None,
            client: get_client(),
            context: get_context(),
            metadata: serde_json::json!({}),
            messages,
            ttl: None,
            low_data: false,
            db,
        }
    }

    fn gen_message(message: &str) -> serde_json::Value {
        serde_json::json!({
            "content_type": "text",
            "content": { "text": message},
        })
    }

    #[test]
    fn ok_messages() {
        make_migrations().unwrap_or({});

        let client = get_client();
        let mut db = init_db().unwrap();
        user::delete_client(&client, &mut db).unwrap();

        let c_id = conversations::create_conversation("Default", "start", &client, None, &mut db).unwrap();

        let msgs = vec![
            gen_message("1"),
            gen_message("2"),
            gen_message("3"),
            gen_message("4"),
        ];

        let mut data = get_conversation_info(
            vec![],
            c_id,
            db
        );

        messages::add_messages_bulk(
            &mut data,
            msgs,
            0,
            "SEND",
        ).unwrap();

        let response = messages::get_client_messages(
            &client,
            &mut data.db,
            Some(2),
            None
        ).unwrap();

        let received_msgs: Vec<serde_json::Value> = serde_json::from_value(response["messages"].clone()).unwrap();

        assert_eq!(2, received_msgs.len());

        assert_eq!("4", received_msgs[0]["payload"]["content"]["text"].as_str().unwrap());
        assert_eq!("3", received_msgs[1]["payload"]["content"]["text"].as_str().unwrap());

        user::delete_client(&client, &mut data.db).unwrap();

        let response = messages::get_client_messages(
            &client,
            &mut data.db,
            Some(2),
            None
        ).unwrap();

        let received_msgs: Vec<serde_json::Value> = serde_json::from_value(response["messages"].clone()).unwrap();
        assert_eq!(0, received_msgs.len());
    }

    #[test]
    fn ok_conversation() {
        make_migrations().unwrap_or({});

        let client = get_client();
        let mut db = init_db().unwrap();

        user::delete_client(&client, &mut db).unwrap();

        conversations::create_conversation("Default", "start", &client, None, &mut db).unwrap();
        conversations::create_conversation("Default", "start", &client, None, &mut db).unwrap();
        conversations::create_conversation("Default", "start", &client, None, &mut db).unwrap();

        let response = conversations::get_client_conversations(
            &client, &mut db,
            Some(6),
            None
        ).unwrap();

        let conversations: Vec<serde_json::Value> = serde_json::from_value(response["conversations"].clone()).unwrap();

        assert_eq!(conversations.len(), 3);

        user::delete_client(&client, &mut db).unwrap();

        let response = conversations::get_client_conversations(
            &client, &mut db,
            Some(6),
            None
        ).unwrap();

        let conversations: Vec<serde_json::Value> = serde_json::from_value(response["conversations"].clone()).unwrap();
        assert_eq!(conversations.len(), 0);
    }

    #[test]
    fn ok_memories() {
        make_migrations().unwrap_or({});

        let client = get_client();
        let mut db = init_db().unwrap();

        user::delete_client(&client, &mut db).unwrap();

        let mems = vec![
            ("key".to_owned(), serde_json::json!("value")),
            ("random".to_owned(), serde_json::json!(42)),
        ];

        for (key, value) in mems.iter() {
            memories::create_client_memory(&client, key.to_owned(), value.to_owned(), None, &mut db).unwrap();
        }

        let response = memories::internal_use_get_memories(&client, &mut db).unwrap();
        let memories: &serde_json::Map<String, serde_json::Value> = response.as_object().unwrap();

        assert_eq!(memories.len(), 2);

        for (key, value) in mems.iter() {
            assert_eq!(memories.get(key).unwrap(), value);
        }

        user::delete_client(&client, &mut db).unwrap();

        let response = memories::internal_use_get_memories(&client, &mut db).unwrap();
        let memories: &serde_json::Map<String, serde_json::Value> = response.as_object().unwrap();

        assert_eq!(memories.len(), 0);
    }

    #[test]
    fn ok_memory() {
        make_migrations().unwrap_or({});

        let client = get_client();
        let mut db = init_db().unwrap();

        user::delete_client(&client, &mut db).unwrap();

        let mems = vec![
            ("memory_key".to_owned(), serde_json::json!("value")),
            ("memory".to_owned(), serde_json::json!("tmp")),
            ("memory_key".to_owned(), serde_json::json!("next")),
        ];

        for (key, value) in mems.iter() {
            memories::create_client_memory(&client, key.to_owned(), value.to_owned(), None, &mut db).unwrap();
        }

        let response = memories::internal_use_get_memories(&client, &mut db).unwrap();
        let memories: &serde_json::Map<String, serde_json::Value> = response.as_object().unwrap();

        assert_eq!(memories.len(), 2);

        let mems = vec![
            ("memory".to_owned(), serde_json::json!("tmp")),
            ("memory_key".to_owned(), serde_json::json!("next")),
        ];

        for (key, value) in mems.iter() {
            assert_eq!(memories.get(key).unwrap(), value);
        }

        memories::delete_client_memory(&client, "memory", &mut db).unwrap();

        let response = memories::internal_use_get_memories(&client, &mut db).unwrap();
        let memories: &serde_json::Map<String, serde_json::Value> = response.as_object().unwrap();

        assert_eq!(memories.len(), 1);

        let mems = vec![
            ("memory_key".to_owned(), serde_json::json!("next")),
        ];

        for (key, value) in mems.iter() {
            assert_eq!(memories.get(key).unwrap(), value);
        }
    }

    #[test]
    fn ok_get_memory() {
        make_migrations().unwrap_or({});

        let client = get_client();
        let mut db = init_db().unwrap();

        user::delete_client(&client, &mut db).unwrap();

        let mems = vec![
            ("my_key".to_owned(), serde_json::json!("value")),
            ("random".to_owned(), serde_json::json!("tmp")),
            ("my_key".to_owned(), serde_json::json!("next")),
        ];

        for (key, value) in mems.iter() {
            memories::create_client_memory(&client, key.to_owned(), value.to_owned(), None, &mut db).unwrap();
        }

        let response = memories::get_memory(&client, "my_key", &mut db).unwrap();

        assert_eq!(serde_json::Value::String("next".to_owned()), response["value"]);

        let response = memories::get_memories(&client, &mut db).unwrap();

        match response {
            serde_json::Value::Array(memories) => {
                for memory in memories {
                    let key = memory["key"].as_str().unwrap();
                    if key != "random" && key != "my_key" {
                        panic!("bad memory => {:?}", memory)
                    }
                }
            }
            value => panic!("bad format => {:?}", value)
        }
    }
}
