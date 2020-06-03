use serde_json::{json, Value}; //, map::Map
use neon::{context::Context, prelude::*, register_module};
use csmlrustmanager::{data::CsmlData, start_conversation, valid_flow, user_close_all_conversations};
// use csmlinterpreter::data::Client;

fn is_valid(mut cx: FunctionContext) -> JsResult<JsObject> {
    let file = cx.argument::<JsString>(0)?.value();
    let object = JsObject::new(&mut cx);

    match valid_flow(&file) {
        Ok(_) => {
            let valid = cx.boolean(true);

            object.set(&mut cx, "valid", valid).unwrap();
            Ok(object)
        }
        Err(e) => {
            let valid = cx.boolean(false);
            let line = cx.number(e.interval.line as f64);
            let column = cx.number(e.interval.column as f64);
            let message = cx.string(&e.message);

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

// CsmlData {
//     pub request_id: String,
//     pub client: Client,
//     pub callback_url: Option<String>,
//     pub payload: CsmlMessagePayload,
//     pub bot: CsmlBot,
//     pub metadata: Value,
//     pub sync: bool,
// }

// const event = {
//   request_id: "tmp",
//   client: { user_id: "alexis", bot_id: "42", channel_id: "1" },
//   callback_url: null,
//   payload: {
//     content_type: 'text',
//     content: { text: "Song of the day" },
//   },
//   metadata: {},
//   sync: false,
// }

// const bot = {
//   id: "botid",
//   name: "plop",
//   fn_endpoint: "xxxx",
//   flows: [
//     {
//       id: "flowid",
//       name: "flow",
//       content: "start:\n say \"plop\"",
//       triggers: ["/plop"],
//     }
//   ],
//   default_flow: "flowid",
// },

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
        Ok((_, obj)) => Ok(neon_serde::to_value(&mut cx, &obj)?),
    }
}

fn get_step_names(mut _cx: FunctionContext) -> JsResult<JsObject> {
    unimplemented!()
}

fn close_conversations(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let json_client = cx.argument::<JsValue>(0)?;

    match user_close_all_conversations(neon_serde::from_value(&mut cx, json_client)?) {
        Ok(_) => Ok(cx.boolean(true)),
        Err(err) => panic!(err)
    }
}

register_module!(mut cx, {
    cx.export_function("validFlow", is_valid)?;
    cx.export_function("run", run_bot)?;
    cx.export_function("getStepNames", get_step_names)?;
    cx.export_function("closeAllConversations", close_conversations)?;
    Ok(())
});
