
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Action {
    m_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Content {
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    action: Action,
    content: Content,
}

#[derive(Serialize, Deserialize, Debug)]
struct RootInterface {
    remember: Option< Vec<String> >,
    message: Vec<Message>,
    next_step: Option<String>,
}

pub fn test_json()
{
    let point = RootInterface{ 
        remember: Option::None,
        message:
            vec![
                Message {
                    action: Action{ m_type : "say".to_owned()},
                    content: Content{text : "text".to_owned()},
                }
            ],
        next_step: Option::None,
    };

    let serialized = serde_json::to_string(&point).unwrap();
    println!("serialized = {}", serialized);
    let deserialized: RootInterface = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}