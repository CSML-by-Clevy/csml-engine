use crate::parser::ast::{Expr, Literal, Ident};
use crate::interpreter::message::*;
use rand::Rng;

//TODO: ERROR Handling in builtin
// return Result<Expr, error>
pub fn typing(args: &Expr) -> &Expr {
    if let Expr::VecExpr(vec) = args {
        if vec.len() == 1 {
            if let Expr::LitExpr(Literal::IntLiteral(_)) = &vec[0] {
                return &vec[0];
            }
        }
        return &vec[0];
    }
    args
}

// return Result<Expr, error>
pub fn wait(args: &Expr) -> &Expr {
    if let Expr::VecExpr(vec) = args {
        if vec.len() == 1 {
            if let Expr::LitExpr(Literal::IntLiteral(_)) = &vec[0] {
                return &vec[0];
            }
        }
        return &vec[0];
    }
    args
}

// return Result<Expr, error>
pub fn text(args: &Expr) -> &Expr {
    if let Expr::VecExpr(vec) = args {
        if vec.len() == 1 {
            if let Expr::LitExpr(_) = &vec[0] {
                return &vec[0];
            }
        }
        return &vec[0];
    }
    args
}

// return Result<Expr, error>
pub fn img(args: &Expr) -> &Expr {
    if let Expr::VecExpr(vec) = args {
        if vec.len() == 1 {
            if let Expr::LitExpr(_) = &vec[0] {
                return &vec[0];
            }
        }
        return &vec[0];
    }
    args
}

// return Result<Expr, error>
//TODO: Find better solution
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

// return Result<Expr, error>
fn search_for_key_in_vec<'a>(key: &str, vec: &'a Vec<Expr>) -> &'a Expr {
    for elem in vec.iter() {
        if let Expr::Assign(Ident(name), var) = elem {
            if name == key {
                return var;
            } 
        }
    }
    panic!("error in search_for_key_in_vec");
}

// return Result<Expr, error>
// TODO: RM when var handling are separate from ast_iterpreter
fn expr_to_string(expr: &Expr) -> String {
    match expr {
        Expr::LitExpr(literal)          => literal.to_string(),
        Expr::IdentExpr(Ident(ident))   => ident.to_owned(),
        _                               => panic!("error in expr_to_string")
    }
}

// return Result<Expr, error>
// TODO: RM when var handling are separate from ast_iterpreter
fn expr_to_vec<'a>(expr: &'a Expr) -> &'a Vec<Expr> {
    match expr {
        Expr::VecExpr(vec)  => vec,
        _                   => panic!("expr_to_vec")
    }
}

// return Result<Expr, error>
fn parse_question(vec: &Vec<Expr>) -> Question {
    let title = search_for_key_in_vec("title", vec); // Option
    let button_type = search_for_key_in_vec("button_type", vec); // Option
    let expr_buttons = expr_to_vec(search_for_key_in_vec("buttons", vec)); // Option
    let mut buttons: Vec<Button> = vec![];
    let mut accepts: Vec<String> = vec![];

    for button in expr_buttons.iter() {
        if let Expr::Assign(Ident(name), expr) = button {
            if name == "Button" {
                buttons.push(parse_quickbutton(expr_to_string(expr), expr_to_string(button_type), &mut accepts));
            }
            // else { WARNING bad element }
        }
    }

    Question {
        title: expr_to_string(title),
        accepts,
        buttons,
    }
}

// return Result<Expr, error>
pub fn question(args: &Expr) -> MessageType {
    if let Expr::VecExpr(vec) = args {
        let question = parse_question(&vec);

        return MessageType::Msg( Message {
            content_type: "question".to_string(), content: Content::Questions(question)
        })
    }
    MessageType::Msg( Message {content_type: "text".to_owned(), content: Content::Text("error in button construction".to_owned()) })
}

// return Result<Expr, error>
pub fn url(args: &Expr) -> &Expr {
    if let Expr::VecExpr(vec) = args {
        if vec.len() == 1 {
            if let Expr::LitExpr(_) = &vec[0] {
                return &vec[0];
            }
        }
        return &vec[0];
    }
    args
}

// return Result<Expr, error>
pub fn one_of(args: &Expr) -> &Expr {
    if let Expr::VecExpr(vec) = args {
        return &vec[rand::thread_rng().gen_range(0, vec.len())];
    }
    args
}

// return Result<Expr, error>
pub fn remember(name: String, value: String) -> MessageType {
    return MessageType::Assign{name, value};
}
