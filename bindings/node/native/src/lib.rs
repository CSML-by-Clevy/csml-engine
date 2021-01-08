use csml_engine::{
    data::{CsmlRequest, BotOpt}, start_conversation, user_close_all_conversations, 
    Client, CsmlResult,
};
use csml_interpreter::data::csml_bot::CsmlBot;
use neon::{context::Context, prelude::*, register_module};
use serde_json::{json, Value};

fn get_open_conversation(mut cx: FunctionContext) -> JsResult<JsValue> {
    let jsclient = cx.argument::<JsValue>(0)?;
    let jsonclient: Value = neon_serde::from_value(&mut cx, jsclient)?;
    let client: Client = serde_json::from_value(jsonclient).unwrap();

    match csml_engine::get_open_conversation(&client) {
        Ok(Some(conversation)) => {
            let mut map = serde_json::json!(conversation).as_object().unwrap().to_owned();

            // DbConversation uses _id instead of id for the default mongodb mapping
            if let Some(id) = map.remove("_id") {
                map.insert("id".to_owned(), id);
            }

            let js_value = neon_serde::to_value(&mut cx, &map)?;
            Ok(js_value)
        }
        Ok(None) => {
            let js_value = neon_serde::to_value(&mut cx, &serde_json::json!(null))?;

            Ok(js_value)
        }
        Err(err) => panic!(err),
    }
}

fn get_bot_steps(mut cx: FunctionContext) -> JsResult<JsObject> {
    let jsbot = cx.argument::<JsValue>(0)?;
    let jsonbot: Value = neon_serde::from_value(&mut cx, jsbot)?;

    let map = csml_engine::get_steps_from_flow(serde_json::from_value(jsonbot).unwrap());

    let js_object = JsObject::new(&mut cx);

    for (flow, steps) in map.iter() {
        let js_array = JsArray::new(&mut cx, steps.len() as u32);

        for (i, step) in steps.iter().enumerate() {
            let step = cx.string(step);
            js_array.set(&mut cx, i as u32, step).unwrap();
        }

        let key = cx.string(flow);
        js_object.set(&mut cx, key, js_array).unwrap();
    }

    Ok(js_object)
}

fn validate_bot(mut cx: FunctionContext) -> JsResult<JsValue> {
    let jsbot = cx.argument::<JsValue>(0)?;
    let jsonbot: Value = neon_serde::from_value(&mut cx, jsbot)?;

    match csml_engine::validate_bot(serde_json::from_value(jsonbot).unwrap()) {
        CsmlResult {
            flows: _,
            warnings,
            errors: None,
        } => {
            let mut value = serde_json::json!({
                "statusCode": 200,
                "valid": true,
            });

            if let Some(warnings) = warnings {
                value["warnings"] = serde_json::json!(warnings);
            }

            Ok(neon_serde::to_value(&mut cx, &value)?)
        }
        CsmlResult {
            flows: _,
            warnings,
            errors: Some(errors),
        } => {
            let mut value = serde_json::json!({
                "statusCode": 400,
                "valid": false,
                "errors": errors,
            });

            if let Some(warnings) = warnings {
                value["warnings"] = serde_json::json!(warnings);
            }

            Ok(neon_serde::to_value(&mut cx, &value)?)
        }
    }
}

fn format_request(json_request: Value) -> Result<CsmlRequest, serde_json::error::Error> {
    Ok(CsmlRequest {
        request_id: json_request["request_id"].as_str().unwrap().to_owned(),
        client: serde_json::from_value(json_request["client"].clone())?,
        callback_url: {
            match json_request["callback_url"].clone() {
                Value::Null => None,
                val => Some(val.as_str().unwrap().to_owned()),
            }
        },
        payload: serde_json::from_value(json_request["payload"].clone())?,
        metadata: {
            match json_request["metadata"].clone() {
                Value::Null => json!({}),
                val => val,
            }
        },
    })
}

fn run_bot(mut cx: FunctionContext) -> JsResult<JsValue> {
    let raw_request = cx.argument::<JsValue>(0)?;
    let raw_bot = cx.argument::<JsValue>(1)?;

    let json_request: Value = neon_serde::from_value(&mut cx, raw_request)?;
    let json_bot: Value = neon_serde::from_value(&mut cx, raw_bot)?;

    let request = match format_request(json_request) {
        Err(err) => panic!("Bad request: event format {:?}", err),
        Ok(value) => value,
    };

    let bot: BotOpt = match serde_json::from_value(json_bot) {
        Err(err) => panic!("Bad request: bot format {:?}", err),
        Ok(value) => value,
    };

    match start_conversation(request, bot) {
        Ok(obj) => {
            let value = serde_json::json!({
                "statusCode": 201,
                "body": obj
            });

            Ok(neon_serde::to_value(&mut cx, &value)?)
        },
        Err(err) => {
            let value = serde_json::json!({
                "statusCode": 400,
                "body": format!("{:?}", err),
            });

            Ok(neon_serde::to_value(&mut cx, &value)?)
        },
    }
}

fn close_conversations(mut cx: FunctionContext) -> JsResult<JsValue> {
    let json_client = cx.argument::<JsValue>(0)?;

    match user_close_all_conversations(neon_serde::from_value(&mut cx, json_client)?) {
        Ok(_) => {
            let value = serde_json::json!({
                "statusCode": 200,
            });

            Ok(neon_serde::to_value(&mut cx, &value)?)
        },
        Err(err) => {
            let value = serde_json::json!({
                "statusCode": 400,
                "body": format!("{:?}", err),
            });

            Ok(neon_serde::to_value(&mut cx, &value)?)
        }
    }
}

