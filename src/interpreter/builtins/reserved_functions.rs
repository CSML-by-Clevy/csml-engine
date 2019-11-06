use crate::error_format::data::ErrorInfo;
use crate::parser::{ast::Interval, literal::Literal, tokens::*};
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;

// TODO: check nbr elemts in built-ins
pub fn typing(
    args: HashMap<String, Literal>,
    name: String,
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    match args.get(DEFAULT) {
        Some(Literal::IntLiteral {
            value: lit,
            interval,
        }) => Ok(Literal::name_object(
            name.to_lowercase(),
            &Literal::name_object(
                "duration".to_owned(),
                &Literal::int(*lit, interval.to_owned()),
                interval.clone()
            ),
            interval.to_owned(),
        )),
        Some(Literal::FloatLiteral {
            value: lit,
            interval,
        }) => Ok(Literal::name_object(
            name.to_lowercase(),
            &Literal::name_object(
                "duration".to_owned(),
                &Literal::float(*lit, interval.to_owned()),
                interval.clone()
            ),
            interval.to_owned(),
        )),
        _ => Err(ErrorInfo {
            message:
                "Builtin Typing expect one argument of type int or float | example: Typing(3, ..)"
                    .to_owned(),
            interval,
        }),
    }
}

pub fn object(object: HashMap<String, Literal>, intrerval: Interval) -> Result<Literal, ErrorInfo> {
    Ok(Literal::object(object, intrerval))
}

pub fn wait(
    args: HashMap<String, Literal>,
    name: String,
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    match args.get(DEFAULT) {
        Some(Literal::IntLiteral {
            value: lit,
            interval,
        }) => Ok(Literal::name_object(
            name.to_lowercase(),
            &Literal::name_object(
                "duration".to_owned(),
                &Literal::int(*lit, interval.to_owned()),
                interval.clone()
            ),
            interval.to_owned(),
        )),
        Some(Literal::FloatLiteral {
            value: lit,
            interval,
        }) => Ok(Literal::name_object(
            name.to_lowercase(),
            &Literal::name_object(
                "duration".to_owned(),
                &Literal::float(*lit, interval.to_owned()),
                interval.clone()
            ),
            interval.to_owned(),
        )),
        _ => Err(ErrorInfo {
            message: "Builtin Wait expect one argument of type int or float | example: Wait(3)"
                .to_owned(),
            interval,
        }),
    }
}

pub fn text(
    args: HashMap<String, Literal>,
    name: String,
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    match args.get(DEFAULT) {
        Some(literal) => {
            let interval = literal.get_interval();
            Ok(Literal::name_object(
                name.to_lowercase(),
                &Literal::name_object(
                    name.to_lowercase(),
                    literal,
                    interval.clone()
                ),
                interval,
            ))
        },
        _ => Err(ErrorInfo {
            message: "Builtin Text expect one argument of type string | example: Text(\"hola\")"
                .to_owned(),
            interval,
        }),
    }
}

pub fn img(
    args: HashMap<String, Literal>,
    name: String,
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    let mut properties = HashMap::new();

    match &search_or_default(&args, "url", &interval, None) {
        Ok(href) if href.is_string() => {

            properties.insert("url".to_owned(), href.clone());

            Ok(
                Literal::name_object(
                    name.to_lowercase(),
                    &Literal::object(properties, interval.clone()),
                    interval
                )
            )
        },
        _ => Err(ErrorInfo{
                message: "Builtin Image expect one argument of type string | example: Image(\"hola\")".to_owned(),
                interval
        })
    }
}

pub fn video(
    args: HashMap<String, Literal>,
    name: String,
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    let mut properties = HashMap::new();

    match &search_or_default(&args, "url", &interval, None) {
        Ok(href) if href.is_string() => {

            properties.insert("url".to_owned(), href.clone());
            match args.get("service") {
                Some(value) if value.is_string() => properties.insert("service".to_owned(), value.to_owned()),
                _ => None
            };

            Ok(
                Literal::name_object(
                    name.to_lowercase(),
                    &Literal::object(properties, interval.clone()),
                    interval
                )
            )
        },
        _ => Err(ErrorInfo{
                message: "Builtin Video expect one argument of type string and 1 optional 'service' argument of type string | example: Video(url = \"hola\", service = \"text\")".to_owned(),
                interval
        })
    }
}

pub fn audio(
    args: HashMap<String, Literal>,
    name: String,
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    let mut url = HashMap::new();

    match &search_or_default(&args, "url", &interval, None) {
        Ok(href) if href.is_string() => {

            url.insert("url".to_owned(), href.clone());

            match args.get("service") {
                Some(value) if value.is_string() => url.insert("service".to_owned(), value.to_owned()),
                _ => None
            };

            Ok(
                Literal::name_object(
                    name.to_lowercase(),
                    &Literal::object(url, interval.clone()),
                    interval
                )
            )
        },
        _ => Err(ErrorInfo{
                message: "Builtin Audio expect one argument of type string and 1 optional 'service' argument of type string | example: Audio(url = \"hola\", service = \"text\")".to_owned(),
                interval
        })
    }
}

