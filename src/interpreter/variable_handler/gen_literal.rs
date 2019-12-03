use crate::error_format::data::ErrorInfo;
use crate::interpreter::{
    data::Data,
    json_to_rust::Event,
    ast_interpreter::get_path,
    variable_handler::{
        get_string_from_complexstring,
        get_var,
        interval::interval_from_expr,
        memory::search_in_metadata, //get_memory_action
        object::get_value_in_object,
    },
};
use crate::parser::{
    ast::{BuilderType, Expr, Identifier, Interval},
    literal::Literal,
    // tokens::{_METADATA}, MEMORY, PAST,
};

pub fn search_str(name: &str, expr: &Expr) -> bool {
    match expr {
        Expr::IdentExpr(Identifier { ident, .. }) if ident == name => true,
        _ => false,
    }
}

pub fn gen_literal_form_expr(expr: &Expr, data: &mut Data) -> Result<Literal, ErrorInfo> {
    match expr {
        Expr::LitExpr(literal) => Ok(literal.clone()),
        Expr::IdentExpr(ident, ..) => get_var(ident.clone(), data),
        e => Err(ErrorInfo {
            message: "Expression must be a literal or an identifier".to_owned(),
            interval: interval_from_expr(e),
        }),
    }
}

pub fn gen_literal_form_builder(expr: &Expr, data: &mut Data) -> Result<Literal, ErrorInfo> {
    match expr {
        Expr::BuilderExpr(BuilderType::Metadata(..), path) => search_in_metadata(path, data),
        // Expr::BuilderExpr(elem, expr) if search_str(PAST, elem) => {
        //     get_memory_action(data.memory, elem, expr, data)
        // }
        // Expr::BuilderExpr(elem, expr) if search_str(MEMORY, elem) => {
        //     get_memory_action(data.memory, elem, expr, data)
        // }
        Expr::BuilderExpr(BuilderType::Normal(ident), path) => {
            let literal = get_var(ident.to_owned(), data)?;
            let path = get_path(&path, data)?;
            get_value_in_object(&literal, &path, &ident.interval)
        }
        Expr::ComplexLiteral(vec, ..) => Ok(get_string_from_complexstring(vec, data)),
        Expr::IdentExpr(ident, ..) => get_var(ident.clone(), data),
        e => Err(ErrorInfo {
            message: "Error in Expression builder".to_owned(),
            interval: interval_from_expr(e),
        }),
    }
}

pub fn gen_literal_form_event(
    event: &Option<Event>,
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    match event {
        Some(Event { payload }) => Ok(Literal::string(payload.to_owned(), interval)),
        None => Ok(Literal::null(interval)),
    }
}
