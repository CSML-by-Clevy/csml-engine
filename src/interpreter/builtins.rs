use rand::Rng;
use std::io::{Error, ErrorKind, Result};
use serde_json::{Value, Map};
use std::collections::HashMap;
use crate::lexer::token::*;
use crate::parser::ast::{Expr, Literal, Ident};
use crate::interpreter:: {
    message::*,
    json_to_rust::*,
    variable_handler::*,
};

pub fn remember(name: String, value: String) -> MessageType {
    MessageType::Assign{name, value}
}

pub fn typing(args: &Expr, name: String) -> Result<MessageType> {
    if let Expr::VecExpr(vec) = args {
        if vec.len() == 1 {
            if let Expr::LitExpr(Literal::IntLiteral(_)) = &vec[0] {
                return Ok(MessageType::Msg(Message::new(&vec[0], name)));
            }
        }
        return Err(Error::new(ErrorKind::Other, "Builtin Typing bad argument"))
    }
    
    Err(Error::new(ErrorKind::Other, "Builtin Typing bad argument"))
}

pub fn wait(args: &Expr, name: String) -> Result<MessageType> {
    if let Expr::VecExpr(vec) = args {
        if vec.len() == 1 {
            if let Expr::LitExpr(Literal::IntLiteral(_)) = &vec[0] {
                return Ok(MessageType::Msg(Message::new(&vec[0], name)));
            }
        }
        return Err(Error::new(ErrorKind::Other, "Builtin Wait bad argument"))
    }

    Err(Error::new(ErrorKind::Other, "Builtin Wait bad argument"))
}

pub fn text(args: &Expr, name: String) -> Result<MessageType> {
    if let Expr::VecExpr(vec) = args {
        if vec.len() == 1 {
            if let Expr::LitExpr(_) = &vec[0] {
                return Ok(MessageType::Msg(Message::new(&vec[0], name)));
            }
        }
        return Err(Error::new(ErrorKind::Other, "Builtin Text bad argument"))
    }

    Err(Error::new(ErrorKind::Other, "Builtin Text bad argument"))
}

pub fn img(args: &Expr, name: String) -> Result<MessageType> {
    if let Expr::VecExpr(vec) = args {
        if vec.len() == 1 {
            if let Expr::LitExpr(_) = &vec[0] {
                return Ok(MessageType::Msg(Message::new(&vec[0], name)));
            }
        }
        return Err(Error::new(ErrorKind::Other, "Builtin Image bad argument"))
    }

    Err(Error::new(ErrorKind::Other, "Builtin Image bad argument"))
}

pub fn url(args: &Expr, name: String) -> Result<MessageType>{
    if let Expr::VecExpr(vec) = args {
        if vec.len() == 1 {
            if let Expr::LitExpr(_) = &vec[0] {
                return Ok(MessageType::Msg(Message::new(&vec[0], name)));
            }
        }
        return Err(Error::new(ErrorKind::Other, "Builtin Url bad argument"))
    }

    Err(Error::new(ErrorKind::Other, "Builtin Url bad argument"))
}

pub fn one_of(args: &Expr, elem_type: String, memory: &Memory, event: &Option<Event>) -> Result<MessageType> {
    if let Expr::VecExpr(vec) = args {
        let value = &vec[rand::thread_rng().gen_range(0, vec.len())];
        let literal = get_var_from_ident(memory, event, value)?;

        return Ok(MessageType::Msg(Message::new(&Expr::LitExpr(literal), elem_type)));
    }

    Err(Error::new(ErrorKind::Other, "Builtin One_of bad argument"))
}

fn parse_quickbutton(val: String, buttton_type: String,  accepts: &mut Vec<String>) -> Button {
    accepts.push(val.clone());

    Button {
        title: val.clone(),
        buttton_type,
        accepts: vec![val.clone()],
        key: val.clone(),
        value: val.clone(),
        payload: val,
    }
}

