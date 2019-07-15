use rand::Rng;
use std::collections::HashMap;
use std::hash::BuildHasher;

use crate::error_format::data::ErrorInfo;
use crate::parser::{ast::{Literal, Interval}, tokens::*};

pub fn typing<S: BuildHasher>(args: &HashMap<String, Literal, S>, interval: Interval) -> Result<Literal, ErrorInfo> {
    if args.len() == 1 {
        match args.get("Numeric") {
            Some(value) => Ok(value.clone()),
            None => Err(ErrorInfo{
                message: "Builtin Typing bad argument type in typing".to_owned(),
                interval
            })
        }
    } else {
        Err(ErrorInfo{
                message: "Builtin Typing bad number of argument".to_owned(),
                interval
        })
    }
}

pub fn wait<S: BuildHasher>(args: &HashMap<String, Literal, S>, interval: Interval) -> Result<Literal, ErrorInfo> {
    if args.len() == 1 {
        match args.get("Numeric") {
            Some(value) => Ok(value.clone()),
            None => Err(ErrorInfo{
                message: "Builtin Typing bad argument type in wait".to_owned(),
                interval
            })
        }
    } else {
        Err(ErrorInfo{
            message: "Builtin Typing bad number of argument".to_owned(),
            interval
        })
    }
}

pub fn text<S: BuildHasher>(args: &HashMap<String, Literal, S>, interval: Interval) -> Result<Literal, ErrorInfo> {
    if args.len() == 1 {
        match args.get("Text") {
            Some(value) => Ok(value.clone()),
            None => Err(ErrorInfo{
                message: "Builtin Typing bad argument type in text".to_owned(),
                interval
            })
        }
    } else {
        Err(ErrorInfo{
                message: "Builtin Typing bad number of argument".to_owned(),
                interval
        })
    }
}

pub fn img<S: BuildHasher>(args: &HashMap<String, Literal, S>, interval: Interval) -> Result<Literal, ErrorInfo> {
    if args.len() == 1 {
        match args.get("Text") {
            Some(value) => Ok(value.clone()),
            None => Err(ErrorInfo{
                message: "Builtin Typing bad argument type in img".to_owned(),
                interval
            }),
        }
    } else {
        Err(ErrorInfo{
                message: "Builtin Typing bad number of argument".to_owned(),
                interval
        })
    }
}

pub fn url<S: BuildHasher>(args: &HashMap<String, Literal, S>, interval: Interval) -> Result<Literal, ErrorInfo> {
    if args.len() == 1 {
        match args.get("Text") {
            Some(value) => Ok(value.clone()),
            None => Err(ErrorInfo{
                message: "Builtin Typing bad argument type in url".to_owned(),
                interval
            }),
        }
    } else {
        Err(ErrorInfo{
            message: "Builtin Typing bad number of argument".to_owned(),
            interval
        })
    }
}

pub fn one_of<S: BuildHasher>(args: &HashMap<String, Literal, S>, _interval: Interval) -> Result<Literal, ErrorInfo> {
    let lit = args
        .values()
        .nth(rand::thread_rng().gen_range(0, args.len()))
        .expect("error in get one_of");
    Ok(lit.clone())
}

fn search_or_default<S: BuildHasher>(values: &HashMap<String, Literal, S>, name: &str, default: &Option<Literal>, interval: &Interval) -> Result<Literal, ErrorInfo> {
    match values.get(name) {
        Some(value) => Ok(value.to_owned()),
        None => {
            match default {
                Some(value) => Ok(value.to_owned()),
                None => Err(ErrorInfo{
                        message: format!("No value '{}' or default value found", name),
                        interval: interval.to_owned()
                })
            }
        }
    }
}

pub fn button<S: BuildHasher>(values: &HashMap<String, Literal, S>, interval: &Interval) -> Result<Literal, ErrorInfo> {
    let mut button_value = HashMap::new();
    let default = get_one_of(values);

    button_value.insert("title".to_owned(), search_or_default(values, "title", &default, interval)?);
    button_value.insert("buttton_type".to_owned(), search_or_default(values, "buttton_type", &default, interval)?);
    button_value.insert("accept".to_owned(), search_or_default(values, "accept", &default, interval)?);
    button_value.insert("key".to_owned(), search_or_default(values, "key", &default, interval)?);
    button_value.insert("value".to_owned(), search_or_default(values, "value", &default, interval)?);
    button_value.insert("payload".to_owned(), search_or_default(values, "payload", &default, interval)?);

    Ok(Literal::ObjectLiteral {
        name: BUTTON.to_owned(),
        value: button_value,
    })
}

fn get_one_of<S: BuildHasher>(map: &HashMap<String, Literal, S>) -> Option<Literal> {
    if let Some(elem) = map.get("Text") {
        return Some(elem.clone());
    }
    if let Some(elem) = map.get("Numeric") {
        return Some(elem.clone());
    }
    if let Some(elem) = map.get("Array") {
        return Some(elem.clone());
    }

    None
}

fn create_accepts_from_list(buttons: &Vec<Literal>) -> Vec<Literal> {
    buttons.iter().fold(vec![], |mut vec, elem| {
            match elem {
                Literal::ObjectLiteral{value, name} if name == BUTTON => {
                    let test = value.get("accept").unwrap().clone();
                    vec.push(test);
                    vec
                },
                _ => unreachable!()
            }
    })

}

pub fn question<S: BuildHasher>(
    args: &HashMap<String, Literal, S>,
    name: String,
    interval: Interval
) -> Result<Literal, ErrorInfo> {
    let mut question_value = HashMap::new();

    let expr_title = args.get("title").expect("error in question expr_title");
    let expr_buttons = args.get("buttons").expect("error in question expr_buttons");

    let mut buttons: Vec<Literal> = vec![];
    if let Literal::ArrayLiteral(array) = expr_buttons {
        for literal in array.iter() {
            match literal {
                Literal::ObjectLiteral { name, .. } if name == BUTTON => {
                    buttons.push(button(args, &interval)?)
                }
                err => return Err(ErrorInfo{
                        message: format!("Builtin Typing bad argument type -> {:?}", err),
                        interval
                    }
                ),
            }
        }

        let accepts = create_accepts_from_list(&buttons);
        question_value.insert("title".to_owned(), expr_title.clone());
        question_value.insert("accepts".to_owned(), Literal::ArrayLiteral(accepts));
        question_value.insert("buttons".to_owned(), Literal::ArrayLiteral(buttons));
    };

    Ok(Literal::ObjectLiteral {
        name: name.to_lowercase().to_owned(),
        value: question_value,
    })
}
