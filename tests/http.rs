mod support;

use csmlinterpreter::data::{Event, MessageData};
use csmlinterpreter::interpret;
use serde_json::Value;

use support::tools::{gen_context, gen_event, message_to_json_value, read_file};

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
                    "body":{ 
                        "content_type":"body"
                    },
                    "header":{
                        "accept":"application/json,text/*",
                        "content-type":"application/json",
                        "content_type":"header"
                    },
                    "method":"get",
                    "query":{
                        "content_type":"query"
                    },
                    "url":"https://clevy.io"
                }
            }
        ],
        "messages":[
            {
                "content":{
                    "body": {
                        "content_type":"body"
                    },
                    "header":{
                        "accept":"application/json,text/*",
                        "content-type":"application/json",
                        "content_type":"header"
                    },
                    "method":"get",
                    "query":{
                        "content_type":"query"
                    },
                    "url":"https://clevy.io"
                },
                "content_type":"http"
            }
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(gen_event(""), "http_0");

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
                    "body":{
                        "content_type":"body"
                    },
                    "header":{
                        "accept":"application/json,text/*",
                        "content-type":"application/json",
                        "content_type":"header"
                    },
                    "method":"get",
                    "query":{
                        "content_type":"query"
                    },
                    "url":"https://clevy.io"
                }
            }
        ],
        "messages":[
            {
                "content":{
                    "body": {
                        "content_type":"body"
                    },
                    "header":{
                        "accept":"application/json,text/*",
                        "content-type":"application/json",
                        "content_type":"header"
                    },
                    "method":"get",
                    "query":{
                        "content_type":"query"
                    },
                    "url":"https://clevy.io"
                },
                "content_type":"http"
            },
            {
                "content":{
                    "body": {
                        "content_type":"body"
                    },
                    "header":{
                        "accept":"application/json,text/*",
                        "content-type":"application/json",
                        "content_type":"header"
                    },
                    "method":"get",
                    "query":{
                        "content_type":"query"
                    },
                    "url":"https://clevy.io"
                },
                "content_type":"http"
            }
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(gen_event(""), "get_0");

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

// #[test]
// fn http_get_1() {
//     let data = r#"{
//         "memories":[],
//         "messages":[
//             {
//                 "content":{
//                     "body": "quia et suscipit\nsuscipit recusandae consequuntur expedita et cum\nreprehenderit molestiae ut ut quas totam\nnostrum rerum est autem sunt rem eveniet architecto",
//                     "id":1,
//                     "title":"sunt aut facere repellat provident occaecati excepturi optio reprehenderit",
//                     "userId":1
//                 },
//                 "content_type":"object"
//             }
//         ],
//         "next_flow":null,
//         "next_step":null}"#;
//     let msg = format_message(gen_event(""), "get_1");

//     let v1: Value = message_to_json_value(msg);
//     let v2: Value = serde_json::from_str(data).unwrap();

//     assert_eq!(v1, v2)
// }

#[test]
fn http_set_0() {
    let data = r#"{
        "memories":[
            {
                "key":"http", "value":{
                    "body":{
                        "content_type":"body"
                    },
                    "header":{
                        "accept":"application/json,text/*",
                        "content-type":"application/json",
                        "content_type":"header"
                    },
                    "method":"get",
                    "query":{
                        "content_type":"query"
                    },
                    "url":"https://clevy.io"
                }
            }
        ],
        "messages":[
            {
                "content":{
                    "body": {
                        "content_type":"body"
                    },
                    "header":{
                        "accept":"application/json,text/*",
                        "content-type":"application/json",
                        "hello":"world",
                        "content_type":"header"
                    },
                    "method":"get",
                    "query":{
                        "content_type":"query"
                    },
                    "url":"https://clevy.io"
                },
                "content_type":"http"
            },
            {
                "content":{
                    "body": {
                        "content_type":"body"
                    },
                    "header":{
                        "accept":"application/json,text/*",
                        "content-type":"application/json",
                        "content_type":"header"
                    },
                    "method":"get",
                    "query":{
                        "content_type":"query"
                    },
                    "url":"https://clevy.io"
                },
                "content_type":"http"
            }
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(gen_event(""), "set_0");

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
                    "body":{
                        "content_type":"body"
                    },
                    "header":{
                        "accept":"application/json,text/*",
                        "content-type":"application/json",
                        "content_type":"header"
                    },
                    "method":"get",
                    "query":{
                        "content_type":"query"
                    },
                    "url":"https://clevy.io"
                }
            }
        ],
        "messages":[
            {
                "content":{
                    "body": {
                        "content_type":"body"
                    },
                    "header":{
                        "accept":"application/json,text/*",
                        "content-type":"application/json",
                        "content_type":"header"
                    },
                    "method":"get",
                    "query":{
                        "hello":"world",
                        "content_type":"query"
                    },
                    "url":"https://clevy.io"
                },
                "content_type":"http"
            },
            {
                "content":{
                    "body": {
                        "content_type":"body"
                    },
                    "header":{
                        "accept":"application/json,text/*",
                        "content-type":"application/json",
                        "content_type":"header"
                    },
                    "method":"get",
                    "query":{
                        "content_type":"query"
                    },
                    "url":"https://clevy.io"
                },
                "content_type":"http"
            }
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(gen_event(""), "query_0");

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
                    "body":{
                        "content_type":"body"
                    },
                    "header":{
                        "accept":"application/json,text/*",
                        "content-type":"application/json",
                        "content_type":"header"
                    },
                    "method":"get",
                    "query":{
                        "content_type":"query"
                    },
                    "url":"https://clevy.io"
                }
            }
        ],
        "messages":[
            {
                "content":{
                    "body": {
                        "hello":"world",
                        "content_type":"body"
                    },
                    "header":{
                        "accept":"application/json,text/*",
                        "content-type":"application/json",
                        "content_type":"header"
                    },
                    "method":"delete",
                    "query":{
                        "content_type":"query"
                    },
                    "url":"https://clevy.io"
                },
                "content_type":"http"
            },
            {
                "content":{
                    "body":{
                        "content_type":"body"
                    },
                    "header":{
                        "accept":"application/json,text/*",
                        "content-type":"application/json",
                        "content_type":"header"
                    },
                    "method":"get",
                    "query":{
                        "content_type":"query"
                    },
                    "url":"https://clevy.io"
                },
                "content_type":"http"
            }
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(gen_event(""), "delete_0");

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

// #[test]
// fn http_delete_1() {
//     let data = r#"{
//         "memories":[],
//         "messages":[
//             {
//                 "content":{},
//                 "content_type":"object"
//             }
//         ],
//         "next_flow":null,
//         "next_step":null}"#;
//     let msg = format_message(gen_event(""), "delete_1");

//     let v1: Value = message_to_json_value(msg);
//     let v2: Value = serde_json::from_str(data).unwrap();

//     assert_eq!(v1, v2)
// }

#[test]
fn http_put_0() {
    let data = r#"{
        "memories":[
            {
                "key":"http", "value":{
                    "body":{
                        "content_type":"body"
                    },
                    "header":{
                        "accept":"application/json,text/*",
                        "content-type":"application/json",
                        "content_type":"header"
                    },
                    "method":"get",
                    "query":{
                        "content_type":"query"
                    },
                    "url":"https://clevy.io"
                }
            }
        ],
        "messages":[
            {
                "content":{
                    "body": {
                        "hello":"world",
                        "content_type":"body"
                    },
                    "header":{
                        "accept":"application/json,text/*",
                        "content-type":"application/json",
                        "content_type":"header"
                    },
                    "method":"put",
                    "query":{
                        "content_type":"query"
                    },
                    "url":"https://clevy.io"
                },
                "content_type":"http"
            },
            {
                "content":{
                    "body": {
                        "content_type":"body"
                    },
                    "header":{
                        "accept":"application/json,text/*",
                        "content-type":"application/json",
                        "content_type":"header"
                    },
                    "method":"get",
                    "query":{
                        "content_type":"query"
                    },
                    "url":"https://clevy.io"
                },
                "content_type":"http"
            }
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(gen_event(""), "put_0");

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

// #[test]
// fn http_put_1() {
//     let data = r#"{
//         "memories":[],
//         "messages":[
//             {
//                 "content":{
//                     "body": "bar",
//                     "id":1,
//                     "title":"foo",
//                     "userId":"1"
//                 },
//                 "content_type":"object"
//             }
//         ],
//         "next_flow":null,
//         "next_step":null}"#;
//     let msg = format_message(gen_event(""), "put_1");

//     let v1: Value = message_to_json_value(msg);
//     let v2: Value = serde_json::from_str(data).unwrap();

//     assert_eq!(v1, v2)
// }

#[test]
fn http_patch_0() {
    let data = r#"{
        "memories":[
            {
                "key":"http", "value":{
                    "body":{
                        "content_type":"body"
                    },
                    "header":{
                        "accept":"application/json,text/*",
                        "content-type":"application/json",
                        "content_type":"header"
                    },
                    "method":"get",
                    "query":{
                        "content_type":"query"
                    },
                    "url":"https://clevy.io"
                }
            }
        ],
        "messages":[
            {
                "content":{
                    "body": {
                        "hello":"world",
                        "content_type":"body"
                    },
                    "header":{
                        "accept":"application/json,text/*",
                        "content-type":"application/json",
                        "content_type":"header"
                    },
                    "method":"patch",
                    "query":{
                        "content_type":"query"
                    },
                    "url":"https://clevy.io"
                },
                "content_type":"http"
            },
            {
                "content":{
                    "body": {
                        "content_type":"body"
                    },
                    "header":{
                        "accept":"application/json,text/*",
                        "content-type":"application/json",
                        "content_type":"header"
                    },
                    "method":"get",
                    "query":{
                        "content_type":"query"
                    },
                    "url":"https://clevy.io"
                },
                "content_type":"http"
            }
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(gen_event(""), "patch_0");

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

// #[test]
// fn http_patch_1() {
//     let data = r#"{
//         "memories":[],
//         "messages":[
//             {
//                 "content":{
//                     "body": "quia et suscipit\nsuscipit recusandae consequuntur expedita et cum\nreprehenderit molestiae ut ut quas totam\nnostrum rerum est autem sunt rem eveniet architecto",
//                     "id":1,
//                     "title":"foo",
//                     "userId":1
//                 },
//                 "content_type":"object"
//             }
//         ],
//         "next_flow":null,
//         "next_step":null}"#;
//     let msg = format_message(gen_event(""), "patch_1");

//     let v1: Value = message_to_json_value(msg);
//     let v2: Value = serde_json::from_str(data).unwrap();

//     assert_eq!(v1, v2)
// }

#[test]
fn http_post_0() {
    let data = r#"{
        "memories":[
            {
                "key":"http", "value":{
                    "body":{
                        "content_type":"body"
                    },
                    "header":{
                        "accept":"application/json,text/*",
                        "content-type":"application/json",
                        "content_type":"header"
                    },
                    "method":"get",
                    "query":{
                        "content_type":"query"
                    },
                    "url":"https://clevy.io"
                }
            }
        ],
        "messages":[
            {
                "content":{
                    "body": {
                        "hello":"world",
                        "content_type":"body"
                    },
                    "header":{
                        "accept":"application/json,text/*",
                        "content-type":"application/json",
                        "content_type":"header"
                    },
                    "method":"post",
                    "query":{
                        "content_type":"query"
                    },
                    "url":"https://clevy.io"
                },
                "content_type":"http"
            },
            {
                "content":{
                    "body": {
                        "content_type":"body"
                    },
                    "header":{
                        "accept":"application/json,text/*",
                        "content-type":"application/json",
                        "content_type":"header"
                    },
                    "method":"get",
                    "query":{
                        "content_type":"query"
                    },
                    "url":"https://clevy.io"
                },
                "content_type":"http"
            }
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(gen_event(""), "post_0");

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

// #[test]
// fn http_post_1() {
//     let data = r#"{
//         "memories":[],
//         "messages":[
//             {
//                 "content":{
//                     "id":101,
//                     "title":"foo",
//                     "body": "bar",
//                     "userId":"1"
//                 },
//                 "content_type":"object"
//             }
//         ],
//         "next_flow":null,
//         "next_step":null}"#;
//     let msg = format_message(gen_event(""), "post_1");

//     let v1: Value = message_to_json_value(msg);
//     let v2: Value = serde_json::from_str(data).unwrap();

//     assert_eq!(v1, v2)
// }
