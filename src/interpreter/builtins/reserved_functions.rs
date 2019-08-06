use rand::seq::SliceRandom;
use crate::error_format::data::ErrorInfo;
use crate::parser::{ast::{Literal, Interval}, tokens::*};

pub fn typing(args: &[Literal], name: String, interval: Interval) -> Result<Literal, ErrorInfo> {
    let var = match Literal::search_in_obj(args, "default") {
        Some(literal) => literal.set_name("value".to_owned()),
        None => return Err(ErrorInfo{
                message: "Builtin Typing expect one argument of type int or float | example: Typing(3)".to_owned(),
                interval
        })
    };

    match var {
        Literal::IntLiteral{..}  => Ok(Literal::object(vec![var.clone()], Some(name.to_lowercase()))),
        Literal::FloatLiteral{..}  => Ok(Literal::object(vec![var.clone()], Some(name.to_lowercase()))),
        _ => Err(ErrorInfo{
            message: "Builtin Typing expect one argument of type int or float | example: Typing(3)".to_owned(),
            interval
        })
    }
}

pub fn wait(args: &[Literal], name: String, interval: Interval) -> Result<Literal, ErrorInfo> {
    let var = match Literal::search_in_obj(args, "default") {
        Some(literal) => literal.set_name("value".to_owned()),
        None => return Err(ErrorInfo{
                message: "Builtin Wait expect one argument of type int or float | example: Wait(3)".to_owned(),
                interval
        })
    };

    match var {
        Literal::IntLiteral{..}  => Ok(Literal::object(vec![var.clone()], Some(name.to_lowercase()))),
        Literal::FloatLiteral{..}  => Ok(Literal::object(vec![var.clone()], Some(name.to_lowercase()))),
        _ => Err(ErrorInfo{
            message: "Builtin Wait expect one argument of type int or float | example: Wait(3)".to_owned(),
            interval
        })
    }
}

pub fn text(args: &[Literal], name: String, interval: Interval) -> Result<Literal, ErrorInfo> {
    let var = match Literal::search_in_obj(args, "default") {
        Some(literal) => literal.set_name("value".to_owned()),
        None => return Err(ErrorInfo{
                message: "Builtin Text expect one argument of type string | example: Text(\"hola\")".to_owned(),
                interval
        })
    };

    match var {
        Literal::StringLiteral{..}  => Ok(Literal::object(vec![var.clone()], Some(name.to_lowercase()))),
        _ => Err(ErrorInfo{
            message: "Builtin Text expect one argument of type string | example: Text(\"hola\")".to_owned(),
            interval
        })
    }
}

pub fn img(args: &[Literal], name: String, interval: Interval) -> Result<Literal, ErrorInfo> {
    let var = match Literal::search_in_obj(args, "default") {
        Some(literal) => literal.set_name("value".to_owned()),
        None => return Err(ErrorInfo{
                message: "Builtin Image expect one argument of type string | example: Image(\"hola\")".to_owned(),
                interval
        })
    };

    match var {
        Literal::StringLiteral{..}  => Ok(Literal::object(vec![var.clone()], Some(name.to_lowercase()))),
        _ => Err(ErrorInfo{
            message: "Builtin Image expect one argument of type string | example: Image(\"hola\")".to_owned(),
            interval
        })
    }
}

pub fn url(args: &[Literal], name: String, interval: Interval) -> Result<Literal, ErrorInfo> {
    let var = match Literal::search_in_obj(args, "default") {
        Some(literal) => literal.set_name("value".to_owned()),
        None => return Err(ErrorInfo{
                message: "Builtin Url expect one argument of type string | example: Url(\"hola\")".to_owned(),
                interval
        })
    };

    match var {
        Literal::StringLiteral{..} => Ok(Literal::object(vec![var.clone()], Some(name.to_lowercase()))),
        _ => Err(ErrorInfo{
            message: "Builtin Url expect one argument of type string | example: Url(\"hola\")".to_owned(),
            interval
        })
    }
}

