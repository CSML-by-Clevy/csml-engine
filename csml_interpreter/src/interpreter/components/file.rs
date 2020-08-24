pub fn add_file(map: &mut serde_json::Map<String, serde_json::Value>) {
    map.insert(
        "File".to_owned(),
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
