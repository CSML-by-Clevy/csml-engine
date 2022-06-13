use csml_engine::{
    data::RunRequest, start_conversation, user_close_all_conversations, Client, CsmlResult,
    ErrorInfo, Warnings,
};
use csml_interpreter::data::csml_bot::CsmlBot;
use neon::{context::Context, prelude::*, register_module};
use serde::Deserialize;

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTION
////////////////////////////////////////////////////////////////////////////////

fn get_client<'a>(cx: &'a mut FunctionContext, object: Handle<JsObject>) -> NeonResult<Client> {
    let bot_id: Handle<JsString> = object.get(cx, "bot_id")?;
    let channel_id: Handle<JsString> = object.get(cx, "channel_id")?;
    let user_id: Handle<JsString> = object.get(cx, "user_id")?;

    Ok(Client {
        bot_id: bot_id.value(),
        channel_id: channel_id.value(),
        user_id: user_id.value(),
    })
}

fn get_bot<'a>(cx: &'a mut FunctionContext, object: Handle<JsValue>) -> NeonResult<CsmlBot> {
    let bot = get_json(cx, object)?;

    Ok(serde_json::from_value(bot).unwrap())
}

fn client_to_object<'a>(cx: &mut FunctionContext<'a>, client: Client) -> JsResult<'a, JsObject> {
    let obj = cx.empty_object();

    let bot_id = cx.string(client.bot_id);
    obj.set(cx, "bot_id", bot_id)?;

    let channel_id = cx.string(client.channel_id);
    obj.set(cx, "channel_id", channel_id)?;

    let user_id = cx.string(client.user_id);
    obj.set(cx, "user_id", user_id)?;

    Ok(obj)
}

fn get_json<'a>(
    cx: &mut FunctionContext<'a>,
    value: Handle<JsValue>,
) -> NeonResult<serde_json::Value> {
    if value.is_a::<JsString>() {
        let js_value = value.downcast::<JsString>().unwrap();

        return Ok(serde_json::json!(js_value.value()));
    }

    if value.is_a::<JsNumber>() {
        let js_value = value.downcast::<JsNumber>().unwrap();

        return Ok(serde_json::json!(js_value.value()));
    }

    if value.is_a::<JsBoolean>() {
        let js_value = value.downcast::<JsBoolean>().unwrap();

        return Ok(serde_json::json!(js_value.value()));
    }

    if value.is_a::<JsArray>() {
        let js_value = value.downcast::<JsArray>().unwrap();
        let js_vec = js_value.to_vec(cx)?;
        let mut vec = vec![];

        for val in js_vec {
            vec.push(get_json(cx, val)?);
        }

        return Ok(serde_json::json!(vec));
    }

    if value.is_a::<JsObject>() {
        let js_obj = value.downcast::<JsObject>().unwrap();
        let names = js_obj.get_own_property_names(cx)?.to_vec(cx)?;

        let mut obj = serde_json::Map::new();

        for js_name in names {
            let name = js_name.downcast::<JsString>().unwrap().value();

            let value: Handle<JsValue> = js_obj.get(cx, name.as_str())?;
            let json_value = get_json(cx, value)?;

            obj.insert(name, json_value);
        }

        return Ok(serde_json::json!(obj));
    }

    Ok(serde_json::Value::Null)
}

fn to_js_value<'a>(
    cx: &mut FunctionContext<'a>,
    value: serde_json::Value,
) -> JsResult<'a, JsValue> {
    match value {
        serde_json::Value::Bool(value) => Ok(cx.boolean(value).upcast::<JsValue>()),
        serde_json::Value::Number(value) => {
            Ok(cx.number(value.as_f64().unwrap()).upcast::<JsValue>())
        }
        serde_json::Value::String(value) => Ok(cx.string(value).upcast::<JsValue>()),
        serde_json::Value::Null => Ok(cx.null().upcast::<JsValue>()),
        serde_json::Value::Array(vec) => {
            let array: Handle<JsArray> = cx.empty_array();

            for (i, value) in vec.into_iter().enumerate() {
                let value = to_js_value(cx, value)?;

                array.set(cx, i as u32, value)?;
            }

            Ok(array.upcast::<JsValue>())
        }
        serde_json::Value::Object(map) => {
            let obj = cx.empty_object();

            for (key, value) in map {
                let key = cx.string(key);
                let value = to_js_value(cx, value)?;

                obj.set(cx, key, value)?;
            }

            Ok(obj.upcast::<JsValue>())
        }
    }
}

