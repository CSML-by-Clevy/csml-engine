pub fn add_question(map: &mut serde_json::Map<String, serde_json::Value>) {
    map.insert(
        "Question".to_owned(), 
        serde_json::json!(
            {
                "params": [
                    {
                        "title": {
                            "required": false,
                            "type": "String",
                            "default_value": [
                            ],
                            "add_value": [
                            ]
                        }
                    },
                    {
                        "buttons": {
                            "required": true,
                            "type": "Array"
                        }
                    }
                ]
            }
        )
    );
}