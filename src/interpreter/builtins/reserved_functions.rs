use rand::Rng;
use std::io::{Error, ErrorKind, Result};
use std::borrow::Borrow;

use crate::parser::{ast::{Expr, Literal, ReservedFunction}, tokens::*};
use crate::interpreter:: {
    message::*,
    json_to_rust::*,
    variable_handler::*,
    builtins::*,
};

pub fn remember(name: String, value: String) -> MessageType {
    MessageType::Assign{name, value}
}

pub fn typing(args: &Expr, name: String) -> Result<MessageType> {
    if let Expr::VecExpr(vec) = args {
        if vec.len() == 1 {
            if let Expr::LitExpr{lit: Literal::IntLiteral(_)} = &vec[0] {
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
            if let Expr::LitExpr{lit: Literal::IntLiteral(_)} = &vec[0] {
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
            if let Expr::LitExpr{..} = &vec[0] {
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
            if let Expr::LitExpr{..} = &vec[0] {
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
            if let Expr::LitExpr{..} = &vec[0] {
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

        return Ok(MessageType::Msg(Message::new(&Expr::LitExpr{lit: literal}, elem_type)));
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

fn match_buttons(buttons: &mut Vec<Button>, button_type: &Expr, accepts: &mut Vec<String>, name: &str, expr: &Expr, memory: &Memory, event: &Option<Event>) -> Result<bool> {
    match (name, expr.borrow()) {
        (BUTTON, Expr::VecExpr(expr_vec))   => {
            for elem in expr_vec.iter() {
                buttons.push(parse_quickbutton(
                    get_var_from_ident(memory, event, elem)?.to_string(),
                    get_var_from_ident(memory, event, button_type)?.to_string(),
                    accepts)
                );
            }
        }
        _                                   => return Err(Error::new(ErrorKind::Other, "bad Button Type"))
    }

    Ok(true)
}

fn parse_question(vec: &[Expr], memory: &Memory, event: &Option<Event>) -> Result<Question> {
    let expr_title = search_for_key_in_vec("title", vec)?; // Option
    let button_type = search_for_key_in_vec("button_type", vec)?; // Option
    let expr_buttons = expr_to_vec(search_for_key_in_vec("buttons", vec)?)?; // Option

    let mut buttons: Vec<Button> = vec![];
    let mut accepts: Vec<String> = vec![];

    for button in expr_buttons.iter() {
        if let Expr::FunctionExpr(ReservedFunction::Normal(name), expr) = button {
            match_buttons(&mut buttons, &button_type, &mut accepts, &name, &expr, memory, event)?;
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
