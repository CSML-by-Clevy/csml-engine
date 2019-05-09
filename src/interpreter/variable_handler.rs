use std::io::{Error, ErrorKind, Result};
use crate::parser::ast::*;
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

pub fn search_var_memory(memory: &Memory, name: &Ident) -> Result<Literal> {
    match name {
        Ident(var) if memory.metadata.contains_key(var) => memorytype_to_literal(memory.metadata.get(var)),
        Ident(var) if memory.current.contains_key(var)  => memorytype_to_literal(memory.current.get(var)),
        Ident(var) if memory.past.contains_key(var)     => memorytype_to_literal(memory.past.get(var)),
        _                                                    => Err(Error::new(ErrorKind::Other, "unown variable in search_var_memory")),
    }
}

pub fn get_var(memory: &Memory, event: &Option<Event>, name: &Ident) -> Result<Literal> {
    match name {
        Ident(var) if var == "event"    => gen_literal_form_event(event),
        Ident(_)                        => search_var_memory(memory, name),
    }
}

pub fn get_string_from_complexstring(memory: &Memory, event: &Option<Event>, exprs: &[Expr]) -> Literal {
    let mut new_string = String::new();

    for elem in exprs.iter() {
        match get_var_from_ident(memory, event, elem) {
            Ok(var) => new_string.push_str(&var.to_string()),
            Err(_)  => new_string.push_str(&format!(" NULL "))
        }
    }

    Literal::StringLiteral(new_string)
}

pub fn get_var_from_ident(memory: &Memory, event: &Option<Event>, expr: &Expr) -> Result<Literal> {
    match expr {
        Expr::LitExpr(lit)           => Ok(lit.clone()),
        Expr::IdentExpr(ident)       => get_var(memory, event, ident),
        Expr::BuilderExpr(..)        => gen_literal_form_builder(memory, event, expr),
        Expr::ComplexLiteral(..)     => gen_literal_form_builder(memory, event, expr),
        _                            => Err(Error::new(ErrorKind::Other, "unown variable in Ident err n#1"))
    }
}

pub fn gen_literal_form_builder(memory: &Memory, event: &Option<Event>, expr: &Expr) -> Result<Literal> {
    match expr {
        Expr::BuilderExpr(elem, exp) if search_str("past", elem)     => get_memory_action(memory, elem, exp),
        Expr::BuilderExpr(elem, exp) if search_str("memory", elem)   => get_memory_action(memory, elem, exp),
        Expr::BuilderExpr(elem, exp) if search_str("metadata", elem) => get_memory_action(memory, elem, exp),
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
pub fn search_str(name: &str, expr: &Expr) -> bool {
    match expr {
        Expr::IdentExpr(Ident(ident)) if ident == name  => true,
        _                                               => false
    }
}

pub fn memory_get<'a>(memory: &'a Memory, name: &Expr, expr: &Expr) -> Option<&'a MemoryType> {
    match (name, expr) {
        (Expr::IdentExpr(Ident(ident)), Expr::LitExpr(Literal::StringLiteral(lit))) if ident == "past"    => memory.past.get(lit),
        (Expr::IdentExpr(Ident(ident)), Expr::LitExpr(Literal::StringLiteral(lit))) if ident == "memory"  => memory.current.get(lit),
        (_, Expr::LitExpr(Literal::StringLiteral(lit)))                                                   => memory.metadata.get(lit),
        _                                                                                                 => None,
    }
}

//TODO: RM UNWRAP
pub fn memory_first<'a>(memory: &'a Memory, name: &Expr, expr: &Expr) -> Option<&'a MemoryType> {
    match (name, expr) {
        (Expr::IdentExpr(Ident(ident)), Expr::LitExpr(Literal::StringLiteral(lit))) if ident == "past"    => memory.past.get_vec(lit).unwrap().last(),
        (Expr::IdentExpr(Ident(ident)), Expr::LitExpr(Literal::StringLiteral(lit))) if ident == "memory"  => memory.current.get_vec(lit).unwrap().last(),
        (_, Expr::LitExpr(Literal::StringLiteral(lit)))                                                   => memory.metadata.get_vec(lit).unwrap().last(),
        _                                                                                                 => None,
    }
}

//NOTE:Only work with Strings for now 
pub fn get_memory_action(memory: &Memory, name: &Expr, expr: &Expr) -> Result<Literal> {
    match expr {
        Expr::FunctionExpr(Ident(ident), exp) if ident == "get"         => {
            memorytype_to_literal(memory_get(memory, name, exp))
        },
        Expr::FunctionExpr(Ident(ident), exp) if ident == "first"       => {
            memorytype_to_literal(memory_first(memory, name, exp))
        },
        _                                                               => Err(Error::new(ErrorKind::Other, "Error in memory action")),
    }
}
