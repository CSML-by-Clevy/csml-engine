use crate::error_format::data::ErrorInfo;
use crate::parser::{ast::Interval, literal::Literal, tokens::*};
use crate::primitive::{
    array::PrimitiveArray, boolean::PrimitiveBoolean, float::PrimitiveFloat, int::PrimitiveInt,
    object::PrimitiveObject, string::PrimitiveString, PrimitiveType,
};
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn accept_to_array(literal: &HashMap<String, Literal>, mut vec: Vec<Literal>) -> Vec<Literal> {
    match literal.get("accepts") {
        Some(literal) => {
            match Literal::get_value::<Vec<Literal>>(&literal.primitive) {
                Ok(array) => vec.append(&mut array.to_owned()),
                Err(..) => vec.push(literal.to_owned()),
            }
            vec
        }
        None => vec,
    }
}

fn accepts_from_buttons(buttons: &Literal) -> Literal {
    match Literal::get_value::<Vec<Literal>>(&buttons.primitive) {
        Ok(vec) => {
            let array = vec.iter().fold(vec![], |vec, elem| {
                match Literal::get_value::<HashMap<String, Literal>>(&elem.primitive) {
                    Ok(value) => accept_to_array(value, vec),
                    Err(..) => vec,
                }
            });
            PrimitiveArray::get_literal("array", &array, buttons.interval)
        }
        Err(..) => PrimitiveArray::get_literal("array", &[], buttons.interval),
    }
}

fn search_or_default(
    values: &HashMap<String, Literal>,
    name: &str,
    interval: Interval,
    default: Option<Literal>,
) -> Result<Literal, ErrorInfo> {
    match (values.get(name), default) {
        (Some(value), ..) => Ok(value.to_owned()),
        (None, Some(default)) => Ok(default),
        (None, None) => match values.get(DEFAULT) {
            Some(value) => Ok(value.to_owned()),
            None => Err(ErrorInfo {
                message: format!("No value '{}' or default value found", name),
                interval,
            }),
        },
    }
}

