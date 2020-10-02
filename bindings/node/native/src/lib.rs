use csml_interpreter::data::csml_bot::CsmlBot;
use csml_engine::{
    data::CsmlRequest, start_conversation, user_close_all_conversations, Client, CsmlResult,
    ErrorInfo, Warnings,
};
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

fn format_warnings<'a, C: Context<'a>>(
    cx: &mut C,
    array: &mut Handle<JsArray>,
    warnings: Vec<Warnings>,
) {
    for (index, warning) in warnings.iter().enumerate() {
        let object = JsObject::new(cx);
        let flow = cx.string(warning.position.flow.clone());
        let step = cx.string(warning.position.step.clone());
        let line = cx.number(warning.position.interval.line as f64);
        let column = cx.number(warning.position.interval.column as f64);
        let message = cx.string(&warning.message);

        object.set(cx, "flow", flow).unwrap();
        object.set(cx, "step", step).unwrap();
        object.set(cx, "line", line).unwrap();
        object.set(cx, "column", column).unwrap();
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
        let step = cx.string(err.position.step.clone());
        let line = cx.number(err.position.interval.line as f64);
        let column = cx.number(err.position.interval.column as f64);
        let message = cx.string(&err.message);

        object.set(cx, "flow", flow).unwrap();
        object.set(cx, "step", step).unwrap();
        object.set(cx, "line", line).unwrap();
        object.set(cx, "column", column).unwrap();
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
fn format_bot(data: Value) -> Result<CsmlBot, serde_json::error::Error> {
    Ok(CsmlBot {
        id: data["id"].as_str().unwrap().to_owned(),
        name: data["name"].as_str().unwrap().to_owned(),
        default_flow: data["default_flow"].as_str().unwrap().to_owned(),
        fn_endpoint: match data["fn_endpoint"].to_owned() {
            serde_json::Value::Null => None,
            val => Some(val.as_str().unwrap().to_owned()),
        },
        flows: serde_json::from_value(data["flows"].to_owned()).unwrap(),
        custom_components: serde_json::from_value(data["custom_components"].to_owned()).unwrap(),
        native_components: serde_json::from_value(data["native_components"].to_owned()).unwrap(),
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

    let bot: CsmlBot = match format_bot(json_bot) {
        Err(err) => panic!("Bad request: bot format {:?}", err),
        Ok(value) => value,
    };

    match start_conversation(request, bot) {
        Err(err) => panic!("{:?}", err),
        Ok(obj) => Ok(neon_serde::to_value(&mut cx, &obj)?),
    }
}

fn close_conversations(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let json_client = cx.argument::<JsValue>(0)?;

    match user_close_all_conversations(neon_serde::from_value(&mut cx, json_client)?) {
        Ok(_) => Ok(cx.boolean(true)),
        Err(err) => panic!(err),
    }
}

register_module!(mut cx, {
    // cx.export_function("validFlow", is_valid)?;
    cx.export_function("getOpenConversation", get_open_conversation)?;
    cx.export_function("validateBot", validate_bot)?;
    cx.export_function("getBotSteps", get_bot_steps)?;
    cx.export_function("run", run_bot)?;
    cx.export_function("closeAllConversations", close_conversations)?;
    Ok(())
});
