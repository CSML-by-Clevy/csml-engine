use csml_engine::{
    data::{RunRequest}, start_conversation, user_close_all_conversations,
    Client, CsmlResult, ErrorInfo, Warnings
};
use csml_interpreter::data::csml_bot::CsmlBot;
use neon::{context::Context, prelude::*, register_module};
use serde_json::{Value};
use serde::Deserialize;

fn get_open_conversation(mut cx: FunctionContext) -> JsResult<JsValue> {
    let jsclient = cx.argument::<JsValue>(0)?;
    let client: Client = neon_serde::from_value(&mut cx, jsclient)?;

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
        Err(err) => panic!("{:?}", err),
    }
}

fn get_client_current_state(mut cx: FunctionContext) -> JsResult<JsValue> {
    let jsclient = cx.argument::<JsValue>(0)?;
    let client: Client = neon_serde::from_value(&mut cx, jsclient)?;

    match csml_engine::get_current_state(&client) {
        Ok(Some(value)) => {
            Ok(neon_serde::to_value(&mut cx, &value)?)
        },
        Ok(None) => {
            Ok(neon_serde::to_value(&mut cx, &serde_json::Value::Null)?)
        },
        Err(err) => {
            let value = serde_json::json!({
                "error": format!("{:?}", err),
            });

            Ok(neon_serde::to_value(&mut cx, &value)?)
        },
    }
}

