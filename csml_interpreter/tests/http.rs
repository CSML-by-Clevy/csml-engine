mod support;

use csml_interpreter::data::context::Context;
use csml_interpreter::data::event::Event;
use std::collections::HashMap;

use crate::support::tools::format_message;
use crate::support::tools::message_to_json_value;

use serde_json::Value;

#[test]
fn http_http_0() {
    let data = r#"{
        "memories":[
            {
                "key":"http",
                "value":{
                    "_content":{
                        "header":{
                            "Accept":"application/json,text/*",
                            "Content-Type":"application/json",
                            "User-Agent": "csml/v1"
                        },
                        "method":"get",
                        "url":"https://clevy.io"
                    },
                    "_content_type":"http"
                }
            }
        ],
        "messages":[
            {
                "content":{
                    "header":{
                        "Accept":"application/json,text/*",
                        "Content-Type":"application/json",
                        "User-Agent": "csml/v1"
                    },
                    "method":"get",
                    "url":"https://clevy.io"
                },
                "content_type":"http"
            }
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(HashMap::new(), HashMap::new(), None, None, "http_0", "flow"),
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
                "key":"http",
                "value":{
                    "_content":{
                        "header":{
                            "Accept":"application/json,text/*",
                            "Content-Type":"application/json",
                            "User-Agent": "csml/v1"
                        },
                        "method":"get",
                        "url":"https://clevy.io"
                    },
                    "_content_type":"http"
                }
            }
        ],
        "messages":[
            {
                "content":{
                    "header":{
                        "Accept":"application/json,text/*",
                        "Content-Type":"application/json",
                        "User-Agent": "csml/v1"
                    },
                    "method":"get",
                    "url":"https://clevy.io"
                },
                "content_type":"http"
            }
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(HashMap::new(), HashMap::new(), None, None, "get_0", "flow"),
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
                    "_content":{
                        "header":{
                            "Accept":"application/json,text/*",
                            "Content-Type":"application/json",
                            "User-Agent": "csml/v1"
                        },
                        "method":"get",
                        "url":"https://clevy.io"
                    },
                    "_content_type":"http"
                }
            }
        ],
        "messages":[
            {
                "content":{
                    "header":{
                        "Accept":"application/json,text/*",
                        "Content-Type":"application/json",
                        "User-Agent": "csml/v1",
                        "hello":"world"
                    },
                    "method":"get",
                    "url":"https://clevy.io"
                },
                "content_type":"http"
            },
            {
                "content":{
                    "header":{
                        "Accept":"application/json,text/*",
                        "Content-Type":"application/json",
                        "User-Agent": "csml/v1"
                    },
                    "method":"get",
                    "url":"https://clevy.io"
                },
                "content_type":"http"
            }
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(HashMap::new(), HashMap::new(), None, None, "set_0", "flow"),
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
                "key":"http",
                "value":{
                    "_content":{
                        "header":{
                            "Accept":"application/json,text/*",
                            "Content-Type":"application/json",
                            "User-Agent": "csml/v1"
                        },
                        "method":"get",
                        "url":"https://clevy.io"
                    },
                    "_content_type":"http"
                }
            }
        ],
        "messages":[
            {
                "content":{
                    "header":{
                        "Accept":"application/json,text/*",
                        "Content-Type":"application/json",
                        "User-Agent": "csml/v1"
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
                    "header":{
                        "Accept":"application/json,text/*",
                        "Content-Type":"application/json",
                        "User-Agent": "csml/v1"
                    },
                    "method":"get",
                    "url":"https://clevy.io"
                },
                "content_type":"http"
            }
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
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
                "key":"http",
                "value":{
                    "_content":{
                        "header":{
                            "Accept":"application/json,text/*",
                            "Content-Type":"application/json",
                            "User-Agent": "csml/v1"
                        },
                        "method":"get",
                        "url":"https://clevy.io"
                    },
                    "_content_type":"http"
                }
            }
        ],
        "messages":[
            {
                "content":{
                    "header":{
                        "Accept":"application/json,text/*",
                        "Content-Type":"application/json",
                        "User-Agent": "csml/v1"
                    },
                    "method":"delete",
                    "url":"https://clevy.io"
                },
                "content_type":"http"
            },
            {
                "content":{
                    "header":{
                        "Accept":"application/json,text/*",
                        "Content-Type":"application/json",
                        "User-Agent": "csml/v1"
                    },
                    "method":"get",
                    "url":"https://clevy.io"
                },
                "content_type":"http"
            }
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
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
                    "_content":{
                        "header":{
                            "Accept":"application/json,text/*",
                            "Content-Type":"application/json",
                            "User-Agent": "csml/v1"
                        },
                        "method":"get",
                        "url":"https://clevy.io"
                    },
                    "_content_type":"http"
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
                        "Accept":"application/json,text/*",
                        "Content-Type":"application/json",
                        "User-Agent": "csml/v1"
                    },
                    "method":"put",
                    "url":"https://clevy.io"
                },
                "content_type":"http"
            },
            {
                "content":{
                    "body": {"hello":"world"},
                    "header":{
                        "Accept":"application/json,text/*",
                        "Content-Type":"application/json",
                        "User-Agent": "csml/v1"
                    },
                    "method":"get",
                    "url":"https://clevy.io"
                },
                "content_type":"http"
            }
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(HashMap::new(), HashMap::new(), None, None, "put_0", "flow"),
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
                "key":"http",
                "value":{
                    "_content":{
                        "header":{
                            "Accept":"application/json,text/*",
                            "Content-Type":"application/json",
                            "User-Agent": "csml/v1"
                        },
                        "method":"get",
                        "url":"https://clevy.io"
                    },
                    "_content_type":"http"
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
                        "Accept":"application/json,text/*",
                        "Content-Type":"application/json",
                        "User-Agent": "csml/v1"
                    },
                    "method":"patch",
                    "url":"https://clevy.io"
                },
                "content_type":"http"
            },
            {
                "content":{
                    "header":{
                        "Accept":"application/json,text/*",
                        "Content-Type":"application/json",
                        "User-Agent": "csml/v1"
                    },
                    "method":"get",
                    "url":"https://clevy.io"
                },
                "content_type":"http"
            }
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
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
                    "_content":{
                        "header":{
                            "Accept":"application/json,text/*",
                            "Content-Type":"application/json",
                            "User-Agent": "csml/v1"
                        },
                        "method":"get",
                        "url":"https://clevy.io"
                        },
                    "_content_type":"http"
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
                        "Accept":"application/json,text/*",
                        "Content-Type":"application/json",
                        "User-Agent": "csml/v1"
                    },
                    "method":"post",
                    "url":"https://clevy.io"
                },
                "content_type":"http"
            },
            {
                "content":{
                    "body": {"hello":"world"},
                    "header":{
                        "Accept":"application/json,text/*",
                        "Content-Type":"application/json",
                        "User-Agent": "csml/v1"
                    },
                    "method":"get",
                    "url":"https://clevy.io"
                },
                "content_type":"http"
            }
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(HashMap::new(), HashMap::new(), None, None, "post_0", "flow"),
        "CSML/basic_test/stdlib/http.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}



#[test]
fn http_auth_0() {
    let data = r#"{
        "memories":[],
        "messages":[
            {
                "content":{
                    "header":{
                        "Accept":"application/json,text/*",
                        "Content-Type":"application/json",
                        "User-Agent": "csml/v1",
                        "Authorization": "Basic dXNlcjpwYXNzd2Q="
                    },
                    "method":"get",
                    "url":"https://clevy.io"
                },
                "content_type":"http"
            }
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(HashMap::new(), HashMap::new(), None, None, "auth_0", "flow"),
        "CSML/basic_test/stdlib/http.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
