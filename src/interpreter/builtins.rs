use crate::parser::ast::*;
use crate::interpreter::message::*;
use rand::Rng;

fn exprvec_to_vec(vec: &[Expr]) -> Vec<String> {
    vec.iter().filter_map(|elem|
        match elem {
           Expr::LitExpr(Literal::StringLiteral(string))    => Some(string.clone()),
           Expr::LitExpr(Literal::IntLiteral(int))          => Some(int.to_string()),
           _                                                => None
        }
    ).collect::<Vec<String>>()
}

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
    Button{
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
