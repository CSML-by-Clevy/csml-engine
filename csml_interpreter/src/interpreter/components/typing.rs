pub fn add_typing(map: &mut serde_json::Map<String, serde_json::Value>) {
    map.insert(
        "Typing".to_owned(),
        serde_json::json!(
            {
                "params": [
                    {
                        "duration": {
                            "required": true,
                            "type": "Number"
                        }
                    }
                ]
            }
        ),
    );
}
