use crate::data::primitive::PrimitiveObject;
use crate::data::{ast::Interval, tokens::DEFAULT, Literal};
use crate::error_format::*;
use crate::interpreter::builtins::tools::*;
use std::collections::HashMap;

pub fn button(args: HashMap<String, Literal>, interval: Interval) -> Result<Literal, ErrorInfo> {
    let mut button: HashMap<String, Literal> = args.clone();
    let mut accepts = vec![];

    let title = match (button.remove("title"), button.remove(DEFAULT)) {
        (Some(title), ..) | (.., Some(title)) => {
            accepts.push(title.clone());
            title
        }
        _ => return Err(gen_error_info(interval, ERROR_BUTTON.to_owned())),
    };

    button.insert("title".to_owned(), title.to_owned());

    match button.get("payload") {
        Some(val) => {
            accepts.push(val.clone());
        }
        None => {
            button.insert("payload".to_owned(), title.clone());
        }
    }

    button.insert(
        "accepts".to_owned(),
        format_accept(button.get("accepts"), accepts, interval),
    );

    let mut result = PrimitiveObject::get_literal(&button, interval);
    result.set_content_type("button");

    Ok(result)
}

pub fn card(args: HashMap<String, Literal>, interval: Interval) -> Result<Literal, ErrorInfo> {
    let mut card: HashMap<String, Literal> = args.clone();

    match (card.remove("title"), card.remove(DEFAULT)) {
        (Some(title), ..) | (.., Some(title)) => {
            card.insert("title".to_owned(), title.to_owned());
        }
        _ => return Err(gen_error_info(interval, ERROR_CARD_TITLE.to_owned())),
    };

    match card.get("buttons") {
        Some(..) => {}
        _ => return Err(gen_error_info(interval, ERROR_CARD_BUTTON.to_owned())),
    };

    let mut result = PrimitiveObject::get_literal(&card, interval);
    result.set_content_type("card");

    Ok(result)
}
