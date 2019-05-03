use rand::Rng;
use std::io::{Error, ErrorKind, Result};

use crate::parser::ast::{Expr, Literal, Ident};
use crate::interpreter::message::*;

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

pub fn one_of(args: &Expr, name: String) -> Result<MessageType> {
    if let Expr::VecExpr(vec) = args {
        return Ok(MessageType::Msg(Message::new(&vec[rand::thread_rng().gen_range(0, vec.len())], name)));
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

// TODO: RM when var handling are separate from ast_iterpreter
fn expr_to_string(expr: &Expr) -> Result<String> {
    match expr {
        Expr::LitExpr(literal)          => Ok(literal.to_string()),
        Expr::IdentExpr(Ident(ident))   => Ok(ident.to_owned()),
        _                               => Err(Error::new(ErrorKind::Other, " expr_to_string"))
    }
}

// return Result<Expr, error>
// TODO: RM when var handling are separate from ast_iterpreter
fn expr_to_vec(expr: &Expr) -> Result<&Vec<Expr> > {
    match expr {
        Expr::VecExpr(vec)  => Ok(vec),
        _                   => Err(Error::new(ErrorKind::Other, " expr_to_vec"))
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


fn parse_question(vec: &[Expr]) -> Result<Question> {
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
                    buttons.push(parse_quickbutton(expr_to_string(elem)?, expr_to_string(button_type)?, &mut accepts));
                }
            }
            // else { WARNING bad element }
        }
        // else { WARNING bad element }
    }

    Ok(Question {
        title: expr_to_string(expr_title)?,
        accepts,
        buttons,
    })
}

pub fn question(args: &Expr, name: String) -> Result<MessageType>{
    if let Expr::VecExpr(vec) = args {
        let question = parse_question(&vec)?;

        return Ok(MessageType::Msg(
            Message {
                content_type: name.to_lowercase(),
                content: Content::Questions(question)
            }
        ))
    }

    Err(Error::new(ErrorKind::Other, "Builtin question bad argument"))
}
