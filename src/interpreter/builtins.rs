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

fn tmp_formatter_string(elem: &Expr) -> String {
    if let Expr::LitExpr(Literal::StringLiteral(string)) = elem {
        string.to_string()
    } else {
        "Error in button elem".to_owned()
    }
}

fn tmp_formatter_vec(args: &Expr) -> Vec<String> {
    let mut vec = vec![];
    if let Expr::VecExpr(elem) = args {
        for val in elem.iter() {
            vec.push( tmp_formatter_string(val) );
        }
        vec
    } else {
        vec.push("Error in button vec elem".to_owned());
        vec
    }
}

//TODO: Find better solution
fn parse_button(button: &[Expr]) -> Button {
    Button {
        title: tmp_formatter_string(button.get(0).unwrap()),
        accepts: tmp_formatter_vec(button.get(1).unwrap()), //Vec<String>
        key: tmp_formatter_string(button.get(2).unwrap()),
        value: tmp_formatter_string(button.get(3).unwrap()),
        payload: tmp_formatter_string(button.get(4).unwrap()),
    }
}

// return Result<Expr, error>
pub fn button(args: &Expr) -> MessageType {
    if let Expr::VecExpr(vec) = args {
        let mut buttons = vec![];
        for elem in vec.iter() {
            if let Expr::VecExpr(button) = elem {
                buttons.push( parse_button(button) )
            }
        }

        return MessageType::Msg( Message {
            content_type: "button".to_string(), content: Content::Buttons(buttons)
        })
    }

    MessageType::Msg( Message {content_type: "text".to_owned(), content: Content::Text("error in button construction".to_owned()) })
}

//TODO: Find better solution
fn parse_quickbutton(val: String) -> Button {
    Button {
        title: val.clone(),
        accepts: vec![val.clone()],
        key: val.clone(),
        value: val.clone(),
        payload: val,
    }
}

// return Result<Expr, error>
pub fn quick_button(args: &Expr) -> MessageType {
    if let Expr::VecExpr(vec) = args {
        let mut buttons: Vec<Button> = vec![];
        for elem in vec.iter() {
            if let Expr::LitExpr(literal) = elem {
                buttons.push( parse_quickbutton(literal.to_string()) );
            }
        }        
        return MessageType::Msg( Message {
            content_type: "quick_button".to_string(), content: Content::Buttons(buttons)
        })
    }

    MessageType::Msg( Message {content_type: "text".to_owned(), content: Content::Text("error in button construction".to_owned()) })
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct Button {
//     pub title: String,
//     pub accepts: Vec<String>,
//     pub key: String,
//     pub value: String,
//     pub payload: String,
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct Question {
//     pub title: String,
//     pub accepts: Vec<String>,
//     pub buttons: Vec<Button>
// }

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

// TODO: RM when var handling are separate from ast_iterpreter
fn expr_to_string(expr: &Expr) -> String {
    match expr {
        Expr::LitExpr(literal)          => literal.to_string(),
        Expr::IdentExpr(Ident(ident))   => ident.to_owned(),
        _                               => panic!("error in expr_to_string")
    }
}

fn expr_to_vec<'a>(expr: &'a Expr) -> &'a Vec<Expr> {
    match expr {
        Expr::VecExpr(vec)  => vec,
        _                   => panic!("expr_to_vec")
    }
}

fn parse_question(vec: &Vec<Expr>) -> Question {
    let title = search_for_key_in_vec("title", vec);
    let button_type = search_for_key_in_vec("button_type", vec);
    let expr_buttons = expr_to_vec(search_for_key_in_vec("buttons", vec));
    let mut buttons: Vec<Button> = vec![];
    let mut accepts: Vec<String> = vec![];

    for button in expr_buttons.iter() {
        if let Expr::Assign(Ident(name), expr) = button {
            if name == "QuickButton" {
                buttons.push(parse_quickbutton(expr_to_string(expr)));
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
