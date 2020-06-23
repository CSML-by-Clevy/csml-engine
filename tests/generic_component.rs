mod support;

use csmlinterpreter::data::context::ContextJson;
use csmlinterpreter::data::csml_bot::CsmlBot;
use csmlinterpreter::data::csml_flow::CsmlFlow;
use csmlinterpreter::data::event::Event;
use csmlinterpreter::data::MessageData;
use csmlinterpreter::interpret;

use crate::support::tools::message_to_json_value;
use crate::support::tools::read_file;

use serde_json::Value;

const DEFAULT_ID_NAME: &str = "id";
const DEFAULT_FLOW_NAME: &str = "default";
const DEFAULT_BOT_NAME: &str = "my_bot";

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTION
////////////////////////////////////////////////////////////////////////////////

fn format_message(
    event: Event,
    context: ContextJson,
    vector: &[&str],
    header: serde_json::Value,
) -> MessageData {
    let default_content = read_file(vector[0].to_string()).unwrap();
    let default_flow = CsmlFlow::new(DEFAULT_ID_NAME, "default", &default_content, Vec::default());

    let bot = CsmlBot::new(
        DEFAULT_ID_NAME,
        DEFAULT_BOT_NAME,
        None,
        vec![default_flow],
        header,
        DEFAULT_FLOW_NAME,
    );

    interpret(bot, context, event, None)
}

////////////////////////////////////////////////////////////////////////////////
// TEST FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
// VALID COMPONENT DEFAULT
////////////////////////////////////////////////////////////////////////////////

#[test]
fn empty() {
    let data = r#"{"memories":[], "messages":[
	{
		"content": {},
		"content_type": "button"
	}
	]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "without_argument",
            DEFAULT_FLOW_NAME,
        ),
        &vec!["CSML/basic_test/generic_component.csml"],
        serde_json::json!({"Button": {}}),
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2);
}

#[test]
fn default() {
    let data = r#"{"memories":[], "messages":[
	{
		"content": {"title": {}},
		"content_type": "button"
	}
	]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "without_argument",
            DEFAULT_FLOW_NAME,
        ),
        &vec!["CSML/basic_test/generic_component.csml"],
        serde_json::json!({
            "Button": {
                "params": [
                    {
                        "title": {
                            "required": false,
                            "type": "Object",
                            "default_value": [
                            ]
                        }
                    }
                ]
            }
        }),
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2);
}

#[test]
fn test_all() {
    let data = r#"{"memories":[], "messages":[
        {
            "content": {
                "foo": {
                    "param_0": "foo",
                    "Hello": "World",
                    "Goodbye": "World",
                    "Morning": "World"
                },
                "bar": {
                    "Goodbye": "World",
                    "Morning": "World"
                },
                "baz": {
                    "Goodbye": "World",
                    "Morning": "World"
                }
            },
            "content_type": "button"
        }
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "with_argument",
            DEFAULT_FLOW_NAME,
        ),
        &vec!["CSML/basic_test/generic_component.csml"],
        serde_json::json!({
            "Button": {
                "params": [
                    {
                        "foo": {
                            "required": true,
                            "type": "Object",
                            "default_value": [
                                {"$_set": "Hello"}
                            ],
                            "add_value": [
                                {"$_set": {"Hello": "World"}},
                                {"$_get": "bar"}
                            ]
                        }
                    },
                    {
                        "bar": {
                            "type": "Object",
                            "default_value": [
                                {"$_get": "baz"},
                                {"$_get": "baz"}
                            ],
                            "add_value": [
                                {"$_get": "baz"}
                            ]
                        }
                    },
                    {
                        "baz": {
                            "required": false,
                            "type": "Object",
                            "default_value": [
                                {"$_set": {"Goodbye": "World"}},

                            ],
                            "add_value": [
                                {"$_set": {"Goodbye": "World"}},
                                {"$_set": {"Morning": "World"}}
                            ]
                        }
                    }
                ]
            }
        }),
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2);
}

#[test]
fn default_set() {
    let data = r#"{"memories":[], "messages":[
	{
		"content": {"title": {"hello": "world"}},
		"content_type": "button"
	}
	]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "without_argument",
            DEFAULT_FLOW_NAME,
        ),
        &vec!["CSML/basic_test/generic_component.csml"],
        serde_json::json!({
            "Button": {
                "params": [
                    {
                        "title": {
                            "required": false,
                            "type": "Object",
                            "default_value": [
                                {"$_set": {"hello": "world"}},
                            ]
                        }
                    }
                ]
            }
        }),
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2);
}

#[test]
fn default_get() {
    let data = r#"{"memories":[], "messages":[
	{
		"content": {
            "title": {"hello": "world"},
            "payload": {"hello": "world"}
        },
		"content_type": "button"
	}
	]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "without_argument",
            DEFAULT_FLOW_NAME,
        ),
        &vec!["CSML/basic_test/generic_component.csml"],
        serde_json::json!({
            "Button": {
                "params": [
                    {
                        "title": {
                            "required": false,
                            "type": "Object",
                            "default_value": [
                                {"$_get": "payload"}
                            ]
                        }
                    },
                    {
                        "payload": {
                            "required": false,
                            "type": "Object",
                            "default_value": [
                                {"$_set": {"hello": "world"}},
                            ]
                        }
                    }
                ]
            }
        }),
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2);
}

