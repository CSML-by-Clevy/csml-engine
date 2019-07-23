use rand::seq::SliceRandom;
use crate::error_format::data::ErrorInfo;
use crate::parser::{ast::{Literal, Interval}, tokens::*};

pub fn typing(args: &Vec<Literal>, name: String, interval: Interval) -> Result<Literal, ErrorInfo> {
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

pub fn wait(args: &Vec<Literal>, name: String, interval: Interval) -> Result<Literal, ErrorInfo> {
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

pub fn text(args: &Vec<Literal>, name: String, interval: Interval) -> Result<Literal, ErrorInfo> {
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

pub fn img(args: &Vec<Literal>, name: String, interval: Interval) -> Result<Literal, ErrorInfo> {
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

pub fn url(args: &Vec<Literal>, name: String, interval: Interval) -> Result<Literal, ErrorInfo> {
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

pub fn one_of(args: &Vec<Literal>, interval: Interval) -> Result<Literal, ErrorInfo> {
    match args.choose(&mut rand::thread_rng()) {
        Some(lit) => Ok(lit.to_owned()),
        None => Err(ErrorInfo{
            message: "Builtin OneOf expect a list of elements | example: OneOf(1, 2, 3)".to_owned(),
            interval
        })
    }
}

fn search_or_default(values: &Vec<Literal>, name: &str, interval: &Interval) -> Result<Literal, ErrorInfo> {
    match Literal::search_in_obj(values, name) {
        Some(value) => Ok(value.to_owned()),
        None => {
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

pub fn button(values: &Vec<Literal>, name: String, interval: &Interval) -> Result<Literal, ErrorInfo> {
    let mut button_value = vec![];

    button_value.push(search_or_default(values, "title", interval)?);
    button_value.push(Literal::string("quick_button".to_owned(), Some("buttton_type".to_owned()))); //search_or_default(values, "buttton_type", interval)?
    button_value.push(search_or_default(values, "accept", interval)?);
    button_value.push(search_or_default(values, "key", interval)?);
    button_value.push(search_or_default(values, "value", interval)?);
    button_value.push(search_or_default(values, "payload", interval)?);

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
    args: &Vec<Literal>,
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
