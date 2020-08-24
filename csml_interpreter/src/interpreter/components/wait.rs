pub fn add_wait(map: &mut serde_json::Map<String, serde_json::Value>) {
    map.insert(
        "Wait".to_owned(),
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