#[test]
fn default_multiple_get() {
    let data = r#"{"memories":[], "messages":[
	{
		"content": {
            "title": {
                "hello": "world",
                "hello": "world"
            },
            "payload": {"hello": "world"}
        },
		"content_type": "button"
	}
	]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "without_argument",
            DEFAULT_FLOW_NAME,
        ),
        &vec!["CSML/basic_test/generic_component.csml"],
        serde_json::json!({
            "Button": {
                "params": [
                    {
                        "title": {
                            "required": false,
                            "type": "Object",
                            "default_value": [
                                {"$_get": "payload"},
                                {"$_get": "payload"}
                            ]
                        }
                    },
                    {
                        "payload": {
                            "required": false,
                            "type": "Object",
                            "default_value": [
                                {"$_set": {"hello": "world"}},
                            ]
                        }
                    }
                ]
            }
        }),
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2);
}

#[test]
fn default_add_value() {
    let data = r#"{"memories":[], "messages":[
	{
		"content": {"title": {
            "hello": "world"
        }},
		"content_type": "button"
	}
	]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "without_argument",
            DEFAULT_FLOW_NAME,
        ),
        &vec!["CSML/basic_test/generic_component.csml"],
        serde_json::json!({
            "Button": {
                "params": [
                    {
                        "title": {
                            "required": false,
                            "type": "Object",
                            "default_value": [
                            ],
                            "add_value": [
                                {"$_set": {"hello": "world"}},
                            ]
                        }
                    }
                ]
            }
        }),
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2);
}

#[test]
fn default_add_value_empty() {
    let data = r#"
    {
        "memories":[],
        "messages":[
            {
                "content": {"title": {}},
                "content_type": "button"
            }
        ]
    }"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "without_argument",
            DEFAULT_FLOW_NAME,
        ),
        &vec!["CSML/basic_test/generic_component.csml"],
        serde_json::json!({
            "Button": {
                "params": [
                    {
                        "title": {
                            "type": "Object",
                            "default_value": [
                            ],
                            "add_value": [
                            ]
                        }
                    }
                ]
            }
        }),
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2);
}

////////////////////////////////////////////////////////////////////////////////
// VALID COMPONENT PARAMETERS
////////////////////////////////////////////////////////////////////////////////

#[test]
fn parameter() {
    let data = r#"{
        "memories":[], "messages":[
        {
            "content": {
                "foo": {"param_0": "foo"}
            },
            "content_type": "button"
        }
        ]
    }"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "with_argument",
            DEFAULT_FLOW_NAME,
        ),
        &vec!["CSML/basic_test/generic_component.csml"],
        serde_json::json!({
            "Button": {
                "params": [
                    {
                        "foo": {
                            "required": true,
                            "type": "Object",
                            "default_value": [
                            ],
                            "add_value": [
                            ]
                        }
                    }
                ]
            }
        }),
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2);
}

#[test]
fn parameter_multiple() {
    let data = r#"
        {"memories":[], "messages":[
        {
            "content": {
                "foo": {
                    "param_1": "bar",
                    "Hello": 42
                },
                "bar": {"param_0": "foo"},
                "baz": {
                    "param_1": "bar",
                    "Hello": 42
                }
            },
            "content_type": "button"
        }
        ]
    }"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "with_argument",
            DEFAULT_FLOW_NAME,
        ),
        &vec!["CSML/basic_test/generic_component.csml"],
        serde_json::json!({
            "Button": {
                "params": [
                    {
                        "foo": {
                            "type": "Object",
                            "default_value": [
                                {"$_get": "baz"}
                            ],
                            "add_value": [
                            ]
                        }
                    },
                    {
                        "bar": {
                            "required": true,
                            "type": "Object",
                            "default_value": [
                            ],
                            "add_value": [
                            ]
                        }
                    },
                    {
                        "baz": {
                            "required": true,
                            "type": "Object",
                            "default_value": [
                            ],
                            "add_value": [
                                {"$_set": {"Hello": 42}}
                            ]
                        }
                    }
                ]
            }
        }),
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2);
}

////////////////////////////////////////////////////////////////////////////////
// INVALID COMPONENT
////////////////////////////////////////////////////////////////////////////////

#[test]
fn unknown_component() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "unknown_component",
            DEFAULT_FLOW_NAME,
        ),
        &vec!["CSML/basic_test/generic_component.csml"],
        serde_json::json!({
            "Button": {
                "params": [
                    {
                        "foo": {
                            "type": "Object",
                            "default_value": [
                            ],
                            "add_value": [
                            ]
                        }
                    }
                ]
            }
        }),
    );

    if msg.messages[0].content_type == "error" {
        return assert!(true);
    }

    assert!(false);
}