/*
* create bot version
*
*{"statusCode": 200,"body": {"version_id": String} }
*
*/
fn create_bot_version(mut cx: FunctionContext) -> JsResult<JsValue> {
    let raw_bot = cx.argument::<JsValue>(0)?;

    let json_bot: Value = neon_serde::from_value(&mut cx, raw_bot)?;

    let bot: CsmlBot = match serde_json::from_value(json_bot) {
        Err(err) => panic!("Bad bot format: {:?}", err),
        Ok(bot) => bot,
    };

    match csml_engine::create_bot_version(bot) {
        Ok(version_id) => {
            let value = serde_json::json!({
                "statusCode": 201,
                "body": {"version_id": version_id}
            });

            Ok(neon_serde::to_value(&mut cx, &value)?)
        },
        Err(err) => {
            let value = serde_json::json!({
                "statusCode": 400,
                "body": format!("{:?}", err),
            });

            Ok(neon_serde::to_value(&mut cx, &value)?)
        },
    }
}

/*
* get bot by version
*
* {"statusCode": 200,"body": Bot}
*
* BOT = {
*  "version_id": String,
*  "id": String,
*  "name": String,
*  "custom_components": Option<String>,
*  "default_flow": String
*  "engine_version": String
*  "created_at": String
* }
*/
fn get_bot_by_version_id(mut cx: FunctionContext) -> JsResult<JsValue> {
    let id = cx.argument::<JsString>(0)?.value();
    let bot_id = cx.argument::<JsString>(1)?.value();

    match csml_engine::get_bot_by_version_id(&id, &bot_id) {
        Ok(bot) => {
            let value = match bot {
                Some(bot) => {
                    serde_json::json!({
                        "statusCode": 200,
                        "body": bot.flatten()
                    })
                }
                None => {
                    serde_json::json!({
                        "statusCode": 400,
                        "body": "Not found"
                    })
                }
            };

            Ok(neon_serde::to_value(&mut cx, &value)?)
        }
        Err(err) => {
            let value = serde_json::json!({
                "statusCode": 400,
                "body": format!("{:?}", err),
            });

            Ok(neon_serde::to_value(&mut cx, &value)?)
        },
    }
}

/* 
* get last bot version
*
* bot: {
*   id: String,
*   name: String,
*   fn_endpoint: Option<String>,
*   flows: Vec<CsmlFlow>,
*   custom_components: Option<>,
*   default_flow: String,
* }
*/
fn get_last_bot_version(mut cx: FunctionContext) -> JsResult<JsValue> {
    let bot_id = cx.argument::<JsString>(0)?.value();

    match csml_engine::get_last_bot_version(&bot_id) {
        Ok(bot) => {
            let value = match bot {
                Some(bot) => {
                    serde_json::json!({
                        "statusCode": 200,
                        "body": bot.flatten()
                    })
                }
                None => {
                    serde_json::json!({
                        "statusCode": 400,
                        "body": "Not found"
                    })
                }
            };

            Ok(neon_serde::to_value(&mut cx, &value)?)
        },
        Err(err) => {
            let value = serde_json::json!({
                "statusCode": 400,
                "body": format!("{:?}", err),
            });

            Ok(neon_serde::to_value(&mut cx, &value)?)
        },
    }
}

/*
* Get the last 20 versions of the bot if no limit is set
*
* {"statusCode": 200,"body": Vec<Bot>}
*
* BOT = {
*  "version_id": String,
*  "id": String,
*  "name": String,
*  "custom_components": Option<String>,
*  "default_flow": String
*  "engine_version": String
*  "created_at": String
* }
*/
fn get_bot_versions(mut cx: FunctionContext) -> JsResult<JsValue> {
    let bot_id = cx.argument::<JsString>(0)?.value();
    let limit = match cx.argument::<JsNumber>(1) {
        Ok(key) => Some(key.value() as i64),
        Err(_) => None
    };

    let last_key = match cx.argument::<JsString>(2) {
        Ok(key) => Some(key.value()),
        Err(_) => None
    };

    match csml_engine::get_bot_versions(&bot_id, limit, last_key) {
        Ok(value) => {
            let value= serde_json::json!({
                "statusCode": 200,
                "body": value
            });

            Ok(neon_serde::to_value(&mut cx, &value)?)
        },
        Err(err) => {
            let value = serde_json::json!({
                "statusCode": 400,
                "body": format!("{:?}", err),
            });

            Ok(neon_serde::to_value(&mut cx, &value)?)
        },
    }
}

register_module!(mut cx, {
    cx.export_function("validateBot", validate_bot)?;

    cx.export_function("createBotVersion", create_bot_version)?;
    cx.export_function("getBotByVersionId", get_bot_by_version_id)?;
    cx.export_function("getLastBotVersion", get_last_bot_version)?;
    cx.export_function("getBotVersions", get_bot_versions)?;
    
    cx.export_function("run", run_bot)?;
    
    cx.export_function("closeAllConversations", close_conversations)?;
    cx.export_function("getBotSteps", get_bot_steps)?;
    cx.export_function("getOpenConversation", get_open_conversation)?;
    Ok(())
});