fn parse_question(vec: &[Expr], memory: &Memory, event: &Option<Event>) -> Result<Question> {
    let expr_title = search_for_key_in_vec("title", vec)?; // Option
    let button_type = search_for_key_in_vec("button_type", vec)?; // Option
    let expr_buttons = expr_to_vec(search_for_key_in_vec("buttons", vec)?)?; // Option
    
    let mut buttons: Vec<Button> = vec![];
    let mut accepts: Vec<String> = vec![];

    for button in expr_buttons.iter() {
        if let Expr::Action{ builtin: Ident(name), args } = button {
            let vec = get_vec_from_box(args)?;

            if name == "Button" {
                for elem in vec.iter() {
                    buttons.push(parse_quickbutton(
                        get_var_from_ident(memory, event, elem)?.to_string(),
                        get_var_from_ident(memory, event, button_type)?.to_string(),
                        &mut accepts)
                    );
                }
            }
            // else { WARNING bad element }
        }
        // else { WARNING bad element }
    }

    Ok(Question {
        title: get_var_from_ident(memory, event, expr_title)?.to_string(),
        accepts,
        buttons,
    })
}

pub fn question(args: &Expr, name: String, memory: &Memory, event: &Option<Event>) -> Result<MessageType> {
    if let Expr::VecExpr(vec) = args {
        let question = parse_question(&vec, memory, event)?;

        return Ok(MessageType::Msg(
            Message {
                content_type: name.to_lowercase(),
                content: Content::Questions(question)
            }
        ))
    }

    Err(Error::new(ErrorKind::Other, "Builtin question bad argument"))
}

// ########################################################################

fn search_for_key_in_vec<'a>(key: &str, vec: &'a [Expr]) -> Result<&'a Expr> {
    for elem in vec.iter() {
        if let Expr::Assign(Ident(name), var) = elem {
            if name == key {
                return Ok(var);
            } 
        }
    }

    Err(Error::new(ErrorKind::Other, " search_for_key_in_vec"))
}

fn create_submap<'a>(keys: &[&str], vec: &'a [Expr], memory: &Memory, event: &Option<Event>) -> Result<Map<String, Value> > {
    let mut map = Map::new();

    for elem in vec.iter() {
        if let Expr::Assign(Ident(name), var) = elem {
            if let None = keys.iter().find(|&&x| x == name) {
                let value = get_var_from_ident(memory, event, var)?.to_string();
                map.insert(name.clone(), Value::String(value));
            }
        }
    }

    Ok(map)
}

// TODO: RM when var handling are separate from ast_iterpreter
fn expr_to_vec(expr: &Expr) -> Result<&Vec<Expr> > {
    match expr {
        Expr::VecExpr(vec)  => Ok(vec),
        _                   => Err(Error::new(ErrorKind::Other, " expr_to_vec"))
    }
}

fn value_or_default(key: &str, vec: &[Expr], default: Option<String>, memory: &Memory, event: &Option<Event>) -> Result<String> {
    match (search_for_key_in_vec(key, vec), default) {
        (Ok(arg), ..)           => Ok(get_var_from_ident(memory, event, arg)?.to_string()),
        (Err(..), Some(string)) => Ok(string),
        (Err(..), None)         => Err(Error::new(ErrorKind::Other, format!("Error: no key {} found", key)))
    }
}

//see if it can be a generic macro
fn get_vec_from_box(expr: &Expr) -> Result<&Vec<Expr> > {
    if let Expr::VecExpr(vec) = expr {
        Ok(vec)
    } else {
        Err(Error::new(ErrorKind::Other, " get_vec_from_box"))
    }
}

// meto ###############################################

fn parse_meteo(vec: &[Expr], memory: &Memory, event: &Option<Event>) -> Result<String> {
    let hostname = value_or_default("hostname", vec, Some("localhost".to_owned()), memory, event )?;
    let port = value_or_default("port", vec, Some(PORT.to_owned()), memory, event )?;
    let city = value_or_default("city", vec, Some("paris".to_owned()), memory, event )?;
    let lang = value_or_default("lang", vec, Some("en".to_owned()), memory, event )?;

    Ok(format!("http://{}:{}/meteo?city={}&lang={}", hostname, port, city, lang))
}

