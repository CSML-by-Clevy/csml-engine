pub fn add_image(map: &mut serde_json::Map<String, serde_json::Value>) {
    map.insert(
        "Image".to_owned(), 
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