#[test]
fn missing_type() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "unknown_component",
            DEFAULT_FLOW_NAME,
        ),
        &vec!["CSML/basic_test/generic_component.csml"],
        serde_json::json!({
            "Button": {
                "params": [
                    {
                        "foo": {
                            "default_value": [
                            ],
                            "add_value": [
                            ]
                        }
                    }
                ]
            }
        }),
    );

    if msg.messages[0].content_type == "error" {
        return assert!(true);
    }

    assert!(false);
}

#[test]
fn illegal_operation_default() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "unknown_component",
            DEFAULT_FLOW_NAME,
        ),
        &vec!["CSML/basic_test/generic_component.csml"],
        serde_json::json!({
            "Button": {
                "params": [
                    {
                        "foo": {
                            "type": "Object",
                            "default_value": [
                                {"$_set": "Hello"}
                            ],
                            "add_value": [
                            ]
                        }
                    }
                ]
            }
        }),
    );

    if msg.messages[0].content_type == "error" {
        return assert!(true);
    }

    assert!(false);
}

#[test]
fn illegal_operation_add() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "unknown_component",
            DEFAULT_FLOW_NAME,
        ),
        &vec!["CSML/basic_test/generic_component.csml"],
        serde_json::json!({
            "Button": {
                "params": [
                    {
                        "foo": {
                            "type": "Object",
                            "default_value": [
                            ],
                            "add_value": [
                                {"$_set": "Hello"}
                            ]
                        }
                    }
                ]
            }
        }),
    );

    if msg.messages[0].content_type == "error" {
        return assert!(true);
    }

    assert!(false);
}

#[test]
fn illegal_operation_parameter() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "unknown_component",
            DEFAULT_FLOW_NAME,
        ),
        &vec!["CSML/basic_test/generic_component.csml"],
        serde_json::json!({
            "Button": {
                "params": [
                    {
                        "foo": {
                            "required": true,
                            "type": "String",
                            "default_value": [
                            ],
                            "add_value": [
                            ]
                        }
                    }
                ]
            }
        }),
    );

    if msg.messages[0].content_type == "error" {
        return assert!(true);
    }

    assert!(false);
}

#[test]
fn circular_dependencie_on_other_key_default() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "with_argument",
            DEFAULT_FLOW_NAME,
        ),
        &vec!["CSML/basic_test/generic_component.csml"],
        serde_json::json!({
            "Button": {
                "params": [
                    {
                        "foo": {
                            "type": "Object",
                            "default_value": [
                                {"$_get": "bar"}
                            ],
                            "add_value": [
                            ]
                        }
                    },
                    {
                        "bar": {
                            "type": "Object",
                            "default_value": [
                                {"$_get": "foo"}
                            ],
                            "add_value": [
                            ]
                        }
                    }
                ]
            }
        }),
    );

    if msg.messages[0].content_type == "error" {
        return assert!(true);
    }

    assert!(false);
}

#[test]
fn circular_dependencie_on_self_default() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "with_argument",
            DEFAULT_FLOW_NAME,
        ),
        &vec!["CSML/basic_test/generic_component.csml"],
        serde_json::json!({
            "Button": {
                "params": [
                    {
                        "foo": {
                            "type": "Object",
                            "default_value": [
                                {"$_get": "foo"}
                            ],
                            "add_value": [
                            ]
                        }
                    },
                ]
            }
        }),
    );

    if msg.messages[0].content_type == "error" {
        return assert!(true);
    }

    assert!(false);
}

#[test]
fn circular_dependencie_on_other_key_add() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "with_argument",
            DEFAULT_FLOW_NAME,
        ),
        &vec!["CSML/basic_test/generic_component.csml"],
        serde_json::json!({
            "Button": {
                "params": [
                    {
                        "foo": {
                            "type": "Object",
                            "default_value": [
                            ],
                            "add_value": [
                                {"$_get": "bar"}
                            ]
                        }
                    },
                    {
                        "bar": {
                            "type": "Object",
                            "default_value": [
                            ],
                            "add_value": [
                                {"$_get": "foo"}
                            ]
                        }
                    }
                ]
            }
        }),
    );

    if msg.messages[0].content_type == "error" {
        return assert!(true);
    }

    assert!(false);
}

#[test]
fn circular_dependencie_on_self_add() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "with_argument",
            DEFAULT_FLOW_NAME,
        ),
        &vec!["CSML/basic_test/generic_component.csml"],
        serde_json::json!({
            "Button": {
                "params": [
                    {
                        "foo": {
                            "type": "Object",
                            "default_value": [
                            ],
                            "add_value": [
                                {"$_get": "foo"}
                            ]
                        }
                    },
                ]
            }
        }),
    );

    if msg.messages[0].content_type == "error" {
        return assert!(true);
    }

    assert!(false);
}

#[test]
fn missing_parameter() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "without_argument",
            DEFAULT_FLOW_NAME,
        ),
        &vec!["CSML/basic_test/generic_component.csml"],
        serde_json::json!({
            "Button": {
                "params": [
                    {
                        "foo": {
                            "required": true,
                            "type": "Object",
                            "default_value": [
                            ],
                            "add_value": [
                            ]
                        }
                    }
                ]
            }
        }),
    );

    if msg.messages[0].content_type == "error" {
        return assert!(true);
    }

    assert!(false);
}
