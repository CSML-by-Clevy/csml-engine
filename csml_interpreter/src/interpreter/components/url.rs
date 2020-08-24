pub fn add_url(map: &mut serde_json::Map<String, serde_json::Value>) {
    map.insert(
        "Url".to_owned(),
        serde_json::json!(
            {
                "params": [
                    {
                        "url": {
                            "required": true,
                            "type": "String"
                        }
                    },
                    {
                        "text": {
                            "required": false,
                            "type": "String",
                            "default_value": [
                                {"$_get": "url"}
                            ]
                        }
                    },
                    {
                        "title": {
                            "required": false,
                            "type": "String",
                            "default_value": [
                                {"$_get": "url"}
                            ]
                        }
                    }
                ]
            }
        ),
    );
}
