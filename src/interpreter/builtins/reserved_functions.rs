use rand::Rng;
use std::collections::HashMap;
use crate::error_format::data::ErrorInfo;
use crate::parser::{ast::{Literal, Interval}}; //, tokens::*

// TODO: check nbr elemts in built-ins

pub fn typing(args: HashMap<String, Literal>, name: String, interval: Interval) -> Result<Literal, ErrorInfo> {
    match args.get("default") {
        Some(Literal::IntLiteral{..}) => Ok(Literal::lit_to_obj(Literal::object(args), name.to_lowercase())),
        Some(Literal::FloatLiteral{..}) => Ok(Literal::lit_to_obj(Literal::object(args), name.to_lowercase())),
        None => return Err(ErrorInfo{
                message: "Builtin Typing expect one argument of type int or float | example: Typing(3, ..)".to_owned(),
                interval
        })
    }
}

pub fn wait(args: HashMap<String, Literal>, name: String, interval: Interval) -> Result<Literal, ErrorInfo> {
    match args.get("default") {
        Some(Literal::IntLiteral{..}) => Ok(Literal::lit_to_obj(Literal::object(args), name.to_lowercase())),
        Some(Literal::FloatLiteral{..}) => Ok(Literal::lit_to_obj(Literal::object(args), name.to_lowercase())),
        None => return Err(ErrorInfo{
                message: "Builtin Wait expect one argument of type int or float | example: Wait(3)".to_owned(),
                interval
        })
    }
}

pub fn text(args: HashMap<String, Literal>, name: String, interval: Interval) -> Result<Literal, ErrorInfo> {
    match args.get("default") {
        Some(Literal::StringLiteral{..}) => Ok(Literal::lit_to_obj(Literal::object(args), name.to_lowercase())),
        None => return Err(ErrorInfo{
                message: "Builtin Text expect one argument of type string | example: Text(\"hola\")".to_owned(),
                interval
        })
    }
}

pub fn img(args: HashMap<String, Literal>, name: String, interval: Interval) -> Result<Literal, ErrorInfo> {
    match args.get("default") {
        Some(Literal::StringLiteral{..}) => Ok(Literal::lit_to_obj(Literal::object(args), name.to_lowercase())),
        None => return Err(ErrorInfo{
                message: "Builtin Image expect one argument of type string | example: Image(\"hola\")".to_owned(),
                interval
        })
    }
}

pub fn url(args: HashMap<String, Literal>, name: String, interval: Interval) -> Result<Literal, ErrorInfo> {
    match args.get("default") {
        Some(Literal::StringLiteral{..}) => Ok(Literal::lit_to_obj(Literal::object(args), name.to_lowercase())),
        None => return Err(ErrorInfo{
                message: "Builtin Url expect one argument of type string | example: Url(\"hola\")".to_owned(),
                interval
        })
    }
}

pub fn one_of(args: HashMap<String, Literal>, interval: Interval) -> Result<Literal, ErrorInfo> {
    let lit = args.values()
        .nth(rand::thread_rng().gen_range(0, args.len()))
        .expect("Error in get one_of");
    Ok(lit.to_owned())
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
        None => title
    }
}

pub fn button(values: HashMap<String, Literal>, name: String, interval: &Interval) -> Result<Literal, ErrorInfo> {
    let mut button_value = HashMap::new();
    let title = search_or_default(&values, "title", interval, None)?;

    button_value.insert("title".to_owned(), title.to_owned());
    button_value.insert(
        "buttton_type".to_owned(),
        search_or_default(
            &values,
            "buttton_type",
            interval,
            Some(Literal::string("quick_button".to_owned()))
        )?
    );
    button_value.insert("accept".to_owned(), format_accept(values.get("accept"), title));
    button_value.insert("key".to_owned(), search_or_default(&values, "key", interval, None)?);
    button_value.insert("value".to_owned(), search_or_default(&values, "value", interval, None)?);
    button_value.insert("payload".to_owned(), search_or_default(&values, "payload", interval, None)?);

    Ok(Literal::lit_to_obj(Literal::object(button_value), name))
}

fn create_accepts_from_buttons(buttons: &Literal) -> Literal {
    if let Literal::ArrayLiteral{items, ..} = buttons {
        let array = items.iter().fold(vec![], |mut vec, elem| {
            match elem {
                Literal::ObjectLiteral{properties, ..}  => {
                    if let Some(elem) = properties.get("accept") {
                        vec.push(elem.to_owned());
                    }
                    vec
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

    let accepts = create_accepts_from_buttons(&buttons);
    let mut question = HashMap::new();
    question.insert("title".to_owned(), title);
    question.insert("accepts".to_owned(), accepts);
    question.insert("buttons".to_owned(), buttons);

    Ok(Literal::lit_to_obj(Literal::object(question), name.to_lowercase().to_owned()))
}
