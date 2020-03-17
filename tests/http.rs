mod support;

use csmlinterpreter::data::{Event, MessageData};
use csmlinterpreter::interpret;
use serde_json::Value;

use support::tools::{gen_context, gen_event, message_to_jsonvalue, read_file};

fn format_message(event: Event, step: &str) -> MessageData {
    let text = read_file("CSML/basic_test/stdlib/http.csml".to_owned()).unwrap();

    let context = gen_context(serde_json::json!({}), serde_json::json!({}));

    interpret(&text, step, context, &event, None)
}

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
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(gen_event(""), "http_0");

    let v1: Value = message_to_jsonvalue(msg);
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
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(gen_event(""), "get_0");

    let v1: Value = message_to_jsonvalue(msg);
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
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(gen_event(""), "set_0");

    let v1: Value = message_to_jsonvalue(msg);
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
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(gen_event(""), "query_0");

    let v1: Value = message_to_jsonvalue(msg);
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
                    "body": {},
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
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(gen_event(""), "delete_0");

    let v1: Value = message_to_jsonvalue(msg);
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
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(gen_event(""), "put_0");

    let v1: Value = message_to_jsonvalue(msg);
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
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(gen_event(""), "patch_0");

    let v1: Value = message_to_jsonvalue(msg);
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
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(gen_event(""), "post_0");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
