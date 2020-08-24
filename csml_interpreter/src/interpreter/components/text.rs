pub fn add_text(map: &mut serde_json::Map<String, serde_json::Value>) {
    map.insert(
        "Text".to_owned(),
        serde_json::json!(
            {
                "params": [
                    {
                        "text": {
                            "required": true,
                            "type": "String"
                        }
                    }
                ]
            }
        ),
    );
}
