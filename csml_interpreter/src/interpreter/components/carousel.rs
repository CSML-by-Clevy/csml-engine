pub fn add_carousel(map: &mut serde_json::Map<String, serde_json::Value>) {
    map.insert(
        "Carousel".to_owned(),
        serde_json::json!(
            {
                "params": [
                    {
                        "cards": {
                            "required": true,
                            "type": "Array"
                        }
                    }
                ]
            }
        ),
    );
}