pub fn meteo(args: &Expr, memory: &Memory, event: &Option<Event>) -> Result<MessageType> {
    if let Expr::VecExpr(vec) = args {
        let meteo_arg = parse_meteo(&vec, memory, event)?;

        println!("http call {:?}", meteo_arg);
        match reqwest::get(&meteo_arg) {
            Ok(ref mut arg) => {
                match arg.text() {
                    Ok(text) => {
                        println!("reqwest get ok : ");
                        return Ok(MessageType::Msg(Message::new( &Expr::LitExpr(Literal::StringLiteral(text)) , "text".to_owned())))
                    },
                    Err(e)  => {
                        println!("error in parsing reqwest result: {:?}", e);
                        return Err(Error::new(ErrorKind::Other, "Error in parsing reqwest result"))
                    }
                }
            },
            Err(e) => {
                println!("error in reqwest get {:?}", e);
                return Err(Error::new(ErrorKind::Other, "Error in reqwest get"))
            }
        };
    }

    println!("meto is not correctly formatted");
    Err(Error::new(ErrorKind::Other, "Builtin meteo bad argument"))
}

// wttj ###############################################

fn parse_wttj(vec: &[Expr], memory: &Memory, event: &Option<Event>) -> Result<(String, HashMap<String, Value>) > {
    let hostname = value_or_default("hostname", vec, Some("localhost".to_owned()), memory, event)?;
    let port = value_or_default("port", vec, Some(PORT.to_owned()), memory, event)?;
    let action = value_or_default("action", vec, None, memory, event)?;

    let mut map: HashMap<String, Value> = HashMap::new();

    match action.as_ref() {
        "getCandidates"   => {
            map.insert("action".to_owned(), Value::String("getCandidates".to_owned()) );
        },
        "moveCandidate"   => {
            let name = value_or_default("name", vec, None, memory, event)?;
            let stage = value_or_default("stage", vec, None, memory, event)?;

            map.insert("action".to_owned(), Value::String("moveCandidate".to_owned()) );
            map.insert("name".to_owned(), Value::String(name));
            map.insert("stage".to_owned(), Value::String(stage));
        },
        "createCandidate" => {
            let mut candidate_info = Map::new();

            candidate_info.insert("firstname".to_string(), Value::String(value_or_default("firstname", vec, None, memory, event )?) );
            candidate_info.insert("lastname".to_string(), Value::String(value_or_default("lastname", vec, None, memory, event )?) );
            candidate_info.insert("email".to_string(), Value::String(value_or_default("email", vec, None, memory, event )?) );

            map.insert("action".to_owned(), Value::String("createCandidate".to_owned()) );
            map.insert("candidate".to_owned(), Value::Object(candidate_info));
        },
        action            => return Err(Error::new(ErrorKind::Other, format!("no action exist with name {}", action)))
    }

    Ok( (format!("http://{}:{}/wttj", hostname, port), map) )
}

pub fn wttj(args: &Expr, memory: &Memory, event: &Option<Event>) -> Result<MessageType> {
    if let Expr::VecExpr(vec) = args {
        let (http_arg, map) = parse_wttj(&vec, memory, event)?;

        println!("http call {:?}", http_arg);
        println!("map {:?}", serde_json::to_string(&map).unwrap());
        match reqwest::Client::new().post(&http_arg).json(&map).send() {
            Ok(ref mut arg) => {
                match arg.text() {
                    Ok(text) => {
                        println!("reqwest get ok : ");
                        return Ok(MessageType::Msg(Message::new( &Expr::LitExpr(Literal::StringLiteral(text)) , "text".to_owned())))
                    },
                    Err(e)  => {
                        println!("error in parsing reqwest result: {:?}", e);
                        return Err(Error::new(ErrorKind::Other, "Error in parsing reqwest result"))
                    }
                }
            },
            Err(e) => {
                println!("error in reqwest get {:?}", e);
                return Err(Error::new(ErrorKind::Other, "Error in reqwest get"))
            }
        };
    }

    println!("wttj is not correctly formatted");
    Err(Error::new(ErrorKind::Other, "Builtin WTTJ bad argument"))
}