pub fn url(
    args: HashMap<String, Literal>,
    name: String,
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    let mut url = HashMap::new();

    match &search_or_default(&args, "url", &interval, None) {
        Ok(href) if href.is_string() => {

            url.insert("url".to_owned(), href.clone());
            if let Ok(title) = search_or_default(&args, "title", &interval, Some(href.clone())) {
                url.insert("title".to_owned(), title.to_owned());
            }
            if let Ok(text) = search_or_default(&args, "text", &interval, Some(href.clone())) {
                url.insert("text".to_owned(), text.to_owned());
            }

            Ok(
                Literal::name_object(
                    name.to_lowercase(),
                    &Literal::object(url, interval.clone()),
                    interval
                )
            )
        },
        _ => Err(ErrorInfo{
                message: "Builtin Url expect one argument of type string and 2 optional string argmuments: text, title | example: Url(href = \"hola\", text = \"text\", title = \"title\")".to_owned(),
                interval
        })
    }
}

pub fn one_of(
    args: HashMap<String, Literal>,
    one_of_inter: Interval,
) -> Result<Literal, ErrorInfo> {
    match args.get(DEFAULT) {
        Some(Literal::ArrayLiteral { items, interval }) => {
            match items.get(rand::thread_rng().gen_range(0, items.len())) {
                Some(lit) => Ok(lit.to_owned()),
                None => Err(ErrorInfo{
                    message: "ERROR: Builtin OneOf expect one value of type Array | example: OneOf( [1, 2, 3] )".to_owned(),
                    interval: interval.to_owned()
                })
            }
        }
        _ => Err(ErrorInfo {
            message:
                "ERROR: Builtin OneOf expect one value of type Array | example: OneOf( [1, 2, 3] )"
                    .to_owned(),
            interval: one_of_inter,
        }),
    }
}

pub fn shuffle(args: HashMap<String, Literal>, interval: Interval) -> Result<Literal, ErrorInfo> {
    match args.get(DEFAULT)  {
        Some(Literal::ArrayLiteral{items, interval}) => {
            let mut vec = items.to_owned();
            vec.shuffle(&mut rand::thread_rng());
            Ok(Literal::array(vec, interval.to_owned()))
        },
        _ => Err(ErrorInfo{
                message: "ERROR: Builtin Shuffle expect one value of type Array | example: Shuffle( [1, 2, 3] )".to_owned(),
                interval
        })
    }
}

pub fn length(args: HashMap<String, Literal>, interval: Interval) -> Result<Literal, ErrorInfo> {
    match args.get(DEFAULT) {
        Some(Literal::StringLiteral{value, interval}) => {
            Ok(Literal::int(value.len() as i64, interval.to_owned()))
        },
        Some(Literal::ArrayLiteral{items, interval}) => {
            Ok(Literal::int(items.len() as i64, interval.to_owned()))
        },
        _ => Err(ErrorInfo{
                message: "ERROR: Builtin Lenght expect one value of type Array or String | example: Lenght( value )".to_owned(),
                interval
        })
    }
}

pub fn find(args: HashMap<String, Literal>, interval: Interval) -> Result<Literal, ErrorInfo> {
    let mut string = None;
    let mut case = false;

    if let Some(Literal::StringLiteral { value, .. }) = args.get("in") {
        string = Some(value);
    } else if string.is_none() {
        return Err(ErrorInfo{
            message: "ERROR: Builtin Find expect in to be of type String | example: Contain(value, in = \"hola\", case_sensitive = true)".to_owned(),
            interval
        });
    }
    if let Some(Literal::BoolLiteral { value, .. }) = args.get("case_sensitive") {
        case = *value;
    }

    match (args.get(DEFAULT), string) {
        (Some(Literal::StringLiteral{value, interval}), Some(string)) => {
            if case {
                Ok(Literal::boolean(string.contains(value), interval.to_owned()))
            } else {
                Ok(Literal::boolean(string.to_lowercase().contains(&value.to_lowercase()), interval.to_owned()))
            }
        },
        (_, _) => Err(ErrorInfo{
            message: "ERROR: Builtin Find expect value to be of type String | example: Find(value, in = \"hola\", case_sensitive = true)".to_owned(),
            interval
        })
    }
}

pub fn random(interval: &Interval) -> Result<Literal, ErrorInfo> {
    let mut rng = rand::thread_rng();
    Ok(Literal::float(rng.gen(), interval.to_owned()))
}

