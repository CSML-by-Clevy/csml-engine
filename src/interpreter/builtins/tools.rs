use crate::data::primitive::array::PrimitiveArray;
use crate::data::{Client, Literal};
use crate::error_format::*;
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::hash::BuildHasher;

pub fn create_submap<S: BuildHasher>(
    keys: &[&str],
    args: &HashMap<String, Literal, S>,
) -> Result<Map<String, Value>, ErrorInfo> {
    let mut map = Map::new();

    for elem in args.keys() {
        if keys.iter().find(|&&x| x == elem).is_none() {
            if let Some(literal) = args.get(&*elem) {
                map.insert(elem.clone(), literal.primitive.to_json());
            }
        }
    }
    Ok(map)
}

pub fn client_to_json(client: &Client) -> Map<String, Value> {
    let mut map = Map::new();

    map.insert("bot_id".to_owned(), Value::String(client.bot_id.to_owned()));
    map.insert(
        "channel_id".to_owned(),
        Value::String(client.channel_id.to_owned()),
    );
    map.insert(
        "user_id".to_owned(),
        Value::String(client.user_id.to_owned()),
    );

    map
}

pub fn accept_to_array(literal: &HashMap<String, Literal>, mut vec: Vec<Literal>) -> Vec<Literal> {
    match literal.get("accepts") {
        Some(literal) => {
            match Literal::get_value::<Vec<Literal>>(&literal.primitive) {
                Some(array) => vec.append(&mut array.to_owned()),
                None => vec.push(literal.to_owned()),
            }
            vec
        }
        None => vec,
    }
}

pub fn accepts_from_buttons(buttons: &Literal) -> Literal {
    match Literal::get_value::<Vec<Literal>>(&buttons.primitive) {
        Some(vec) => {
            let array = vec.iter().fold(vec![], |vec, elem| {
                match Literal::get_value::<HashMap<String, Literal>>(&elem.primitive) {
                    Some(value) => accept_to_array(value, vec),
                    None => vec,
                }
            });
            PrimitiveArray::get_literal(&array, buttons.interval)
        }
        None => PrimitiveArray::get_literal(&[], buttons.interval),
    }
}

// pub fn search_or_default(values: &HashMap<String, Literal>, name: &str) -> Option<Literal> {
//     match values.get(name) {
//         Some(value) => Some(value.to_owned()),
//         None => match values.get(DEFAULT) {
//             Some(value) => Some(value.to_owned()),
//             None => None,
//         },
//     }
// }

pub fn format_accept(values: Option<&Literal>, title: Literal) -> Literal {
    match values {
        Some(literal) => match Literal::get_value::<Vec<Literal>>(&literal.primitive) {
            Some(res) => {
                let mut vector = res.clone();

                vector.push(title);

                PrimitiveArray::get_literal(&vector, literal.interval)
            }
            None => {
                let mut vector = Vec::new();

                vector.push(literal.to_owned());
                vector.push(title);

                PrimitiveArray::get_literal(&vector, literal.interval)
            }
        },
        None => {
            let mut items = Vec::new();

            items.push(title.to_owned());

            PrimitiveArray::get_literal(&items, title.interval)
        }
    }
}
