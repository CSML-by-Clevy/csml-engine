use crate::data::primitive::{PrimitiveArray, PrimitiveString};
use crate::data::{Client, Interval, Literal};
use crate::error_format::*;
use std::collections::HashMap;
use std::hash::BuildHasher;

pub fn create_submap<S: BuildHasher>(
    keys: &[&str],
    args: &HashMap<String, Literal, S>,
) -> Result<HashMap<String, Literal>, ErrorInfo> {
    let mut map = HashMap::new();

    for elem in args.keys() {
        if keys.iter().find(|&&x| x == elem).is_none() {
            if let Some(literal) = args.get(&*elem) {
                map.insert(elem.clone(), literal.clone());
            }
        }
    }
    Ok(map)
}

pub fn client_to_json(client: &Client, interval: Interval) -> HashMap<String, Literal> {
    let mut map = HashMap::new();

    map.insert("bot_id".to_owned(), PrimitiveString::get_literal(&client.bot_id, interval));
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

pub fn accept_to_array(literal: &HashMap<String, Literal>, mut vec: Vec<Literal>) -> Vec<Literal> {
    match literal.get("accepts") {
        Some(literal) => {
            match Literal::get_value::<Vec<Literal>>(
                &literal.primitive,
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

pub fn accepts_from_buttons(buttons: &Literal) -> Literal {
    match Literal::get_value::<Vec<Literal>>(
        &buttons.primitive,
        buttons.interval,
        ERROR_UNREACHABLE.to_owned(),
    ) {
        Ok(vec) => {
            let array = vec.iter().fold(vec![], |vec, elem| {
                match Literal::get_value::<HashMap<String, Literal>>(
                    &elem.primitive,
                    buttons.interval,
                    ERROR_UNREACHABLE.to_owned(),
                ) {
                    Ok(value) => accept_to_array(value, vec),
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
    interval: Interval,
) -> Literal {
    match values {
        Some(literal) => match Literal::get_value::<Vec<Literal>>(
            &literal.primitive,
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
