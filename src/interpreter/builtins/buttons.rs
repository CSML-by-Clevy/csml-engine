use crate::data::primitive::{object::PrimitiveObject, string::PrimitiveString};
use crate::data::{ast::Interval, Literal};
use crate::error_format::ErrorInfo;
use crate::interpreter::builtins::tools::*;
use std::collections::HashMap;

pub fn button(
    args: HashMap<String, Literal>,
    _name: String,
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    let mut button: HashMap<String, Literal> = HashMap::new();

    let title = search_or_default(&args, "title", interval, None)?;
    let mut literal = PrimitiveString::get_literal("quick_button", interval.to_owned());
    literal.set_content_type("button");

    button.insert("title".to_owned(), title.to_owned());
    button.insert(
        "button_type".to_owned(),
        search_or_default(&args, "button_type", interval, Some(literal))?,
    );

    button.insert(
        "accepts".to_owned(),
        format_accept(args.get("accepts"), title),
    );

    match args.get("theme") {
        Some(theme) => {
            button.insert("theme".to_owned(), theme.to_owned());
        }
        None => {
            button.insert(
                "theme".to_owned(),
                PrimitiveString::get_literal("primary", interval)
            );
        }
    };

    if let Some(icon) = args.get("icon") {
        button.insert("icon".to_owned(), icon.to_owned());
    }

    if let Ok(payload) =  search_or_default(&args, "payload", interval, None) {
        button.insert("payload".to_owned(), payload.to_owned());
    }

    let mut result = PrimitiveObject::get_literal(&button, interval);
    result.set_content_type("button");

    Ok(result)
}

pub fn card(
    args: HashMap<String, Literal>,
    _name: String,
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    let mut card: HashMap<String, Literal> = HashMap::new();

    let subtitle = search_or_default(&args, "subtitle", interval, None)?;

    card.insert("subtitle".to_owned(), subtitle.to_owned());

    
    match args.get("buttons") {
        Some(buttons) => card.insert("buttons".to_owned(), buttons.to_owned()),
        _ => {
            return Err(ErrorInfo {
                message: "argument buttons in Builtin Cards need to be of type Array of Button Component example: [ Button(\"b1\"), Button(\"b2\") ]".to_owned(),
                interval,
            })
        }
    };
    
    if let Some(image_url) = args.get("image_url") {
        card.insert("image_url".to_owned(), image_url.to_owned());
    }
    let mut result = PrimitiveObject::get_literal(&card, interval);
    result.set_content_type("card");

    Ok(result)
}
