use crate::data::error_info::ErrorInfo;
use crate::data::position::Position;
use crate::data::primitive::{PrimitiveObject, PrimitiveString, PrimitiveType};
use crate::data::{ast::Interval, ApiInfo, ArgsType, Client, Data, Literal, MessageData, MSG};
use crate::error_format::*;
use crate::interpreter::{
    builtins::{http::http_request, tools::*},
    json_to_rust::interpolate,
};

use std::{collections::HashMap, env, sync::mpsc};

fn format_body(args: &ArgsType, flow_name: &str, interval: Interval, client: Client) -> Result<Literal, ErrorInfo> {
    let mut map: HashMap<String, Literal> = HashMap::new();

    match args.get("fn_id", 0) {
        Some(literal) if literal.primitive.get_type() == PrimitiveType::PrimitiveString => {
            let fn_id = Literal::get_value::<String>(
                &literal.primitive,
                flow_name,
                literal.interval,
                ERROR_FN_ID.to_owned(),
            )?;

            map.insert(
                "function_id".to_owned(),
                PrimitiveString::get_literal(&fn_id, interval),
            );
        }
        _ => {
            return Err(gen_error_info(
                Position::new(interval, flow_name),
                ERROR_FN_ID.to_owned(),
            ))
        }
    };
    let mut sub_map = HashMap::new();
    args.populate(&mut sub_map, &["fn_id"], flow_name, interval)?;

    let client = client_to_json(&client, interval);

    map.insert(
        "data".to_owned(),
        PrimitiveObject::get_literal(&sub_map, interval),
    );
    map.insert(
        "client".to_owned(),
        PrimitiveObject::get_literal(&client, interval),
    );

    Ok(PrimitiveObject::get_literal(&map, interval))
}

fn format_headers(interval: Interval) -> HashMap<String, Literal> {
    let mut header = HashMap::new();
    header.insert(
        "content-type".to_owned(),
        PrimitiveString::get_literal("application/json", interval),
    );
    header.insert(
        "accept".to_owned(),
        PrimitiveString::get_literal("application/json,text/*", interval),
    );

    match env::var("FN_X_API_KEY") {
        Ok(value) => {
            header.insert(
                "X-Api-Key".to_owned(),
                PrimitiveString::get_literal(&value, interval),
            );
        }
        Err(_e) => {}
    };

    header
}

pub fn api(
    args: ArgsType,
    interval: Interval,
    data: &mut Data,
    msg_data: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<Literal, ErrorInfo> {
    let (client, url) = match &data.context.api_info {
        Some(ApiInfo {
            client,
            fn_endpoint,
        }) => (client.to_owned(), fn_endpoint.to_owned()),
        None => {
            return Err(gen_error_info(
                Position::new(interval, &data.context.flow),
                ERROR_FN_ENDPOINT.to_owned(),
            ))
        }
    };

    let mut http: HashMap<String, Literal> = HashMap::new();
    let header = format_headers(interval);
    let body = format_body(&args, &data.context.flow, interval, client)?;

    http.insert(
        "url".to_owned(),
        PrimitiveString::get_literal(&url, interval),
    );

    let lit_header = PrimitiveObject::get_literal(&header, interval);
    http.insert("header".to_owned(), lit_header);
    http.insert("body".to_owned(), body);

    match http_request(&http, "post", &data.context.flow, interval) {
        Ok(value) => match value.get("data") {
            Some(value) => interpolate(value, interval, data, msg_data, sender),
            None => {
                let err = gen_error_info(Position::new(interval, &data.context.flow), ERROR_HTTP_NOT_DATA.to_owned());
                Ok(MSG::send_error_msg(sender, msg_data, Err(err)))
            }
        },
        Err(err) => Ok(MSG::send_error_msg(sender, msg_data, Err(err))),
    }
}