// Hubspot ###############################################
fn parse_hub_spot(vec: &[Expr], memory: &Memory, event: &Option<Event>) -> Result<(String, HashMap<String, Value>) > {
    let hostname = value_or_default("hostname", vec, Some("localhost".to_owned()), memory, event)?;
    let port = value_or_default("port", vec, Some(PORT.to_owned()), memory, event)?;
    let action = value_or_default("action", vec, None, memory, event)?;
    let sub_map = create_submap(&["hostname", "port", "action"], vec, memory, event)?;

    let mut map: HashMap<String, Value> = HashMap::new();

    map.insert("action".to_owned(), Value::String(action));
    map.insert("candidate".to_owned(), Value::Object(sub_map));

    Ok((format!("http://{}:{}/hubspot", hostname, port), map))
}

pub fn hub_spot(args: &Expr, memory: &Memory, event: &Option<Event>) -> Result<MessageType> {
    if let Expr::VecExpr(vec) = args {
        let (http_arg, map) = parse_hub_spot(&vec, memory, event)?;

        println!("http call {:?}", http_arg);
        println!("map {:?}", serde_json::to_string(&map).unwrap());
        match reqwest::Client::new().post(&http_arg).json(&map).send() {
            Ok(ref mut arg) => {
                match arg.text() {
                    Ok(text) => {
                        println!("reqwest get ok : ");
                        return Ok(MessageType::Msg(Message::new( &Expr::LitExpr(Literal::StringLiteral(text)) , "text".to_owned())))
                    },
                    Err(e)  => {
                        println!("error in parsing reqwest result: {:?}", e);
                        return Err(Error::new(ErrorKind::Other, "Error in parsing reqwest result"))
                    }
                }
            },
            Err(e) => {
                println!("error in reqwest get {:?}", e);
                return Err(Error::new(ErrorKind::Other, "Error in reqwest get"))
            }
        };
    }

    println!("hub_spot is not correctly formatted");
    Err(Error::new(ErrorKind::Other, "Builtin hub_spot bad argument"))
}

// GetGSheet ###############################################

fn parse_get_gsheet(vec: &[Expr], memory: &Memory, event: &Option<Event>) -> Result<String> {
    let hostname = value_or_default("hostname", vec, Some("localhost".to_owned()), memory, event )?;
    let port = value_or_default("port", vec, Some(PORT.to_owned()), memory, event )?;
    
    let sheet_id = value_or_default("sheet_id", vec, None, memory, event )?;

    Ok(format!("http://{}:{}/gsheets?sheet_id={}", hostname, port, sheet_id))
}

pub fn get_gsheet(args: &Expr, memory: &Memory, event: &Option<Event>) -> Result<MessageType> {
    if let Expr::VecExpr(vec) = args {
        let meteo_arg = parse_get_gsheet(&vec, memory, event)?;

        println!("http call {:?}", meteo_arg);
        match reqwest::get(&meteo_arg) {
            Ok(ref mut arg) => {
                match arg.text() {
                    Ok(text) => {
                        println!("reqwest get ok : ");
                        return Ok(MessageType::Msg(Message::new( &Expr::LitExpr(Literal::StringLiteral(text)) , "text".to_owned())))
                    },
                    Err(e)  => {
                        println!("error in parsing reqwest result: {:?}", e);
                        return Err(Error::new(ErrorKind::Other, "Error in parsing reqwest result"))
                    }
                }
            },
            Err(e) => {
                println!("error in reqwest get {:?}", e);
                return Err(Error::new(ErrorKind::Other, "Error in reqwest get"))
            }
        };
    }

    println!("get_gsheet is not correctly formatted");
    Err(Error::new(ErrorKind::Other, "Builtin get_gsheet bad argument"))
}

// AppendGsheet ###############################################

