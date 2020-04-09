mod support;

use csmlinterpreter::data::context::ContextJson;
use csmlinterpreter::data::event::Event;

use crate::support::tools::format_message;
use crate::support::tools::message_to_json_value;

use serde_json::Value;

#[test]
fn http_http_0() {
    let data = r#"{
        "memories":[
            {
                "key":"http", "value":{
                    "body":{},
                    "header":{
                        "accept":"application/json,text/*",
                        "content-type":"application/json"
                    },
                    "method":"get",
                    "query":{},
                    "url":"https://clevy.io"
                }
            }
        ],
        "messages":[
            {
                "content":{
                    "body": {},
                    "header":{
                        "accept":"application/json,text/*",
                        "content-type":"application/json"
                    },
                    "method":"get",
                    "query":{},
                    "url":"https://clevy.io"
                },
                "content_type":"http"
            }
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "http_0",
            "flow",
        ),
        "CSML/basic_test/stdlib/http.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn http_get_0() {
    let data = r#"{
        "memories":[
            {
                "key":"http", "value":{
                    "body":{},
                    "header":{
                        "accept":"application/json,text/*",
                        "content-type":"application/json"
                    },
                    "method":"get",
                    "query":{},
                    "url":"https://clevy.io"
                }
            }
        ],
        "messages":[
            {
                "content":{
                    "body": {},
                    "header":{
                        "accept":"application/json,text/*",
                        "content-type":"application/json"
                    },
                    "method":"get",
                    "query":{},
                    "url":"https://clevy.io"
                },
                "content_type":"http"
            },
            {
                "content":{
                    "body": {},
                    "header":{
                        "accept":"application/json,text/*",
                        "content-type":"application/json"
                    },
                    "method":"get",
                    "query":{},
                    "url":"https://clevy.io"
                },
                "content_type":"http"
            }
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "get_0",
            "flow",
        ),
        "CSML/basic_test/stdlib/http.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn http_set_0() {
    let data = r#"{
        "memories":[
            {
                "key":"http", "value":{
                    "body":{},
                    "header":{
                        "accept":"application/json,text/*",
                        "content-type":"application/json"
                    },
                    "method":"get",
                    "query":{},
                    "url":"https://clevy.io"
                }
            }
        ],
        "messages":[
            {
                "content":{
                    "body": {},
                    "header":{
                        "accept":"application/json,text/*",
                        "content-type":"application/json",
                        "hello":"world"
                    },
                    "method":"get",
                    "query":{},
                    "url":"https://clevy.io"
                },
                "content_type":"http"
            },
            {
                "content":{
                    "body": {},
                    "header":{
                        "accept":"application/json,text/*",
                        "content-type":"application/json"
                    },
                    "method":"get",
                    "query":{},
                    "url":"https://clevy.io"
                },
                "content_type":"http"
            }
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "set_0",
            "flow",
        ),
        "CSML/basic_test/stdlib/http.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn http_query_0() {
    let data = r#"{
        "memories":[
            {
                "key":"http", "value":{
                    "body":{},
                    "header":{
                        "accept":"application/json,text/*",
                        "content-type":"application/json"
                    },
                    "method":"get",
                    "query":{},
                    "url":"https://clevy.io"
                }
            }
        ],
        "messages":[
            {
                "content":{
                    "body": {},
                    "header":{
                        "accept":"application/json,text/*",
                        "content-type":"application/json"
                    },
                    "method":"get",
                    "query":{
                        "hello":"world"
                    },
                    "url":"https://clevy.io"
                },
                "content_type":"http"
            },
            {
                "content":{
                    "body": {},
                    "header":{
                        "accept":"application/json,text/*",
                        "content-type":"application/json"
                    },
                    "method":"get",
                    "query":{},
                    "url":"https://clevy.io"
                },
                "content_type":"http"
            }
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "query_0",
            "flow",
        ),
        "CSML/basic_test/stdlib/http.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn http_delete_0() {
    let data = r#"{
        "memories":[
            {
                "key":"http", "value":{
                    "body":{},
                    "header":{
                        "accept":"application/json,text/*",
                        "content-type":"application/json"
                    },
                    "method":"get",
                    "query":{},
                    "url":"https://clevy.io"
                }
            }
        ],
        "messages":[
            {
                "content":{
                    "body": {
                        "hello":"world"
                    },
                    "header":{
                        "accept":"application/json,text/*",
                        "content-type":"application/json"
                    },
                    "method":"delete",
                    "query":{},
                    "url":"https://clevy.io"
                },
                "content_type":"http"
            },
            {
                "content":{
                    "body":{},
                    "header":{
                        "accept":"application/json,text/*",
                        "content-type":"application/json"
                    },
                    "method":"get",
                    "query":{},
                    "url":"https://clevy.io"
                },
                "content_type":"http"
            }
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "delete_0",
            "flow",
        ),
        "CSML/basic_test/stdlib/http.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn http_put_0() {
    let data = r#"{
        "memories":[
            {
                "key":"http", "value":{
                    "body":{},
                    "header":{
                        "accept":"application/json,text/*",
                        "content-type":"application/json"
                    },
                    "method":"get",
                    "query":{},
                    "url":"https://clevy.io"
                }
            }
        ],
        "messages":[
            {
                "content":{
                    "body": {
                        "hello":"world"
                    },
                    "header":{
                        "accept":"application/json,text/*",
                        "content-type":"application/json"
                    },
                    "method":"put",
                    "query":{},
                    "url":"https://clevy.io"
                },
                "content_type":"http"
            },
            {
                "content":{
                    "body": {},
                    "header":{
                        "accept":"application/json,text/*",
                        "content-type":"application/json"
                    },
                    "method":"get",
                    "query":{},
                    "url":"https://clevy.io"
                },
                "content_type":"http"
            }
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "put_0",
            "flow",
        ),
        "CSML/basic_test/stdlib/http.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn http_patch_0() {
    let data = r#"{
        "memories":[
            {
                "key":"http", "value":{
                    "body":{},
                    "header":{
                        "accept":"application/json,text/*",
                        "content-type":"application/json"
                    },
                    "method":"get",
                    "query":{},
                    "url":"https://clevy.io"
                }
            }
        ],
        "messages":[
            {
                "content":{
                    "body": {
                        "hello":"world"
                    },
                    "header":{
                        "accept":"application/json,text/*",
                        "content-type":"application/json"
                    },
                    "method":"patch",
                    "query":{},
                    "url":"https://clevy.io"
                },
                "content_type":"http"
            },
            {
                "content":{
                    "body": {},
                    "header":{
                        "accept":"application/json,text/*",
                        "content-type":"application/json"
                    },
                    "method":"get",
                    "query":{},
                    "url":"https://clevy.io"
                },
                "content_type":"http"
            }
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "patch_0",
            "flow",
        ),
        "CSML/basic_test/stdlib/http.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn http_post_0() {
    let data = r#"{
        "memories":[
            {
                "key":"http", "value":{
                    "body":{},
                    "header":{
                        "accept":"application/json,text/*",
                        "content-type":"application/json"
                    },
                    "method":"get",
                    "query":{},
                    "url":"https://clevy.io"
                }
            }
        ],
        "messages":[
            {
                "content":{
                    "body": {
                        "hello":"world"
                    },
                    "header":{
                        "accept":"application/json,text/*",
                        "content-type":"application/json"
                    },
                    "method":"post",
                    "query":{},
                    "url":"https://clevy.io"
                },
                "content_type":"http"
            },
            {
                "content":{
                    "body": {},
                    "header":{
                        "accept":"application/json,text/*",
                        "content-type":"application/json"
                    },
                    "method":"get",
                    "query":{},
                    "url":"https://clevy.io"
                },
                "content_type":"http"
            }
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "post_0",
            "flow",
        ),
        "CSML/basic_test/stdlib/http.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
