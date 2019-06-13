use std::io::{Error, ErrorKind, Result};
use crate::parser::{ast::*, tokens::*};
use crate::interpreter:: {
    json_to_rust::*,
};

pub fn gen_literal_form_event(event: &Option<Event>) -> Result<Literal> {
    match event {
        Some(event)        => {
            match event.payload {
                PayLoad{content_type: ref t, content: ref c} if t == "text" => Ok(Literal::StringLiteral(c.text.to_string())),
                _                                                           => Err(Error::new(ErrorKind::Other, "event type is unown")),
            }
        },
        None               => Err(Error::new(ErrorKind::Other, "no event is received in gen_literal_form_event"))
    }
}

pub fn search_str(name: &str, expr: &Expr) -> bool {
    match expr {
        Expr::IdentExpr(ident) if ident == name  => true,
        _                                        => false
    }
}

pub fn get_var(memory: &Memory, event: &Option<Event>, name: &str) -> Result<Literal> {
    match name {
        var if var == EVENT      => gen_literal_form_event(event),
        _                        => search_var_memory(memory, name),
    }
}

pub fn get_string_from_complexstring(memory: &Memory, event: &Option<Event>, exprs: &[Expr]) -> Literal {
    let mut new_string = String::new();

    for elem in exprs.iter() {
        match get_var_from_ident(memory, event, elem) {
            Ok(var) => new_string.push_str(&var.to_string()),
            Err(_)  => new_string.push_str(" NULL ")
        }
    }

    Literal::StringLiteral(new_string)
}

pub fn get_var_from_ident(memory: &Memory, event: &Option<Event>, expr: &Expr) -> Result<Literal> {
    match expr {
        Expr::LitExpr{lit}           => Ok(lit.clone()),
        Expr::IdentExpr(ident)       => get_var(memory, event, ident),
        Expr::BuilderExpr(..)        => gen_literal_form_builder(memory, event, expr),
        Expr::ComplexLiteral(..)     => gen_literal_form_builder(memory, event, expr),
        _                            => Err(Error::new(ErrorKind::Other, "unown variable in Ident err n#1"))
    }
}

pub fn gen_literal_form_exp(memory: &Memory, event: &Option<Event>, expr: &Expr) -> Result<Literal> {
    match expr {
        Expr::LitExpr{lit}      => Ok(lit.clone()),
        Expr::IdentExpr(ident)  => get_var(memory, event, ident),
        _                       => Err(Error::new(ErrorKind::Other, "Expression must be a literal or an identifier"))
    }
}

pub fn gen_literal_form_builder(memory: &Memory, event: &Option<Event>, expr: &Expr) -> Result<Literal> {
    match expr {
        Expr::BuilderExpr(elem, exp) if search_str(PAST, elem)     => get_memory_action(memory, elem, exp),
        Expr::BuilderExpr(elem, exp) if search_str(MEMORY, elem)   => get_memory_action(memory, elem, exp),
        Expr::BuilderExpr(elem, exp) if search_str(METADATA, elem) => get_memory_action(memory, elem, exp),
        Expr::ComplexLiteral(vec)                                                   => Ok(get_string_from_complexstring(memory, event, vec)),
        Expr::IdentExpr(ident)                                                      => get_var(memory, event,ident),
        _                                                                           => Err(Error::new(ErrorKind::Other, "Error in Exprecion builder"))
    }
}

pub fn memorytype_to_literal(memtype: Option<&MemoryType>) -> Result<Literal> {
    if let Some(elem) = memtype {
        Ok(Literal::StringLiteral(elem.value.to_string()))
    } else {
        Err(Error::new(ErrorKind::Other, "Error in memory action"))
    }
}

// MEMORY ------------------------------------------------------------------

pub fn search_var_memory(memory: &Memory, name: &str) -> Result<Literal> {
    match name {
        var if memory.metadata.contains_key(var) => memorytype_to_literal(memory.metadata.get(var)),
        var if memory.current.contains_key(var)  => memorytype_to_literal(memory.current.get(var)),
        var if memory.past.contains_key(var)     => memorytype_to_literal(memory.past.get(var)),
        _                                        => Err(Error::new(ErrorKind::Other, "unown variable in search_var_memory")),
    }
}

pub fn memory_get<'a>(memory: &'a Memory, name: &Expr, expr: &Expr) -> Option<&'a MemoryType> {
    match (name, expr) {
        (Expr::IdentExpr(ident), Expr::LitExpr{lit: Literal::StringLiteral(lit)}) if ident == PAST    => memory.past.get(lit),
        (Expr::IdentExpr(ident), Expr::LitExpr{lit: Literal::StringLiteral(lit)}) if ident == MEMORY  => memory.current.get(lit),
        (_, Expr::LitExpr{lit: Literal::StringLiteral(lit)})                                          => memory.metadata.get(lit),
        _                                                                                             => None,
    }
}

//TODO: RM UNWRAP
pub fn memory_first<'a>(memory: &'a Memory, name: &Expr, expr: &Expr) -> Option<&'a MemoryType> {
    match (name, expr) {
        (Expr::IdentExpr(ident), Expr::LitExpr{lit: Literal::StringLiteral(lit)}) if ident == PAST    => memory.past.get_vec(lit).unwrap().last(),
        (Expr::IdentExpr(ident), Expr::LitExpr{lit: Literal::StringLiteral(lit)}) if ident == MEMORY  => memory.current.get_vec(lit).unwrap().last(),
        (_, Expr::LitExpr{lit: Literal::StringLiteral(lit)})                                          => memory.metadata.get_vec(lit).unwrap().last(),
        _                                                                                             => None,
    }
}

//NOTE:Only work with Strings for now 
pub fn get_memory_action(memory: &Memory, name: &Expr, expr: &Expr) -> Result<Literal> {
    match expr {
        Expr::FunctionExpr(ReservedFunction::Normal(ident), exp) if ident == GET_VALUE     => {
            memorytype_to_literal(memory_get(memory, name, exp))
        },
        Expr::FunctionExpr(ReservedFunction::Normal(ident), exp) if ident == FIRST         => {
            memorytype_to_literal(memory_first(memory, name, exp))
        },
        _                                                               => Err(Error::new(ErrorKind::Other, "Error in memory action")),
    }
}
