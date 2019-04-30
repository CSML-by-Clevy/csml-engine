use crate::parser::ast::{Expr, Literal};
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

        println!(">>>>>>>>>>>> {:?}", buttons);
        
        return MessageType::Msg( Message {
            content_type: "quick_button".to_string(), content: Content::Buttons(buttons)
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