pub fn floor(args: HashMap<String, Literal>, interval: Interval) -> Result<Literal, ErrorInfo> {
    match args.get(DEFAULT) {
        Some(Literal::FloatLiteral { value, interval }) => {
            Ok(Literal::float(value.floor(), interval.to_owned()))
        }
        _ => Err(ErrorInfo {
            message: "ERROR: Builtin Floor expect one argument of type float| example: Floor(4.2)"
                .to_owned(),
            interval,
        }),
    }
}

// TODO: refactor search_or_default
fn search_or_default(
    values: &HashMap<String, Literal>,
    name: &str,
    interval: &Interval,
    default: Option<Literal>,
) -> Result<Literal, ErrorInfo> {
    match (values.get(name), default) {
        (Some(value), ..) => Ok(value.to_owned()),
        (None, Some(default)) => Ok(default.to_owned()),
        (None, None) => match values.get(DEFAULT) {
            Some(value) => Ok(value.to_owned()),
            None => Err(ErrorInfo {
                message: format!("No value '{}' or default value found", name),
                interval: interval.to_owned(),
            }),
        },
    }
}

fn format_accept(values: Option<&Literal>, title: Literal) -> Literal {
    match values {
        Some(Literal::ArrayLiteral { items, interval }) => {
            let mut val = items.to_owned();

            val.push(title);
            Literal::array(val, interval.to_owned())
        }
        Some(literal) => {
            let items = vec![literal.to_owned(), title];

            Literal::array(items, literal.get_interval())
        }
        None => {
            let interval = title.get_interval();
            Literal::array(vec![title], interval)
        }
    }
}

pub fn button(
    values: HashMap<String, Literal>,
    name: String,
    interval: &Interval,
) -> Result<Literal, ErrorInfo> {
    let mut button_value = HashMap::new();

    let title = search_or_default(&values, "title", interval, None)?;

    button_value.insert("title".to_owned(), title.to_owned());
    button_value.insert(
        "button_type".to_owned(),
        search_or_default(
            &values,
            "button_type",
            interval,
            Some(Literal::string(
                "quick_button".to_owned(),
                interval.to_owned(),
            )),
        )?,
    );

    button_value.insert(
        "accept".to_owned(),
        format_accept(values.get("accept"), title),
    );
    if let Ok(payload) = search_or_default(&values, "payload", interval, None) {
        button_value.insert("payload".to_owned(), payload);
    }

    Ok(Literal::name_object(
        name.to_lowercase(),
        &Literal::object(button_value, interval.to_owned()),
        interval.to_owned(),
    ))
}

fn accept_to_array(literal: &Literal, mut vec: Vec<Literal>) -> Vec<Literal> {
    match literal {
        Literal::ObjectLiteral { properties, .. } => match properties.get("accept") {
            Some(Literal::ArrayLiteral { items, .. }) => {
                vec.append(&mut items.to_owned());
                vec
            }
            Some(literal) => {
                vec.push(literal.to_owned());
                vec
            }
            None => vec,
        },
        _ => vec,
    }
}

fn accepts_from_buttons(buttons: &Literal) -> Literal {
    if let Literal::ArrayLiteral { items, interval } = buttons {
        let array = items.iter().fold(vec![], |vec, elem| match elem {
            Literal::ObjectLiteral { properties, .. } => {
                if let Some(value) = properties.get("button") {
                    accept_to_array(value, vec)
                } else {
                    vec
                }
            }
            _ => vec,
        });
        Literal::array(array, interval.to_owned())
    } else {
        Literal::array(vec![], buttons.get_interval())
    }
}

fn is_button(literal: &Literal) -> bool {
    match literal {
        Literal::FunctionLiteral{name, ..} if name.to_lowercase() == "button" => {
            true
        }
        Literal::ObjectLiteral{properties, ..} if properties.get("button").is_some() => {
            true
        }
        _ => false
    }
}

fn if_buttons(literal: &Literal) -> bool {
    if let Literal::ArrayLiteral{items, ..} = literal {
        for elem in items {
            if !is_button(elem) {
                return false
            }
        }
        true
    } else {
        false
    }
}

pub fn question(
    args: HashMap<String, Literal>,
    name: String,
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    let buttons = match args.get("buttons") {
        Some(literal) if if_buttons(literal) => literal.to_owned(),
        _ => return Err(ErrorInfo {
            message: "ERROR: argument buttons in Builtin Question need to be of type Array of Button Component example: [ Button(\"b1\"), Button(\"b2\") ]".to_owned(),
            interval,
        }),
    };

    let accepts = accepts_from_buttons(&buttons);
    let mut question = HashMap::new();

    if let Ok(title) = search_or_default(&args, "title", &interval, None) {
        question.insert("title".to_owned(), title);
    }
    question.insert("accepts".to_owned(), accepts);
    question.insert("buttons".to_owned(), buttons);

    Ok(Literal::name_object(
        name.to_lowercase(),
        &Literal::object(question, interval.clone()),
        interval,
    ))
}
