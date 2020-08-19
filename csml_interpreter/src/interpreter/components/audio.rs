pub fn add_audio(map: &mut serde_json::Map<String, serde_json::Value>) {
    map.insert(
        "Audio".to_owned(),
        serde_json::json!(
            {
                "params": [
                    {
                        "url": {
                            "required": true,
                            "type": "String"
                        }
                    }
                ]
            }
        )
    );
}