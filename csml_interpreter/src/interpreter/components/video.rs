pub fn add_video(map: &mut serde_json::Map<String, serde_json::Value>) {
    map.insert(
        "Video".to_owned(),
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
        ),
    );
}
