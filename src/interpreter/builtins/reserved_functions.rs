use rand::Rng;
use std::collections::HashMap;
use crate::error_format::data::ErrorInfo;
use crate::parser::{ast::{Literal, Interval}}; //, tokens::*

// TODO: check nbr elemts in built-ins
pub fn typing(args: HashMap<String, Literal>, name: String, interval: Interval) -> Result<Literal, ErrorInfo> {
    match args.get("default") {
        Some(Literal::IntLiteral{value: lit}) => Ok(Literal::name_object(name.to_lowercase(), &Literal::int(*lit))),
        Some(Literal::FloatLiteral{value: lit}) => Ok(Literal::name_object(name.to_lowercase(), &Literal::float(*lit))),
        _ => Err(ErrorInfo{
                message: "Builtin Typing expect one argument of type int or float | example: Typing(3, ..)".to_owned(),
                interval
        })
    }
}

pub fn wait(args: HashMap<String, Literal>, name: String, interval: Interval) -> Result<Literal, ErrorInfo> {
    match args.get("default") {
        Some(Literal::IntLiteral{value: lit}) => Ok(Literal::name_object(name.to_lowercase(), &Literal::int(*lit))),
        Some(Literal::FloatLiteral{value: lit}) => Ok(Literal::name_object(name.to_lowercase(), &Literal::float(*lit))),
        _ => Err(ErrorInfo{
            message: "Builtin Wait expect one argument of type int or float | example: Wait(3)".to_owned(),
            interval
        })
    }
}

pub fn text(args: HashMap<String, Literal>, name: String, interval: Interval) -> Result<Literal, ErrorInfo> {
    match args.get("default") {
        Some(literal) => Ok(Literal::name_object(name.to_lowercase(), literal)),
        // Some(Literal::ObjectLiteral{..}) => Ok(Literal::name_object(name.to_lowercase(), &Literal::object(args))),
        _ => Err(ErrorInfo{
                message: "Builtin Text expect one argument of type string | example: Text(\"hola\")".to_owned(),
                interval
        })
    }
}

pub fn img(args: HashMap<String, Literal>, name: String, interval: Interval) -> Result<Literal, ErrorInfo> {
    match args.get("default") {
        Some(Literal::StringLiteral{value: lit}) => Ok(Literal::name_object(name.to_lowercase(), &Literal::string(lit.to_owned()))),
        _ => Err(ErrorInfo{
                message: "Builtin Image expect one argument of type string | example: Image(\"hola\")".to_owned(),
                interval
        })
    }
}

pub fn url(args: HashMap<String, Literal>, name: String, interval: Interval) -> Result<Literal, ErrorInfo> {
    match args.get("default") {
        Some(Literal::StringLiteral{value: lit}) => Ok(Literal::name_object(name.to_lowercase(), &Literal::string(lit.to_owned()))),
        _ => Err(ErrorInfo{
                message: "Builtin Url expect one argument of type string | example: Url(\"hola\")".to_owned(),
                interval
        })
    }
}

pub fn one_of(args: HashMap<String, Literal>, interval: Interval) -> Result<Literal, ErrorInfo> {
    match args.values().nth(rand::thread_rng().gen_range(0, args.len())) {
        Some(lit) => Ok(lit.to_owned()),
        None => Err(ErrorInfo{
                message: "ERROR: Builtin OneOf".to_owned(),
                interval
        })
    }
}

// TODO: see if search_in_obj default value is useful
fn search_or_default(values: &HashMap<String, Literal>, name: &str, interval: &Interval, default: Option<Literal>) -> Result<Literal, ErrorInfo> {
    match (values.get(name), default) {
        (Some(value), ..) => Ok(value.to_owned()),
        (None, Some(default)) => Ok(default.to_owned()),
        (None, None) => {
            match values.get("default") {
                Some(value) => Ok(value.to_owned()),
                None => Err(ErrorInfo{
                        message: format!("No value '{}' or default value found", name),
                        interval: interval.to_owned()
                })
            }
        }
    }
}

fn format_accept(values: Option<&Literal>, title: Literal) -> Literal {
    match values {
        Some(Literal::ArrayLiteral{items, ..}) => {
            items.to_owned().push(title);

            Literal::array(items.to_owned())
        },
        Some(literal) => {
            let items = vec![literal.to_owned(), title];

            Literal::array(items)
        },
        None => Literal::array(vec![title])
    }
}

pub fn button(values: HashMap<String, Literal>, name: String, interval: &Interval) -> Result<Literal, ErrorInfo> {
    let mut button_value = HashMap::new();
    let title = search_or_default(&values, "title", interval, None)?;

    button_value.insert("title".to_owned(), title.to_owned());
    button_value.insert(
        "button_type".to_owned(),
        search_or_default(
            &values,
            "button_type",
            interval,
            Some(Literal::string("quick_button".to_owned()))
        )?
    );
    button_value.insert("accept".to_owned(), format_accept(values.get("accept"), title));
    if let Ok(payload) = search_or_default(&values, "payload", interval, None) {
        button_value.insert("payload".to_owned(), payload);
    }

    Ok(Literal::name_object(name.to_lowercase(), &Literal::object(button_value)))
}

fn accept_to_array(literal: &Literal, mut vec: Vec<Literal>) -> Vec<Literal> {
    match literal {
        Literal::ObjectLiteral{properties, ..} => match properties.get("accept") {
            Some(Literal::ArrayLiteral{items}) => {
                vec.append(&mut items.to_owned());
                vec
            },
            Some(literal) => {
                vec.push(literal.to_owned());
                vec
            },
            None => vec
        }
        _ => vec
    }
}

fn accepts_from_buttons(buttons: &Literal) -> Literal {
    if let Literal::ArrayLiteral{items, ..} = buttons {
        let array = items.iter().fold(vec![], |vec, elem| {
            match elem {
                Literal::NamedLiteral{name, value, ..}  => {
                    if name == "button" {
                        accept_to_array(value, vec)
                    } else {
                        vec
                    }
                },
                _ => vec
            }
        });
        Literal::array(array)
    } else {
        Literal::array(vec![])
    }
}

pub fn question(
    args: HashMap<String, Literal>,
    name: String,
    _interval: Interval
) -> Result<Literal, ErrorInfo> {
    let title = match args.get("title") {
        Some(literal) => literal.to_owned(),
        _ => Literal::string("question".to_owned())
    };
    let buttons = match args.get("buttons") {
        Some(literal) => literal.to_owned(),
        _ => Literal::array(vec![])
    };

    let accepts = accepts_from_buttons(&buttons);
    let mut question = HashMap::new();
    question.insert("title".to_owned(), title);
    question.insert("accepts".to_owned(), accepts);
    question.insert("buttons".to_owned(), buttons);

    Ok(Literal::name_object(name.to_lowercase(), &Literal::object(question)))
}