fn get_open_conversation(mut cx: FunctionContext) -> JsResult<JsValue> {
    let jsclient = cx.argument::<JsObject>(0)?;
    let client: Client = get_client(&mut cx, jsclient)?;

    match csml_engine::get_open_conversation(&client) {
        Ok(Some(conversation)) => {
            let obj = cx.empty_object();

            let id = cx.string(conversation.id);
            obj.set(&mut cx, "id", id)?;

            let r_client = client_to_object(&mut cx, conversation.client)?;
            obj.set(&mut cx, "client", r_client)?;

            let flow_id = cx.string(conversation.flow_id);
            obj.set(&mut cx, "flow_id", flow_id)?;

            let step_id = cx.string(conversation.step_id);
            obj.set(&mut cx, "step_id", step_id)?;

            let status = cx.string(conversation.status);
            obj.set(&mut cx, "status", status)?;

            let last_interaction_at = cx.string(conversation.last_interaction_at);
            obj.set(&mut cx, "last_interaction_at", last_interaction_at)?;

            let updated_at = cx.string(conversation.updated_at);
            obj.set(&mut cx, "updated_at", updated_at)?;

            let created_at = cx.string(conversation.created_at);
            obj.set(&mut cx, "created_at", created_at)?;

            Ok(obj.upcast())
        }
        Ok(None) => {
            let js_value = cx.null();

            Ok(js_value.upcast())
        }
        Err(err) => panic!("{:?}", err),
    }
}

