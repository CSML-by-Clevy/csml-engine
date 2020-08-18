pub fn add_button(map: &mut serde_json::Map<String, serde_json::Value>) {
    map.insert(
        "Button".to_owned(),
        serde_json::json!(
            {
                "params": [
                    {
                        "title": {
                            "required": true,
                            "type": "String"
                        }
                    },
                    {
                        "payload": {
                            "required": false,
                            "type": "String",
                            "default_value": [
                                {"$_get": "title"}
                            ]
                        }
                    },
                    {
                        "accepts": {
                            "required": false,
                            "type": "Array",
                            "default_value": [
                            ],
                            "add_value": [
                                {"$_get": "title" },
                                {"$_get": "payload" } 
                            ]
                        }
                    }
                ]
            }
        )
    );
}