use csmlrustmanager::{data::CsmlData, start_conversation, user_close_all_conversations};
use neon::{context::Context, prelude::*, register_module};
use serde_json::{json, Value}; //, map::Map

fn get_flow_steps(mut cx: FunctionContext) -> JsResult<JsArray> {
    let jsbot = cx.argument::<JsValue>(0)?;
    let flow = cx.argument::<JsString>(1)?.value();
    let jsonbot: Value = neon_serde::from_value(&mut cx, jsbot)?;

    let vec = csmlrustmanager::get_steps_from_flow(serde_json::from_value(jsonbot).unwrap(), flow);

    let js_array = JsArray::new(&mut cx, vec.len() as u32);

    for (i, obj) in vec.iter().enumerate() {
        let js_string = cx.string(obj);
        js_array.set(&mut cx, i as u32, js_string).unwrap();
    }
    
    Ok(js_array)
}

fn validate_bot(mut cx: FunctionContext) -> JsResult<JsObject> {
    let jsbot = cx.argument::<JsValue>(0)?;
    let jsonbot: Value = neon_serde::from_value(&mut cx, jsbot)?;

    let object = JsObject::new(&mut cx);

    match csmlrustmanager::validate_bot(serde_json::from_value(jsonbot).unwrap()) {
        Ok(_) => {
            let valid = cx.boolean(true);
            object.set(&mut cx, "valid", valid).unwrap();
            Ok(object)
        }
        Err(vec) => {
            let err = vec[0].clone();

            let valid = cx.boolean(false);

            let flow = cx.string(err.position.flow);
            let step = cx.string(err.position.step);
            let line = cx.number(err.position.interval.line as f64);
            let column = cx.number(err.position.interval.column as f64);
            let message = cx.string(&err.message);

            object.set(&mut cx, "flow", flow).unwrap();
            object.set(&mut cx, "step", step).unwrap();
            object.set(&mut cx, "valid", valid).unwrap();
            object.set(&mut cx, "line", line).unwrap();
            object.set(&mut cx, "column", column).unwrap();
            object.set(&mut cx, "message", message).unwrap();
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

fn format_data(json_event: Value, jsdata: Value) -> Result<CsmlData, serde_json::error::Error> {
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
        sync: {
            match json_event["callback_url"].clone() {
                Value::Null => false,
                Value::Bool(val) => val,
                _ => false,
            }
        }, // optional
        bot: serde_json::from_value(jsdata)?,
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
    cx.export_function("validateBot", validate_bot)?;
    cx.export_function("getFlowSteps", get_flow_steps)?;
    cx.export_function("run", run_bot)?;
    cx.export_function("closeAllConversations", close_conversations)?;
    Ok(())
});
