use crate::data::primitive::{object::PrimitiveObject, string::PrimitiveString};
use crate::data::{ast::Interval, Literal};
use crate::error_format::ErrorInfo;
use crate::interpreter::builtins::tools::*;
use std::collections::HashMap;

pub fn button(
    values: HashMap<String, Literal>,
    _name: String,
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    let mut button: HashMap<String, Literal> = HashMap::new();

    let title = search_or_default(&values, "title", interval, None)?;

    button.insert("title".to_owned(), title.to_owned());
    button.insert(
        "button_type".to_owned(),
        search_or_default(
            &values,
            "button_type",
            interval,
            Some(PrimitiveString::get_literal(
                "button",
                "quick_button",
                interval.to_owned(),
            )),
        )?,
    );

    button.insert(
        "accepts".to_owned(),
        format_accept(values.get("accepts"), title),
    );
    if let Ok(payload) = search_or_default(&values, "payload", interval, None) {
        button.insert("payload".to_owned(), payload);
    }

    Ok(PrimitiveObject::get_literal("button", &button, interval))
}