fn create_client_memory(mut cx: FunctionContext) -> JsResult<JsValue> {
    let jsclient = cx.argument::<JsValue>(0)?;
    let client: Client = neon_serde::from_value(&mut cx, jsclient)?;

    let key = cx.argument::<JsString>(1)?.value();

    let jsvalue = cx.argument::<JsValue>(2)?;
    let value: Value = neon_serde::from_value(&mut cx, jsvalue)?;

    match csml_engine::create_client_memory(&client, key, value) {
        Ok(value) => {
            let value= serde_json::json!(
                value
            );

            Ok(neon_serde::to_value(&mut cx, &value)?)
        },
        Err(err) => {
            let value = serde_json::json!({
                "error": format!("{:?}", err),
            });

            Ok(neon_serde::to_value(&mut cx, &value)?)
        },
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

fn format_warnings<'a, C: Context<'a>>(
    cx: &mut C,
    array: &mut Handle<JsArray>,
    warnings: Vec<Warnings>,
) {
    for (index, warning) in warnings.iter().enumerate() {
        let object = JsObject::new(cx);
        let flow = cx.string(warning.position.flow.clone());
        let line = cx.number(warning.position.interval.start_line as f64);
        let column = cx.number(warning.position.interval.start_column as f64);

        let message = cx.string(&warning.message);

        object.set(cx, "flow", flow).unwrap();
        object.set(cx, "start_line", line).unwrap();
        object.set(cx, "start_column", column).unwrap();
        if let (Some(end_line),Some(end_column)) = (warning.position.interval.end_line, warning.position.interval.end_column) {
            let end_line = cx.number(end_line as f64);
            let end_column = cx.number(end_column as f64);

            object.set(cx, "end_line", end_line).unwrap();
            object.set(cx, "end_column", end_column).unwrap();
        }

        object.set(cx, "message", message).unwrap();

        array.set(cx, index as u32, object).unwrap();
    }
}

fn format_errors<'a, C: Context<'a>>(
    cx: &mut C,
    array: &mut Handle<JsArray>,
    errors: Vec<ErrorInfo>,
) {
    for (index, err) in errors.iter().enumerate() {
        let object = JsObject::new(cx);
        let flow = cx.string(err.position.flow.clone());
        let line = cx.number(err.position.interval.start_line as f64);
        let column = cx.number(err.position.interval.start_column as f64);
        let message = cx.string(&err.message);

        object.set(cx, "flow", flow).unwrap();
        object.set(cx, "start_line", line).unwrap();
        object.set(cx, "start_column", column).unwrap();
        if let (Some(end_line),Some(end_column)) = (err.position.interval.end_line, err.position.interval.end_column) {
            let end_line = cx.number(end_line as f64);
            let end_column = cx.number(end_column as f64);

            object.set(cx, "end_line", end_line).unwrap();
            object.set(cx, "end_column", end_column).unwrap();
        }
        object.set(cx, "message", message).unwrap();

        array.set(cx, index as u32, object).unwrap();
    }
}

fn validate_bot(mut cx: FunctionContext) -> JsResult<JsObject> {
    let jsbot = cx.argument::<JsValue>(0)?;
    let jsonbot: Value = neon_serde::from_value(&mut cx, jsbot)?;

    let object = JsObject::new(&mut cx);

    match csml_engine::validate_bot(serde_json::from_value(jsonbot).unwrap()) {
        CsmlResult {
            flows: _,
            warnings,
            errors: None,
        } => {
            let valid = cx.boolean(true);
            object.set(&mut cx, "valid", valid).unwrap();

            if let Some(warnings) = warnings {
                let mut js_warnings = JsArray::new(&mut cx, warnings.len() as u32);
                format_warnings(&mut cx, &mut js_warnings, warnings);

                object.set(&mut cx, "warnings", js_warnings).unwrap();
            }

            Ok(object)
        }
        CsmlResult {
            flows: _,
            warnings,
            errors: Some(errors),
        } => {
            let valid = cx.boolean(false);

            object.set(&mut cx, "valid", valid).unwrap();

            if let Some(warnings) = warnings {
                let mut js_warnings = JsArray::new(&mut cx, warnings.len() as u32);
                format_warnings(&mut cx, &mut js_warnings, warnings);

                object.set(&mut cx, "warnings", js_warnings).unwrap();
            }

            let mut js_errors = JsArray::new(&mut cx, errors.len() as u32);
            format_errors(&mut cx, &mut js_errors, errors);

            object.set(&mut cx, "errors", js_errors).unwrap();

            Ok(object)
        }
    }
}

fn run_bot(mut cx: FunctionContext) -> JsResult<JsValue> {
    let raw_run_request = cx.argument::<JsValue>(0)?;

    let run_request: RunRequest = neon_serde::from_value(&mut cx, raw_run_request)?;

    let bot_opt = match run_request.get_bot_opt() {
        Ok(bot_opt) => bot_opt,
        Err(err) => panic!("{:?}", err),
    };
    let request = run_request.event;

    match start_conversation(request, bot_opt) {
        Err(err) => panic!("{:?}", err),
        Ok(obj) => Ok(neon_serde::to_value(&mut cx, &obj)?),
    }
}

fn close_conversations(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let json_client = cx.argument::<JsValue>(0)?;

    match user_close_all_conversations(neon_serde::from_value(&mut cx, json_client)?) {
        Ok(_) => Ok(cx.boolean(true)),
        Err(err) => panic!("{:?}", err),
    }
}

/*
* create bot version
*
* {"version_id": String}
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
        Ok(version_data) => {
            let value = serde_json::json!(version_data);

            Ok(neon_serde::to_value(&mut cx, &value)?)
        },
        Err(err) => {
            let value = serde_json::json!({
                "error": format!("{:?}", err),
            });

            Ok(neon_serde::to_value(&mut cx, &value)?)
        },
    }
}

/*
* get bot by version
*
* {
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
    let bot_id = cx.argument::<JsString>(0)?.value();
    let version_id = cx.argument::<JsString>(1)?.value();

    match csml_engine::get_bot_by_version_id(&version_id, &bot_id) {
        Ok(bot) => {
            let value = match bot {
                Some(bot) => {
                    serde_json::json!(
                        bot.flatten()
                    )
                }
                None => {
                    serde_json::json!({
                        "error": "Not found"
                    })
                }
            };

            Ok(neon_serde::to_value(&mut cx, &value)?)
        }
        Err(err) => {
            let value = serde_json::json!({
                "error": format!("{:?}", err),
            });

            Ok(neon_serde::to_value(&mut cx, &value)?)
        },
    }
}

/*
* get last bot version
*
* {
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
                    serde_json::json!(
                        bot.flatten()
                    )
                }
                None => {
                    serde_json::json!({
                        "error": "Not found"
                    })
                }
            };

            Ok(neon_serde::to_value(&mut cx, &value)?)
        },
        Err(err) => {
            let value = serde_json::json!({
                "error": format!("{:?}", err),
            });

            Ok(neon_serde::to_value(&mut cx, &value)?)
        },
    }
}

/*
* Delete bot version
*/
fn delete_bot_version(mut cx: FunctionContext) -> JsResult<JsValue> {
    let bot_id = cx.argument::<JsString>(0)?.value();
    let version_id =  cx.argument::<JsString>(1)?.value();

    match csml_engine::delete_bot_version_id(&version_id, &bot_id) {
        Ok(value) => {
            let value= serde_json::json!(
                value
            );

            Ok(neon_serde::to_value(&mut cx, &value)?)
        },
        Err(err) => {
            let value = serde_json::json!({
                "error": format!("{:?}", err),
            });

            Ok(neon_serde::to_value(&mut cx, &value)?)
        },
    }
}

/*
* Delete bot versions
*/
fn delete_bot_versions(mut cx: FunctionContext) -> JsResult<JsValue> {
    let bot_id = cx.argument::<JsString>(0)?.value();

    match csml_engine::delete_all_bot_versions(&bot_id) {
        Ok(value) => {
            let value= serde_json::json!(
                value
            );

            Ok(neon_serde::to_value(&mut cx, &value)?)
        },
        Err(err) => {
            let value = serde_json::json!({
                "error": format!("{:?}", err),
            });

            Ok(neon_serde::to_value(&mut cx, &value)?)
        },
    }
}

/*
* fold bot into a single flow
*
* {"flow": String}
*
*/
fn fold_bot(mut cx: FunctionContext) -> JsResult<JsValue> {
    let raw_bot = cx.argument::<JsValue>(0)?;

    let json_bot: Value = neon_serde::from_value(&mut cx, raw_bot)?;

    let bot: CsmlBot = match serde_json::from_value(json_bot) {
        Err(err) => panic!("Bad bot format: {:?}", err),
        Ok(bot) => bot,
    };

    match csml_engine::fold_bot(bot) {
        Ok(flow) => {
            let value = serde_json::json!({"flow": flow});

            Ok(neon_serde::to_value(&mut cx, &value)?)
        },
        Err(err) => {
            let value = serde_json::json!({
                "error": format!("{:?}", err),
            });

            Ok(neon_serde::to_value(&mut cx, &value)?)
        },
    }
}

/*
* Delete client memory
*/
fn delete_client_memory(mut cx: FunctionContext) -> JsResult<JsValue> {
    let jsclient = cx.argument::<JsValue>(0)?;
    let memory_name = cx.argument::<JsString>(1)?.value();

    let client: Client = neon_serde::from_value(&mut cx, jsclient)?;
    match csml_engine::delete_client_memory(&client, &memory_name) {
        Ok(value) => {
            let value= serde_json::json!(
                value
            );

            Ok(neon_serde::to_value(&mut cx, &value)?)
        },
        Err(err) => {
            let value = serde_json::json!({
                "error": format!("{:?}", err),
            });

            Ok(neon_serde::to_value(&mut cx, &value)?)
        },
    }
}

/*
* Remove all memories associated with a given Client
*/
fn delete_client_memories(mut cx: FunctionContext) -> JsResult<JsValue> {
    let jsclient = cx.argument::<JsValue>(0)?;
    let client: Client = neon_serde::from_value(&mut cx, jsclient)?;

    match csml_engine::delete_client_memories(&client) {
        Ok(value) => {
            let value= serde_json::json!(
                value
            );

            Ok(neon_serde::to_value(&mut cx, &value)?)
        },
        Err(err) => {
            let value = serde_json::json!({
                "error": format!("{:?}", err),
            });

            Ok(neon_serde::to_value(&mut cx, &value)?)
        },
    }
}

/*
* Remove all data associated with a given Client
*/
fn delete_client_data(mut cx: FunctionContext) -> JsResult<JsValue> {
    let jsclient = cx.argument::<JsValue>(0)?;
    let client: Client = neon_serde::from_value(&mut cx, jsclient)?;

    match csml_engine::delete_client(&client) {
        Ok(value) => {
            let value= serde_json::json!(
                value
            );

            Ok(neon_serde::to_value(&mut cx, &value)?)
        },
        Err(err) => {
            let value = serde_json::json!({
                "error": format!("{:?}", err),
            });

            Ok(neon_serde::to_value(&mut cx, &value)?)
        },
    }
}

/*
* Remove all data associated with a given bot:
* conversations, messages, memories, interactions, states, path, versions
*/
fn delete_bot_data(mut cx: FunctionContext) -> JsResult<JsValue> {
    let bot_id = cx.argument::<JsString>(0)?.value();

    match csml_engine::delete_all_bot_data(&bot_id) {
        Ok(value) => {
            let value= serde_json::json!(
                value
            );

            Ok(neon_serde::to_value(&mut cx, &value)?)
        },
        Err(err) => {
            let value = serde_json::json!({
                "error": format!("{:?}", err),
            });

            Ok(neon_serde::to_value(&mut cx, &value)?)
        },
    }
}

/*
* Remove all data associated with a given bot:
* conversations, messages, memories, interactions, states, path, versions
*/
fn delete_expired_data(mut cx: FunctionContext) -> JsResult<JsValue> {

    match csml_engine::delete_expired_data() {
        Ok(value) => {
            let value= serde_json::json!(
                value
            );

            Ok(neon_serde::to_value(&mut cx, &value)?)
        },
        Err(err) => {
            let value = serde_json::json!({
                "error": format!("{:?}", err),
            });

            Ok(neon_serde::to_value(&mut cx, &value)?)
        },
    }
}

fn get_client_memories(mut cx: FunctionContext) -> JsResult<JsValue> {
    let jsclient = cx.argument::<JsValue>(0)?;
    let client: Client = neon_serde::from_value(&mut cx, jsclient)?;

    match csml_engine::get_client_memories(&client) {
        Ok(value) => {

            Ok(neon_serde::to_value(&mut cx, &value)?)
        },
        Err(err) => {
            let value = serde_json::json!({
                "error": format!("{:?}", err),
            });

            Ok(neon_serde::to_value(&mut cx, &value)?)
        },
    }
}

fn get_client_memory(mut cx: FunctionContext) -> JsResult<JsValue> {
    let jsclient = cx.argument::<JsValue>(0)?;
    let key = cx.argument::<JsString>(1)?.value();
    let client: Client = neon_serde::from_value(&mut cx, jsclient)?;

    match csml_engine::get_client_memory(&client, &key) {
        Ok(value) => {

            Ok(neon_serde::to_value(&mut cx, &value)?)
        },
        Err(err) => {
            let value = serde_json::json!({
                "error": format!("{:?}", err),
            });

            Ok(neon_serde::to_value(&mut cx, &value)?)
        },
    }
}

#[derive(Debug, Deserialize)]
pub struct LimitPaginationQueryParams {
  limit: Option<i64>,
  pagination_key: Option<String>,
}

fn get_client_messages(mut cx: FunctionContext) -> JsResult<JsValue> {
    let jsclient = cx.argument::<JsValue>(0)?;
    let client: Client = neon_serde::from_value(&mut cx, jsclient)?;

    let jsparams = cx.argument::<JsValue>(1)?;
    let params: LimitPaginationQueryParams = neon_serde::from_value(&mut cx, jsparams)?;

    match csml_engine::get_client_messages(&client, params.limit, params.pagination_key) {
        Ok(value) => {
            Ok(neon_serde::to_value(&mut cx, &value)?)
        },
        Err(err) => {
            let value = serde_json::json!({
                "error": format!("{:?}", err),
            });

            Ok(neon_serde::to_value(&mut cx, &value)?)
        },
    }
}

fn get_client_conversations(mut cx: FunctionContext) -> JsResult<JsValue> {
    let jsclient = cx.argument::<JsValue>(0)?;
    let client: Client = neon_serde::from_value(&mut cx, jsclient)?;

    let jsparams = cx.argument::<JsValue>(1)?;
    let params: LimitPaginationQueryParams = neon_serde::from_value(&mut cx, jsparams)?;

    match csml_engine::get_client_conversations(&client, params.limit, params.pagination_key) {
        Ok(value) => {
            Ok(neon_serde::to_value(&mut cx, &value)?)
        },
        Err(err) => {
            let value = serde_json::json!({
                "error": format!("{:?}", err),
            });

            Ok(neon_serde::to_value(&mut cx, &value)?)
        },
    }
}

fn make_migrations(mut cx: FunctionContext) -> JsResult<JsValue> {

    match csml_engine::make_migrations() {
        Ok(value) => {
            let value = serde_json::json!(
                value
            );

            Ok(neon_serde::to_value(&mut cx, &value)?)
        },
        Err(err) => {
            let value = serde_json::json!({
                "error": format!("{:?}", err),
            });

            Ok(neon_serde::to_value(&mut cx, &value)?)
        },
    }
}

/*
* Get the last 20 versions of the bot if no limit is set
*
* {
*  "version_id": String,
*  "id": String,
*  "name": String,
*  "custom_components": Option<String>,
*  "default_flow": String
*  "engine_version": String
*  "created_at": String
* }
*/
fn get_bot_versions_limit(mut cx: FunctionContext) -> JsResult<JsValue> {
    let bot_id = cx.argument::<JsString>(0)?.value();

    let jsparams = cx.argument::<JsValue>(1)?;
    let jsonparams: Value = neon_serde::from_value(&mut cx, jsparams)?;
    let params: LimitPaginationQueryParams = serde_json::from_value(jsonparams).unwrap();

    match csml_engine::get_bot_versions(&bot_id, params.limit, params.pagination_key) {
        Ok(value) => {
            Ok(neon_serde::to_value(&mut cx, &value)?)
        },
        Err(err) => {
            let value = serde_json::json!({
                "error": format!("{:?}", err),
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
    cx.export_function("getBotVersionsLimit", get_bot_versions_limit)?;
    cx.export_function("deleteBotVersion", delete_bot_version)?;
    cx.export_function("deleteBotVersions", delete_bot_versions)?;
    cx.export_function("foldBot", fold_bot)?;

    cx.export_function("createClientMemory ", create_client_memory)?;
    cx.export_function("getClientMemories", get_client_memories)?;
    cx.export_function("getClientMemory", get_client_memory)?;
    cx.export_function("getClientMessages", get_client_messages)?;
    cx.export_function("getClientCurrentState", get_client_current_state)?;
    cx.export_function("getClientConversations", get_client_conversations)?;
    cx.export_function("deleteMemory", delete_client_memory)?;
    cx.export_function("deleteMemories", delete_client_memories)?;
    cx.export_function("deleteClientData", delete_client_data)?;
    cx.export_function("deleteBotData", delete_bot_data)?;

    cx.export_function("deleteExpiredData", delete_expired_data)?;

    cx.export_function("migrations", make_migrations)?;

    cx.export_function("run", run_bot)?;

    cx.export_function("closeAllConversations", close_conversations)?;
    cx.export_function("getBotSteps", get_bot_steps)?;
    cx.export_function("getOpenConversation", get_open_conversation)?;
    Ok(())
});