fn parse_append_gsheet(vec: &[Expr], memory: &Memory, event: &Option<Event>) -> Result<(String, HashMap<String, Value>) > {
    let hostname = value_or_default("hostname", vec, Some("localhost".to_owned()), memory, event)?;
    let port = value_or_default("port", vec, Some(PORT.to_owned()), memory, event)?;
    
    let sheet_id = value_or_default("sheet_id", vec, None, memory, event)?;
    let sub_map = create_submap(&["hostname", "port", "sheet_id"], vec, memory, event)?;

    let mut map: HashMap<String, Value> = HashMap::new();

    map.insert("sheet_id".to_owned(), Value::String(sheet_id));
    map.insert("params".to_owned(), Value::Object(sub_map));

    Ok((format!("http://{}:{}/gsheets", hostname, port), map))
}

pub fn append_gsheet(args: &Expr, memory: &Memory, event: &Option<Event>) -> Result<MessageType> {
    if let Expr::VecExpr(vec) = args {
        let (http_arg, map) = parse_append_gsheet(&vec, memory, event)?;

        println!("http call {:?}", http_arg);
        println!("map {:?}", serde_json::to_string(&map).unwrap());
        match reqwest::Client::new().post(&http_arg).json(&map).send() {
            Ok(ref mut arg) => {
                match arg.text() {
                    Ok(text) => {
                        println!("reqwest get ok : ");
                        return Ok(MessageType::Msg(Message::new( &Expr::LitExpr(Literal::StringLiteral(text)) , "text".to_owned())))
                    },
                    Err(e)  => {
                        println!("error in parsing reqwest result: {:?}", e);
                        return Err(Error::new(ErrorKind::Other, "Error in parsing reqwest result"))
                    }
                }
            },
            Err(e) => {
                println!("error in reqwest get {:?}", e);
                return Err(Error::new(ErrorKind::Other, "Error in reqwest get"))
            }
        };
    }

    println!("append_gsheet is not correctly formatted");
    Err(Error::new(ErrorKind::Other, "Builtin append_gsheet bad argument"))
}
// aws #############################################################################

fn parse_aws(vec: &[Expr], memory: &Memory, event: &Option<Event>) -> Result<(String, HashMap<String, Value>) > {
    let hostname = value_or_default("hostname", vec, Some("localhost".to_owned()), memory, event)?;
    let port = value_or_default("port", vec, Some(PORT.to_owned()), memory, event)?;
    let action = value_or_default("action", vec, None, memory, event)?;
    let sub_map = create_submap(&["hostname", "port", "action"], vec, memory, event)?;

    let mut map: HashMap<String, Value> = HashMap::new();

    map.insert("action".to_owned(), Value::String(action));
    map.insert("params".to_owned(), Value::Object(sub_map));

    Ok((format!("http://{}:{}/aws", hostname, port), map))
}

pub fn aws(args: &Expr, memory: &Memory, event: &Option<Event>) -> Result<MessageType> {
    if let Expr::VecExpr(vec) = args {
        let (http_arg, map) = parse_aws(&vec, memory, event)?;

        println!("http call {:?}", http_arg);
        println!("map {:?}", serde_json::to_string(&map).unwrap());
        match reqwest::Client::new().post(&http_arg).json(&map).send() {
            Ok(ref mut arg) => {
                match arg.text() {
                    Ok(text) => {
                        println!("reqwest get ok : ");
                        return Ok(MessageType::Msg(Message::new( &Expr::LitExpr(Literal::StringLiteral(text)) , "text".to_owned())))
                    },
                    Err(e)  => {
                        println!("error in parsing reqwest result: {:?}", e);
                        return Err(Error::new(ErrorKind::Other, "Error in parsing reqwest result"))
                    }
                }
            },
            Err(e) => {
                println!("error in reqwest get {:?}", e);
                return Err(Error::new(ErrorKind::Other, "Error in reqwest get"))
            }
        };
    }

    println!("aws is not correctly formatted");
    Err(Error::new(ErrorKind::Other, "Builtin aws bad argument"))
}
