use crate::data::primitive::{PrimitiveArray, PrimitiveString};
use crate::data::{Client, Interval, Literal};
use crate::error_format::*;
use std::collections::HashMap;

pub fn client_to_json(client: &Client, interval: Interval) -> HashMap<String, Literal> {
    let mut map = HashMap::new();

    map.insert(
        "bot_id".to_owned(),
        PrimitiveString::get_literal(&client.bot_id, interval),
    );
    map.insert(
        "channel_id".to_owned(),
        PrimitiveString::get_literal(&client.channel_id, interval),
    );
    map.insert(
        "user_id".to_owned(),
        PrimitiveString::get_literal(&client.user_id, interval),
    );

    map
}

pub fn accept_to_array(
    literal: &HashMap<String, Literal>,
    mut vec: Vec<Literal>,
    flow_name: &str,
) -> Vec<Literal> {
    match literal.get("accepts") {
        Some(literal) => {
            match Literal::get_value::<Vec<Literal>>(
                &literal.primitive,
                flow_name,
                literal.interval,
                ERROR_UNREACHABLE.to_owned(),
            ) {
                Ok(array) => vec.append(&mut array.to_owned()),
                Err(_) => vec.push(literal.to_owned()),
            }
            vec
        }
        None => vec,
    }
}

pub fn accepts_from_buttons(buttons: &Literal, flow_name: &str) -> Literal {
    match Literal::get_value::<Vec<Literal>>(
        &buttons.primitive,
        flow_name,
        buttons.interval,
        ERROR_UNREACHABLE.to_owned(),
    ) {
        Ok(vec) => {
            let array = vec.iter().fold(vec![], |vec, elem| {
                match Literal::get_value::<HashMap<String, Literal>>(
                    &elem.primitive,
                    flow_name,
                    buttons.interval,
                    ERROR_UNREACHABLE.to_owned(),
                ) {
                    Ok(value) => accept_to_array(value, vec, flow_name),
                    Err(..) => vec,
                }
            });
            PrimitiveArray::get_literal(&array, buttons.interval)
        }
        Err(..) => PrimitiveArray::get_literal(&[], buttons.interval),
    }
}

pub fn format_accept(
    values: Option<&Literal>,
    mut title: Vec<Literal>,
    flow_name: &str,
    interval: Interval,
) -> Literal {
    match values {
        Some(literal) => match Literal::get_value::<Vec<Literal>>(
            &literal.primitive,
            flow_name,
            literal.interval,
            ERROR_UNREACHABLE.to_owned(),
        ) {
            Ok(res) => {
                let mut vector = res.clone();

                vector.append(&mut title);

                PrimitiveArray::get_literal(&vector, literal.interval)
            }
            Err(..) => {
                let mut vector = Vec::new();

                vector.push(literal.to_owned());
                vector.append(&mut title);

                PrimitiveArray::get_literal(&vector, literal.interval)
            }
        },
        None => PrimitiveArray::get_literal(&title, interval),
    }
}
