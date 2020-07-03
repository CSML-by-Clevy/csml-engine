use csmlrustmanager::{
    data::CsmlData, start_conversation, user_close_all_conversations, Client, CsmlResult,
    ErrorInfo, Warnings,
};
use neon::{context::Context, prelude::*, register_module};
use serde_json::{json, Value}; //, map::Map

fn get_open_conversation(mut cx: FunctionContext) -> JsResult<JsValue> {
    let jsclient = cx.argument::<JsValue>(0)?;
    let jsonclient: Value = neon_serde::from_value(&mut cx, jsclient)?;
    let client: Client = serde_json::from_value(jsonclient).unwrap();

    match csmlrustmanager::get_open_conversation(&client) {
        Ok(Some(conversation)) => {
            let mut map = serde_json::Map::new();

            map.insert( "id".to_owned(), serde_json::json!(conversation.id) ).unwrap();
            map.insert( "client".to_owned(), serde_json::json!(conversation.client) ).unwrap();
            map.insert( "flow_id".to_owned(), serde_json::json!(conversation.flow_id)).unwrap();
            map.insert( "step_id".to_owned(), serde_json::json!(conversation.step_id)).unwrap();
            map.insert( "metadata".to_owned(), serde_json::json!(conversation.metadata) ).unwrap();
            map.insert( "status".to_owned(), serde_json::json!(conversation.status) ).unwrap();
            map.insert( "last_interaction_at".to_owned(), serde_json::json!(conversation.last_interaction_at.to_string()))
                .unwrap();
            map.insert( "updated_at".to_owned(), serde_json::json!(conversation.updated_at.to_string()) ).unwrap();
            map.insert( "created_at".to_owned(), serde_json::json!(conversation.created_at.to_string()) ).unwrap();

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

    let map = csmlrustmanager::get_steps_from_flow(serde_json::from_value(jsonbot).unwrap());

    let js_object = JsObject::new(&mut cx);

    for (flow , steps) in map.iter() {
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

    match csmlrustmanager::validate_bot(serde_json::from_value(jsonbot).unwrap()) {
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

// const payloads = [
//   {
//     content_type: "image",
//     content: {
//       url: "https://...."
//     }
//    metadata: OBJ{
//      type: "jpg", //  ou autre type opt
//     } opt
//   },
//   {
//     content_type: "video",
//     content: {
//       url: "https://...."
//     }
//    metadata: OBJ{
//      type: "jpg", //  ou autre type opt
//     } opt
//   },
//   {
//     content_type: "audio",
//     content: {
//       url: "https://...."
//     }
//    metadata: OBJ{
//      type: "jpg", //  ou autre type opt
//     } opt
//   },
//   {
//     content_type: "attachment",
//     content: {
//       url: "https://...."
//     }
//    metadata: OBJ{
//      type: "jpg", //  ou autre type opt
//     } opt
//   },
//   {
//     content_type: "payload",
//     content: {
//       value: String
//     }
//    metadata: OBJ{
//      title: "title of button",
//      type: "jpg", //  ou autre type opt
//     } opt
//   },
//   {
//     content_type: "flow_trigger",
//     content: {
//        flow_id: "xxxxx",
//     }
//     metadata: OBJ{
//      title: "title of button",
//      type: "jpg", //  ou autre type opt
//     } opt
//   }
// ]

fn check_bot(jsbot: &mut Value) {
    if let serde_json::Value::Object(map) = jsbot {
        let id = map.get("id").unwrap().to_owned();
        if map.contains_key("name") {
            map.insert("name".to_owned(), id);
        };
    };
    if let Some(serde_json::Value::Array(flows)) = jsbot.get_mut("flows") {
        for flow in flows.iter_mut() {
            if let serde_json::Value::Object(map) = flow {
                let id = map.get("id").unwrap().to_owned();
                if map.contains_key("name") {
                    map.insert("name".to_owned(), id);
                };
            };
        }
    };
}

fn format_data(json_event: Value, mut jsbot: Value) -> Result<CsmlData, serde_json::error::Error> {

    check_bot(&mut jsbot);

    Ok(CsmlData {
        request_id: json_event["request_id"].as_str().unwrap().to_owned(),
        client: serde_json::from_value(json_event["client"].clone())?,
        callback_url: {
            match json_event["callback_url"].clone() {
                Value::Null => None,
                val => Some(val.as_str().unwrap().to_owned()),
            }
        }, // optional
        payload: serde_json::from_value(json_event.clone())?,
        metadata: {
            match json_event["metadata"].clone() {
                Value::Null => json!({}),
                val => val,
            }
        }, // optional
        bot: serde_json::from_value(jsbot)?,
    })
}

fn run_bot(mut cx: FunctionContext) -> JsResult<JsValue> {
    let raw_event = cx.argument::<JsValue>(0)?;
    let bot = cx.argument::<JsValue>(1)?;

    let jsdata: Value = neon_serde::from_value(&mut cx, bot)?;
    let json_event: Value = neon_serde::from_value(&mut cx, raw_event)?;

    // Calling panic!() in Neon will throw an Error in Node.
    // So panic!("program errored!") is equivalent to throw new Error('program errored!')
    let data = match format_data(json_event.clone(), jsdata) {
        Err(err) => panic!("event bad format {:?}", err),
        Ok(value) => value,
    };
    match start_conversation(json_event, data) {
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
