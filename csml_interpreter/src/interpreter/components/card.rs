pub fn add_card(map: &mut serde_json::Map<String, serde_json::Value>) {
    map.insert(
        "Card".to_owned(),
        serde_json::json!(
            {
                "params": [
                    {
                        "title": {
                            "required": true,
                            "type": "String"
                        }
                    },
                    {
                        "buttons": {
                            "required": false,
                            "type": "Array"
                        }
                    }
                ]
            }
        ),
    );
}