fn get_client_current_state(mut cx: FunctionContext) -> JsResult<JsValue> {
    let jsclient = cx.argument::<JsObject>(0)?;
    let client: Client = get_client(&mut cx, jsclient)?;
    let obj = cx.empty_object();

    match csml_engine::get_current_state(&client) {
        Ok(Some(state_value)) => {
            let obj = cx.empty_object();

            let r_client = client_to_object(&mut cx, client)?;
            obj.set(&mut cx, "client", r_client)?;

            let s_type = cx.string(state_value["type"].as_str().unwrap());
            obj.set(&mut cx, "type", s_type)?;

            let value = cx.string(state_value["value"].as_str().unwrap());
            obj.set(&mut cx, "type", value)?;

            let created_at = cx.string(state_value["created_at"].as_str().unwrap());
            obj.set(&mut cx, "type", created_at)?;

            Ok(obj.upcast())
        }
        Ok(None) => {
            let js_value = cx.null();

            Ok(js_value.upcast())
        }
        Err(err) => {
            let error = cx.string(format!("{:?}", err));
            obj.set(&mut cx, "error", error)?;

            Ok(obj.upcast())
        }
    }
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
        if let (Some(end_line), Some(end_column)) = (
            warning.position.interval.end_line,
            warning.position.interval.end_column,
        ) {
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
        if let (Some(end_line), Some(end_column)) = (
            err.position.interval.end_line,
            err.position.interval.end_column,
        ) {
            let end_line = cx.number(end_line as f64);
            let end_column = cx.number(end_column as f64);

            object.set(cx, "end_line", end_line).unwrap();
            object.set(cx, "end_column", end_column).unwrap();
        }
        object.set(cx, "message", message).unwrap();

        array.set(cx, index as u32, object).unwrap();
    }
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

fn create_client_memory(mut cx: FunctionContext) -> JsResult<JsValue> {
    let jsclient = cx.argument::<JsObject>(0)?;
    let client: Client = get_client(&mut cx, jsclient)?;

    let key = cx.argument::<JsString>(1)?.value();

    let jsvalue = cx.argument::<JsValue>(2)?;
    let json_value = get_json(&mut cx, jsvalue)?;

    match csml_engine::create_client_memory(&client, key, json_value) {
        Ok(_) => Ok(cx.empty_object().upcast()),
        Err(err) => {
            let obj = cx.empty_object();

            let error = cx.string(format!("{:?}", err));
            obj.set(&mut cx, "error", error)?;

            Ok(obj.upcast())
        }
    }
}

fn get_bot_steps(mut cx: FunctionContext) -> JsResult<JsObject> {
    let jsbot = cx.argument::<JsValue>(0)?;
    let bot = get_bot(&mut cx, jsbot)?;

    let map = csml_engine::get_steps_from_flow(bot);

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

fn validate_bot(mut cx: FunctionContext) -> JsResult<JsObject> {
    let jsbot = cx.argument::<JsValue>(0)?;
    let bot = get_bot(&mut cx, jsbot)?;

    let object = JsObject::new(&mut cx);

    match csml_engine::validate_bot(bot) {
        CsmlResult {
            flows: _,
            extern_flows: _,
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
            extern_flows: _,
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
    let json_run_request = get_json(&mut cx, raw_run_request)?;
    let run_request: RunRequest = serde_json::from_value(json_run_request).unwrap();

    let bot_opt = match run_request.get_bot_opt() {
        Ok(bot_opt) => bot_opt,
        Err(err) => panic!("{:?}", err),
    };
    let request = run_request.event;

    match start_conversation(request, bot_opt) {
        Err(err) => panic!("{:?}", err),
        Ok(obj) => Ok(to_js_value(&mut cx, serde_json::json!(obj))?),
    }
}

fn close_conversations(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let jsclient = cx.argument::<JsObject>(0)?;
    let client: Client = get_client(&mut cx, jsclient)?;

    match user_close_all_conversations(client) {
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
    let bot = get_bot(&mut cx, raw_bot)?;

    let obj = cx.empty_object();

    match csml_engine::create_bot_version(bot) {
        Ok(version_data) => {
            let value = serde_json::json!(version_data);

            Ok(to_js_value(&mut cx, value)?)
        }
        Err(err) => {
            let error = cx.string(format!("{:?}", err));
            obj.set(&mut cx, "error", error)?;

            Ok(obj.upcast())
        }
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

    let obj = cx.empty_object();

    match csml_engine::get_bot_by_version_id(&version_id, &bot_id) {
        Ok(bot) => {
            let value = match bot {
                Some(bot) => {
                    serde_json::json!(bot.flatten())
                }
                None => {
                    serde_json::json!({
                        "error": "Not found"
                    })
                }
            };

            Ok(to_js_value(&mut cx, value)?)
        }
        Err(err) => {
            let error = cx.string(format!("{:?}", err));
            obj.set(&mut cx, "error", error)?;

            Ok(obj.upcast())
        }
    }
}

/*
* get last bot version
*
* {
*   id: String,
*   name: String,
*   apps_endpoint: Option<String>,
*   flows: Vec<CsmlFlow>,
*   custom_components: Option<>,
*   default_flow: String,
* }
*/
fn get_last_bot_version(mut cx: FunctionContext) -> JsResult<JsValue> {
    let bot_id = cx.argument::<JsString>(0)?.value();

    let obj = cx.empty_object();

    match csml_engine::get_last_bot_version(&bot_id) {
        Ok(bot) => {
            let value = match bot {
                Some(bot) => {
                    serde_json::json!(bot.flatten())
                }
                None => {
                    serde_json::json!({
                        "error": "Not found"
                    })
                }
            };

            Ok(to_js_value(&mut cx, value)?)
        }
        Err(err) => {
            let error = cx.string(format!("{:?}", err));
            obj.set(&mut cx, "error", error)?;

            Ok(obj.upcast())
        }
    }
}

/*
* Delete bot version
*/
fn delete_bot_version(mut cx: FunctionContext) -> JsResult<JsValue> {
    let bot_id = cx.argument::<JsString>(0)?.value();
    let version_id = cx.argument::<JsString>(1)?.value();

    let obj = cx.empty_object();

    match csml_engine::delete_bot_version_id(&version_id, &bot_id) {
        Ok(value) => {
            let value = serde_json::json!(value);

            Ok(to_js_value(&mut cx, value)?)
        }
        Err(err) => {
            let error = cx.string(format!("{:?}", err));
            obj.set(&mut cx, "error", error)?;

            Ok(obj.upcast())
        }
    }
}

/*
* Delete bot versions
*/
fn delete_bot_versions(mut cx: FunctionContext) -> JsResult<JsValue> {
    let bot_id = cx.argument::<JsString>(0)?.value();

    let obj = cx.empty_object();

    match csml_engine::delete_all_bot_versions(&bot_id) {
        Ok(value) => {
            let value = serde_json::json!(value);

            Ok(to_js_value(&mut cx, value)?)
        }
        Err(err) => {
            let error = cx.string(format!("{:?}", err));
            obj.set(&mut cx, "error", error)?;

            Ok(obj.upcast())
        }
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
    let bot = get_bot(&mut cx, raw_bot)?;

    let obj = cx.empty_object();

    match csml_engine::fold_bot(bot) {
        Ok(flow) => {
            let value = serde_json::json!({ "flow": flow });

            Ok(to_js_value(&mut cx, value)?)
        }
        Err(err) => {
            let error = cx.string(format!("{:?}", err));
            obj.set(&mut cx, "error", error)?;

            Ok(obj.upcast())
        }
    }
}

/*
* Delete client memory
*/
fn delete_client_memory(mut cx: FunctionContext) -> JsResult<JsValue> {
    let jsclient = cx.argument::<JsObject>(0)?;
    let client: Client = get_client(&mut cx, jsclient)?;
    let memory_name = cx.argument::<JsString>(1)?.value();

    let obj = cx.empty_object();

    match csml_engine::delete_client_memory(&client, &memory_name) {
        Ok(value) => {
            let value = serde_json::json!(value);

            Ok(to_js_value(&mut cx, value)?)
        }
        Err(err) => {
            let error = cx.string(format!("{:?}", err));
            obj.set(&mut cx, "error", error)?;

            Ok(obj.upcast())
        }
    }
}

/*
* Remove all memories associated with a given Client
*/
fn delete_client_memories(mut cx: FunctionContext) -> JsResult<JsValue> {
    let jsclient = cx.argument::<JsObject>(0)?;
    let client: Client = get_client(&mut cx, jsclient)?;

    let obj = cx.empty_object();

    match csml_engine::delete_client_memories(&client) {
        Ok(value) => {
            let value = serde_json::json!(value);

            Ok(to_js_value(&mut cx, value)?)
        }
        Err(err) => {
            let error = cx.string(format!("{:?}", err));
            obj.set(&mut cx, "error", error)?;

            Ok(obj.upcast())
        }
    }
}

/*
* Remove all data associated with a given Client
*/
fn delete_client_data(mut cx: FunctionContext) -> JsResult<JsValue> {
    let jsclient = cx.argument::<JsObject>(0)?;
    let client: Client = get_client(&mut cx, jsclient)?;

    let obj = cx.empty_object();

    match csml_engine::delete_client(&client) {
        Ok(value) => {
            let value = serde_json::json!(value);

            Ok(to_js_value(&mut cx, value)?)
        }
        Err(err) => {
            let error = cx.string(format!("{:?}", err));
            obj.set(&mut cx, "error", error)?;

            Ok(obj.upcast())
        }
    }
}

/*
* Remove all data associated with a given bot:
* conversations, messages, memories, interactions, states, path, versions
*/
fn delete_bot_data(mut cx: FunctionContext) -> JsResult<JsValue> {
    let bot_id = cx.argument::<JsString>(0)?.value();

    let obj = cx.empty_object();

    match csml_engine::delete_all_bot_data(&bot_id) {
        Ok(value) => {
            let value = serde_json::json!(value);

            Ok(to_js_value(&mut cx, value)?)
        }
        Err(err) => {
            let error = cx.string(format!("{:?}", err));
            obj.set(&mut cx, "error", error)?;

            Ok(obj.upcast())
        }
    }
}

/*
* Remove all data associated with a given bot:
* conversations, messages, memories, interactions, states, path, versions
*/
fn delete_expired_data(mut cx: FunctionContext) -> JsResult<JsValue> {
    let obj = cx.empty_object();

    match csml_engine::delete_expired_data() {
        Ok(value) => {
            let value = serde_json::json!(value);

            Ok(to_js_value(&mut cx, value)?)
        }
        Err(err) => {
            let error = cx.string(format!("{:?}", err));
            obj.set(&mut cx, "error", error)?;

            Ok(obj.upcast())
        }
    }
}

fn get_client_memories(mut cx: FunctionContext) -> JsResult<JsValue> {
    let jsclient = cx.argument::<JsObject>(0)?;
    let client: Client = get_client(&mut cx, jsclient)?;

    let obj = cx.empty_object();

    match csml_engine::get_client_memories(&client) {
        Ok(value) => Ok(to_js_value(&mut cx, value)?),
        Err(err) => {
            let error = cx.string(format!("{:?}", err));
            obj.set(&mut cx, "error", error)?;

            Ok(obj.upcast())
        }
    }
}

fn get_client_memory(mut cx: FunctionContext) -> JsResult<JsValue> {
    let jsclient = cx.argument::<JsObject>(0)?;
    let client: Client = get_client(&mut cx, jsclient)?;

    let key = cx.argument::<JsString>(1)?.value();

    let obj = cx.empty_object();

    match csml_engine::get_client_memory(&client, &key) {
        Ok(value) => Ok(to_js_value(&mut cx, value)?),
        Err(err) => {
            let error = cx.string(format!("{:?}", err));
            obj.set(&mut cx, "error", error)?;

            Ok(obj.upcast())
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct LimitPaginationQueryParams {
    limit: Option<i64>,
    pagination_key: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BetweenDatesQueryParams {
    from_date: Option<i64>,
    to_date: Option<i64>,
}

fn get_client_messages(mut cx: FunctionContext) -> JsResult<JsValue> {
    let jsclient = cx.argument::<JsObject>(0)?;
    let client: Client = get_client(&mut cx, jsclient)?;

    let jsparams = cx.argument::<JsValue>(1)?;
    let json_value = get_json(&mut cx, jsparams)?;
    let params: LimitPaginationQueryParams = serde_json::from_value(json_value).unwrap();

    let jsparams = cx.argument::<JsValue>(2)?;
    let json_value = get_json(&mut cx, jsparams)?;

    let between_dates: BetweenDatesQueryParams = serde_json::from_value(json_value).unwrap();

    let obj = cx.empty_object();

    match csml_engine::get_client_messages(
        &client,
        params.limit,
        params.pagination_key,
        between_dates.from_date,
        between_dates.to_date,
    ) {
        Ok(value) => Ok(to_js_value(&mut cx, value)?),
        Err(err) => {
            let error = cx.string(format!("{:?}", err));
            obj.set(&mut cx, "error", error)?;

            Ok(obj.upcast())
        }
    }
}

fn get_client_conversations(mut cx: FunctionContext) -> JsResult<JsValue> {
    let jsclient = cx.argument::<JsObject>(0)?;
    let client: Client = get_client(&mut cx, jsclient)?;

    let jsparams = cx.argument::<JsValue>(1)?;
    let json_value = get_json(&mut cx, jsparams)?;

    let params: LimitPaginationQueryParams = serde_json::from_value(json_value).unwrap();

    let obj = cx.empty_object();

    match csml_engine::get_client_conversations(&client, params.limit, params.pagination_key) {
        Ok(value) => Ok(to_js_value(&mut cx, value)?),
        Err(err) => {
            let error = cx.string(format!("{:?}", err));
            obj.set(&mut cx, "error", error)?;

            Ok(obj.upcast())
        }
    }
}

fn make_migrations(mut cx: FunctionContext) -> JsResult<JsValue> {
    let obj = cx.empty_object();

    match csml_engine::make_migrations() {
        Ok(value) => {
            let value = serde_json::json!(value);

            Ok(to_js_value(&mut cx, value)?)
        }
        Err(err) => {
            let error = cx.string(format!("{:?}", err));
            obj.set(&mut cx, "error", error)?;

            Ok(obj.upcast())
        }
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
    let json_value = get_json(&mut cx, jsparams)?;
    let params: LimitPaginationQueryParams = serde_json::from_value(json_value).unwrap();

    let obj = cx.empty_object();

    match csml_engine::get_bot_versions(&bot_id, params.limit, params.pagination_key) {
        Ok(value) => Ok(to_js_value(&mut cx, value)?),
        Err(err) => {
            let error = cx.string(format!("{:?}", err));
            obj.set(&mut cx, "error", error)?;

            Ok(obj.upcast())
        }
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