pub fn one_of(args: &[Literal], interval: Interval) -> Result<Literal, ErrorInfo> {
    match args.choose(&mut rand::thread_rng()) {
        Some(lit) => Ok(lit.to_owned()),
        None => Err(ErrorInfo{
            message: "Builtin OneOf expect a list of elements | example: OneOf(1, 2, 3)".to_owned(),
            interval
        })
    }
}

// TODO: see if search_in_obj default value is useful
fn search_or_default(values: &[Literal], name: &str, interval: &Interval, default: Option<&str>) -> Result<Literal, ErrorInfo> {
    match (Literal::search_in_obj(values, name), default) {
        (Some(value), ..) => Ok(value.to_owned()),
        (None, Some(default)) => Ok(Literal::string(default.to_owned(), Some(name.to_owned()))),
        (None, None) => {
            match Literal::search_in_obj(values, "default") {
                Some(value) => Ok(value.set_name(name.to_owned())),
                None => Err(ErrorInfo{
                        message: format!("No value '{}' or default value found", name),
                        interval: interval.to_owned()
                })
            }
        }
    }
}

fn format_accept(values: &[Literal], name: &str, title: Literal) -> Literal {
    match Literal::search_in_obj(values, name) {
        Some(Literal::ArrayLiteral{items, ..}) => {
            items.to_owned().push(title);
            Literal::array(
                items.to_owned(),
                Some(name.to_owned())
            )
        },
        Some(literal) => {
            let items = dbg!(vec![literal.to_owned(), title]);
            
            Literal::array(
                items,
                Some(name.to_owned())
            )
        },
        None => Literal::array(
            vec![title],
            Some(name.to_owned())
        )
    }
}

pub fn button(values: &[Literal], name: String, interval: &Interval) -> Result<Literal, ErrorInfo> {
    let mut button_value = vec![];

    let title = search_or_default(values, "title", interval, None)?;
    button_value.push(title.clone());
    button_value.push(search_or_default(values, "buttton_type", interval, Some("quick_button"))?);

    button_value.push(search_or_default(values, "accept", interval, None)?);
    format_accept(values, "accept", title);

    button_value.push(search_or_default(values, "key", interval, None)?);
    button_value.push(search_or_default(values, "value", interval, None)?);
    button_value.push(search_or_default(values, "payload", interval, None)?);

    Ok(Literal::object(button_value, Some(name)))
}

fn create_accepts_from_list(buttons: &Literal) -> Literal {
    if let Literal::ArrayLiteral{items, ..} = buttons {
        let array = items.iter().fold(vec![], |mut vec, elem| {
            match elem {
                Literal::ObjectLiteral{properties, name: Some(name), ..} if name == BUTTON => {
                    if let Some(elem) = Literal::search_in_obj(properties, "accept") {
                        vec.push(elem.to_owned());
                    }
                    vec
                },
                _ => vec
            }
        });
        Literal::object(array, Some("accepts".to_owned()))
    } else {
        Literal::object(vec![], Some("accepts".to_owned()))
    }
}

pub fn question(
    args: &[Literal],
    name: String,
    _interval: Interval
) -> Result<Literal, ErrorInfo> {
    let title = match Literal::search_in_obj(args, "title") {
        Some(literal) => literal.to_owned(),
        _ => Literal::string("question".to_owned(), Some("title".to_owned()))
    };
    let buttons = match Literal::search_in_obj(args, "buttons") {
        Some(literal) => literal.to_owned(),
        _ => Literal::array(vec![], Some("buttons".to_owned()))
    };

    let mut question_value = vec![];
    let accepts = create_accepts_from_list(&buttons);
    question_value.push(title);
    question_value.push(accepts);
    question_value.push(buttons);

    Ok(Literal::object(question_value, Some(name.to_lowercase().to_owned())))
}