fn format_accept(values: Option<&Literal>, title: Literal) -> Literal {
    match values {
        Some(literal) => match Literal::get_value::<Vec<Literal>>(&literal.primitive) {
            Ok(res) => {
                let mut vector = res.clone();

                vector.push(title);

                PrimitiveArray::get_literal("array", &vector, literal.interval)
            }
            Err(_) => {
                let mut vector = Vec::new();

                vector.push(literal.to_owned());
                vector.push(title);

                PrimitiveArray::get_literal("array", &vector, literal.interval)
            }
        },
        None => {
            let mut items = Vec::new();

            items.push(title.to_owned());

            PrimitiveArray::get_literal("array", &items, title.interval)
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

// TODO: check nbr elemts in built-ins
pub fn typing(
    args: HashMap<String, Literal>,
    _name: String,
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    let mut typing: HashMap<String, Literal> = HashMap::new();

    match args.get(DEFAULT) {
        Some(literal)
            if literal.primitive.get_type() == PrimitiveType::PrimitiveInt
                || literal.primitive.get_type() == PrimitiveType::PrimitiveFloat =>
        {
            typing.insert("duration".to_owned(), literal.to_owned());
            Ok(PrimitiveObject::get_literal("typing", &typing, interval))
        }
        _ => Err(ErrorInfo {
            message:
                "Builtin Typing expect one argument of type int or float | example: Typing(3, ..)"
                    .to_owned(),
            interval,
        }),
    }
}

pub fn wait(
    args: HashMap<String, Literal>,
    _name: String,
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    let mut wait: HashMap<String, Literal> = HashMap::new();

    match args.get(DEFAULT) {
        Some(literal)
            if literal.primitive.get_type() == PrimitiveType::PrimitiveInt
                || literal.primitive.get_type() == PrimitiveType::PrimitiveFloat =>
        {
            wait.insert("duration".to_owned(), literal.to_owned());

            Ok(PrimitiveObject::get_literal("wait", &wait, interval))
        }
        _ => Err(ErrorInfo {
            message: "Builtin Wait expect one argument of type int or float | example: Wait(3)"
                .to_owned(),
            interval,
        }),
    }
}

pub fn url(
    args: HashMap<String, Literal>,
    _name: String,
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    let mut url: HashMap<String, Literal> = HashMap::new();

    match &search_or_default(&args, "url", interval, None) {
         Ok(href) if href.primitive.get_type() == PrimitiveType::PrimitiveString => {

            url.insert("url".to_owned(), href.clone());

            if let Ok(title) = search_or_default(&args, "title", interval, Some(href.clone())) {
                url.insert("title".to_owned(), title);
            }
            if let Ok(text) = search_or_default(&args, "text", interval, Some(href.clone())) {
                url.insert("text".to_owned(), text);
            }

            Ok(PrimitiveObject::get_literal("url", &url, interval))
        },
        _ => Err(ErrorInfo{
                message: "Builtin Url expect one argument of type string and 2 optional string argmuments: text, title | example: Url(href = \"hola\", text = \"text\", title = \"title\")".to_owned(),
                interval
        })
    }
}

pub fn img(
    args: HashMap<String, Literal>,
    _name: String,
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    let mut img: HashMap<String, Literal> = HashMap::new();

    match &search_or_default(&args, "url", interval, None) {
        Ok(href) if href.primitive.get_type() == PrimitiveType::PrimitiveString => {
            img.insert("url".to_owned(), href.clone());

            Ok(PrimitiveObject::get_literal("image", &img, interval))
        }
        _ => Err(ErrorInfo {
            message: "Builtin Image expect one argument of type string | example: Image(\"hola\")"
                .to_owned(),
            interval,
        }),
    }
}

pub fn question(
    args: HashMap<String, Literal>,
    _name: String,
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    let mut question: HashMap<String, Literal> = HashMap::new();

    let buttons = match args.get("buttons") {
        Some(literal) => literal.to_owned(),
        _ => {
            return Err(ErrorInfo {
            message: "argument buttons in Builtin Question need to be of type Array of Button Component example: [ Button(\"b1\"), Button(\"b2\") ]".to_owned(),
            interval,
            })
        }
    };

    let accepts = accepts_from_buttons(&buttons);

    if let Ok(title) = search_or_default(&args, "title", interval, None) {
        question.insert("title".to_owned(), title);
    }

    question.insert("accepts".to_owned(), accepts);
    question.insert("buttons".to_owned(), buttons);

    Ok(PrimitiveObject::get_literal(
        "question", &question, interval,
    ))
}

pub fn video(
    args: HashMap<String, Literal>,
    _name: String,
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    let mut video: HashMap<String, Literal> = HashMap::new();

    match &search_or_default(&args, "url", interval, None) {
        Ok(href) if href.primitive.get_type() == PrimitiveType::PrimitiveString => {

            video.insert("url".to_owned(), href.clone());
            match args.get("service") {
                Some(value) if value.primitive.get_type() == PrimitiveType::PrimitiveString => {
                    video.insert("service".to_owned(), value.to_owned())
                }
                _ => None
            };

            Ok(PrimitiveObject::get_literal("video", &video, interval))
        },
        _ => Err(ErrorInfo{
                message: "Builtin Video expect one argument of type string and 1 optional 'service' argument of type string | example: Video(url = \"hola\", service = \"text\")".to_owned(),
                interval
        })
    }
}

pub fn audio(
    args: HashMap<String, Literal>,
    _name: String,
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    let mut audio: HashMap<String, Literal> = HashMap::new();

    match &search_or_default(&args, "url", interval, None) {
        Ok(href) if href.primitive.get_type() == PrimitiveType::PrimitiveString => {

            audio.insert("url".to_owned(), href.clone());

            match args.get("service") {
                Some(value) if value.primitive.get_type() == PrimitiveType::PrimitiveString => {
                    audio.insert("service".to_owned(), value.to_owned())
                }
                _ => None
            };

            Ok(PrimitiveObject::get_literal("audio", &audio, interval))

        },
        _ => Err(ErrorInfo{
                message: "Builtin Audio expect one argument of type string and 1 optional 'service' argument of type string | example: Audio(url = \"hola\", service = \"text\")".to_owned(),
                interval
        })
    }
}

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

pub fn object(object: HashMap<String, Literal>, interval: Interval) -> Result<Literal, ErrorInfo> {
    Ok(PrimitiveObject::get_literal("object", &object, interval))
}

pub fn one_of(
    args: HashMap<String, Literal>,
    one_of_inter: Interval,
) -> Result<Literal, ErrorInfo> {
    match args.get(DEFAULT) {
        Some(literal) => {
            match Literal::get_value::<Vec<Literal>>(&literal.primitive) {
                Ok(res) => {
                    match res.get(rand::thread_rng().gen_range(0, res.len())) {
                        Some(lit) => Ok(lit.to_owned()),
                        None => Err(ErrorInfo{
                            message: "ERROR: Builtin OneOf expect one value of type Array | example: OneOf( [1, 2, 3] )".to_owned(),
                            interval: literal.interval,
                        })
                    }
                }
                Err(_) => Err(ErrorInfo{
                        message: "ERROR: Builtin OneOf expect one value of type Array | example: OneOf( [1, 2, 3] )".to_owned(),
                        interval: literal.interval,
                    })
            }
        }
        None => Err(ErrorInfo {
            message:
                "ERROR: Builtin OneOf expect one value of type Array | example: OneOf( [1, 2, 3] )"
                    .to_owned(),
            interval: one_of_inter,
        }),
    }
}

pub fn shuffle(args: HashMap<String, Literal>, interval: Interval) -> Result<Literal, ErrorInfo> {
    match args.get(DEFAULT) {
        Some(literal) => {
            match Literal::get_value::<Vec<Literal>>(&literal.primitive) {
                Ok(res) => {
                    let mut vec = res.to_owned();
                    vec.shuffle(&mut rand::thread_rng());

                    Ok(PrimitiveArray::get_literal("array", &vec, literal.interval))
                }
                Err(_) => Err(ErrorInfo{
                        message: "ERROR: Builtin Shuffle expect one value of type Array | example: Shuffle( [1, 2, 3] )".to_owned(),
                        interval
                })
            }
        }
        None => Err(ErrorInfo{
                message: "ERROR: Builtin Shuffle expect one value of type Array | example: Shuffle( [1, 2, 3] )".to_owned(),
                interval
        })
    }
}

pub fn length(args: HashMap<String, Literal>, interval: Interval) -> Result<Literal, ErrorInfo> {
    match args.get(DEFAULT) {
        Some(literal) => {
            if let Ok(res) = Literal::get_value::<Vec<Literal>>(&literal.primitive) {
                return Ok(PrimitiveInt::get_literal("int", res.len() as i64, literal.interval))
            }
            if let Ok(res) = Literal::get_value::<String>(&literal.primitive) {
                return Ok(PrimitiveInt::get_literal("int", res.len() as i64, literal.interval))
            }

            Err(ErrorInfo{
                message: "ERROR: Builtin Length expect one value of type Array or String | example: Length( value )".to_owned(),
                interval
            })
        }
        None => Err(ErrorInfo{
            message: "ERROR: Builtin Length expect one value of type Array or String | example: Length( value )".to_owned(),
            interval
        })
    }
}

pub fn find(args: HashMap<String, Literal>, interval: Interval) -> Result<Literal, ErrorInfo> {
    let mut string = None;
    let mut case = false;

    if let Some(literal) = args.get("in") {
        if let Ok(res) = Literal::get_value::<String>(&literal.primitive) {
            string = Some(res);
        }
    } else if string.is_none() {
        return Err(ErrorInfo{
            message: "ERROR: Builtin Find expect in to be of type String | example: Contain(value, in = \"hola\", case_sensitive = true)".to_owned(),
            interval
        });
    }

    if let Some(literal) = args.get("in") {
        if let Ok(res) = Literal::get_value::<bool>(&literal.primitive) {
            case = *res;
        }
    }

    match (args.get(DEFAULT), string) {
        (Some(literal), Some(string)) => {
            match Literal::get_value::<String>(&literal.primitive) {
                Ok(res) => {
                    if case {
                        Ok(PrimitiveBoolean::get_literal("boolean", string.contains(res), interval))
                    } else {
                        Ok(PrimitiveBoolean::get_literal("boolean", string.to_lowercase().contains(&res.to_lowercase()), interval))
                    }
                }
                Err(_) => Err(ErrorInfo{
                    message: "ERROR: Builtin Find expect value to be of type String | example: Find(value, in = \"hola\", case_sensitive = true)".to_owned(),
                    interval
                })
            }
        }
        (_, _) => Err(ErrorInfo{
            message: "ERROR: Builtin Find expect value to be of type String | example: Find(value, in = \"hola\", case_sensitive = true)".to_owned(),
            interval
        })
    }
}

pub fn random(interval: Interval) -> Result<Literal, ErrorInfo> {
    let mut rng = rand::thread_rng();

    let random: f64 = rng.gen();

    Ok(PrimitiveFloat::get_literal("float", random, interval))
}

pub fn floor(args: HashMap<String, Literal>, interval: Interval) -> Result<Literal, ErrorInfo> {
    match args.get(DEFAULT) {
        Some(literal) => match Literal::get_value::<f64>(&literal.primitive) {
            Ok(res) => Ok(PrimitiveFloat::get_literal(
                "float",
                res.floor(),
                literal.interval,
            )),
            Err(_) => Err(ErrorInfo {
                message:
                    "ERROR: Builtin Floor expect one argument of type float| example: Floor(4.2)"
                        .to_owned(),
                interval,
            }),
        },
        _ => Err(ErrorInfo {
            message: "ERROR: Builtin Floor expect one argument of type float| example: Floor(4.2)"
                .to_owned(),
            interval,
        }),
    }
}

pub fn text(
    args: HashMap<String, Literal>,
    _name: String,
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    match args.get(DEFAULT) {
        Some(literal) => Ok(PrimitiveString::get_literal(
            "string",
            &literal.primitive.to_string(),
            literal.interval,
        )),
        _ => Err(ErrorInfo {
            message: "Builtin Text expect one argument of type string | example: Text(\"hola\")"
                .to_owned(),
            interval,
        }),
    }
}
