use crate::error_format::data::ErrorInfo;
use crate::interpreter::{ast_interpreter::evaluate_condition, data::Data, json_to_rust::*};
use crate::parser::{ast::*, tokens::*};

pub fn gen_literal_form_event(
    event: &Option<Event>,
    interval: Interval,
) -> Result<SmartLiteral, ErrorInfo> {
    match event {
        Some(event) => match event.payload {
            PayLoad { content_type: ref t, content: ref c, } 
                if t == "text" => Ok(SmartLiteral {
                    literal: Literal::string(c.text.to_string(), None),
                    interval,
                }
            ),
            PayLoad {content_type: ref t, content: ref c}
                if t == "float" => match c.text.to_string().parse::<f64>() {
                Ok(float) => Ok(SmartLiteral {
                    literal: Literal::float(float, None),
                    interval,
                }),
                Err(..) => Err(ErrorInfo {
                    message: format!("event value {} is not of type float", c.text),
                    interval,
                }),
            },
            PayLoad { content_type: ref t, content: ref c}
                if t == "int" => match c.text.to_string().parse::<i64>() {
                Ok(int) => Ok(SmartLiteral {
                    literal: Literal::int(int, None),
                    interval,
                }),
                Err(..) => Err(ErrorInfo {
                    message: format!("event value {} is not of type int", c.text),
                    interval,
                }),
            },
            _ => Err(ErrorInfo {
                message: "event type is unown".to_owned(),
                interval,
            }),
        },
        None => Ok(SmartLiteral{literal: Literal::null(), interval}),
    }
}

// ########################################################## Interval

pub fn interval_from_expr(expr: &Expr) -> Interval {
    match expr {
        Expr::Block{range: RangeInterval{start, ..}, ..}    => start.clone(),
        Expr::ComplexLiteral(_e, RangeInterval{start, ..})  => start.clone(),
        Expr::VecExpr(_e, RangeInterval{start, ..})         => start.clone(),
        Expr::FunctionExpr(fnexpr)                          => interval_from_reserved_fn(fnexpr),
        Expr::InfixExpr(_i, expr, _e)                       => interval_from_expr(expr), // RangeInterval
        Expr::BuilderExpr(expr, _e)                         => interval_from_expr(expr),
        Expr::IdentExpr(ident)                              => interval_from_sident(ident),
        Expr::LitExpr(literal)                              => interval_from_sliteral(literal),
        Expr::IfExpr(ifstmt)                                => interval_from_ifstmt(ifstmt),
    }
}

pub fn interval_from_ifstmt(ifstmt: &IfStatement) -> Interval {
    match ifstmt {
        IfStatement::IfStmt {ref cond, ..}  => interval_from_expr(cond),
        IfStatement::ElseStmt(_e, range)    => range.start.clone(),
    }
}

pub fn interval_from_reserved_fn(reservedfn: &ReservedFunction) -> Interval { 
    match reservedfn {
        ReservedFunction::Goto(_g, ident)       => interval_from_sident(ident),
        ReservedFunction::Use(expr)             => interval_from_expr(expr),
        ReservedFunction::Say(expr)             => interval_from_expr(expr),
        ReservedFunction::Remember(ident, ..)   => interval_from_sident(ident),
        ReservedFunction::Assign(ident, ..)     => interval_from_sident(ident), 
        ReservedFunction::As(ident, ..)         => interval_from_sident(ident),
        ReservedFunction::Import{step_name, ..} => interval_from_sident(step_name),
        ReservedFunction::Normal(ident, ..)     => interval_from_sident(ident),
    }
}

pub fn interval_from_sident(ident: &SmartIdent) -> Interval {
    ident.interval.clone()
}

pub fn interval_from_sliteral(literal: &SmartLiteral) -> Interval {
    literal.interval.clone()
}

// ##########################################################

pub fn search_str(name: &str, expr: &Expr) -> bool {
    match expr {
        Expr::IdentExpr(SmartIdent { ident, .. }) if ident == name => true,
        _ => false,
    }
}

pub fn get_var(name: SmartIdent, data: &mut Data) -> Result<SmartLiteral, ErrorInfo> {
    match &name.ident {
        var if var == EVENT => gen_literal_form_event(data.event, name.interval),
        var if var == RETRIES => Ok(SmartLiteral{literal: Literal::int(data.memory.retries, None), interval: name.interval.to_owned()}),
        _ => match data.step_vars.get(&name.ident) {
            Some(val) => Ok(SmartLiteral {
                literal: val.clone(),
                interval: name.interval
            }),
            None => search_var_memory(data.memory, name),
        },
    }
}

pub fn get_string_from_complexstring(exprs: &[Expr], data: &mut Data) -> SmartLiteral {
    let mut new_string = String::new();
    let mut interval: Option<Interval> = None;

    //TODO: log error and catch inrterval
    for elem in exprs.iter() {
        match get_var_from_ident(elem, data) {
            Ok(var) => {
                if interval.is_none() {
                    interval = Some(var.interval)
                }
                new_string.push_str(&var.literal.to_string())
            }
            Err(err) => {
                if interval.is_none() {
                    interval = Some(err.interval)
                }
                new_string.push_str(&Literal::null().to_string())
            },
        }
    }
    //TODO: check for error empty list
    SmartLiteral {
        literal: Literal::string(new_string, None),
        interval: interval.unwrap(),
    }
}

pub fn get_var_from_ident(expr: &Expr, data: &mut Data) -> Result<SmartLiteral, ErrorInfo> {
    match expr {
        Expr::LitExpr(literal) => Ok(literal.clone()),
        Expr::IdentExpr(ident, ..) => get_var(ident.clone(), data),
        Expr::BuilderExpr(..) => gen_literal_form_builder(expr, data),
        Expr::ComplexLiteral(..) => gen_literal_form_builder(expr, data),
        Expr::InfixExpr(infix, exp1, exp2) => evaluate_condition(infix, exp1, exp2, data),
        e => Err(
            ErrorInfo{
                message: "unown variable in Ident err n#1".to_owned(),
                interval: interval_from_expr(e)
            }
        )
    }
}

pub fn gen_literal_form_exp(expr: &Expr, data: &mut Data) -> Result<SmartLiteral, ErrorInfo> {
    match expr {
        Expr::LitExpr(literal) => Ok(literal.clone()),
        Expr::IdentExpr(ident, ..) => get_var(ident.clone(), data),
        e => Err(
            ErrorInfo{
                message: "Expression must be a literal or an identifier".to_owned(),
                interval: interval_from_expr(e)
            }
        ),
    }
}

pub fn gen_literal_form_builder(expr: &Expr, data: &mut Data) -> Result<SmartLiteral, ErrorInfo> {
    match expr {
        Expr::BuilderExpr(elem, exp) if search_str(PAST, elem) => {
            get_memory_action(data.memory, elem, exp)
        }
        Expr::BuilderExpr(elem, exp) if search_str(MEMORY, elem) => {
            get_memory_action(data.memory, elem, exp)
        }
        Expr::BuilderExpr(elem, exp) if search_str(METADATA, elem) => {
            get_memory_action(data.memory, elem, exp)
        }
        Expr::ComplexLiteral(vec, ..) => Ok(get_string_from_complexstring(vec, data)),
        Expr::IdentExpr(ident, ..) => get_var(ident.clone(), data),
        e => Err(
            ErrorInfo{
                message: "Error in Exprecion builder".to_owned(),
                interval: interval_from_expr(e)
            }
        ),
    }
}

// TODO: Check in MemoryType r#type are correct
pub fn memorytype_to_literal(
    memtype: Option<&MemoryType>,
    interval: Interval,
) -> Result<SmartLiteral, ErrorInfo> {
    if let Some(elem) = memtype {
        Ok(SmartLiteral{literal: elem.value.clone(), interval})
    } else {
        Err(
            ErrorInfo{
                message: "Error in memorytype_to_literal".to_owned(),
                interval
            }
        )
    }
}

// MEMORY ------------------------------------------------------------------

pub fn search_var_memory(memory: &Memory, name: SmartIdent) -> Result<SmartLiteral, ErrorInfo> {
    match &name.ident {
        var if memory.metadata.contains_key(var) => {
            memorytype_to_literal(memory.metadata.get(var), name.interval.clone())
        }
        var if memory.current.contains_key(var) => {
            memorytype_to_literal(memory.current.get(var), name.interval.clone())
        }
        var if memory.past.contains_key(var) => {
            memorytype_to_literal(memory.past.get(var), name.interval.clone())
        }
        _ => Err(
            ErrorInfo{
                message: "unown variable in search_var_memory".to_owned(),
                interval: name.interval
            }
        )
    }
}

pub fn memory_get<'a>(memory: &'a Memory, name: &Expr, expr: &Expr) -> Option<&'a MemoryType> {
    match (name, expr) {
        (
            Expr::IdentExpr(SmartIdent { ident, .. }),
            Expr::LitExpr(SmartLiteral {
                literal: Literal::StringLiteral{value, ..},
                ..
            }),
        ) if ident == PAST => memory.past.get(value),
        (
            Expr::IdentExpr(SmartIdent { ident, .. }),
            Expr::LitExpr(SmartLiteral {
                literal: Literal::StringLiteral{value, ..},
                ..
            }),
        ) if ident == MEMORY => memory.current.get(value),
        (
            _,
            Expr::LitExpr(SmartLiteral {
                literal: Literal::StringLiteral{value, ..},
                ..
            }),
        ) => memory.metadata.get(value),
        _ => None,
    }
}

pub fn memory_first<'a>(memory: &'a Memory, name: &Expr, expr: &Expr) -> Option<&'a MemoryType> {
    match (name, expr) {
        (
            Expr::IdentExpr(SmartIdent { ident, .. }),
            Expr::LitExpr(SmartLiteral {
                literal: Literal::StringLiteral{value, ..},
                ..
            }),
        ) if ident == PAST => memory.past.get_vec(value).unwrap().last(),
        (
            Expr::IdentExpr(SmartIdent { ident, .. }),
            Expr::LitExpr(SmartLiteral {
                literal: Literal::StringLiteral{value, ..},
                ..
            }),
        ) if ident == MEMORY => memory.current.get_vec(value).unwrap().last(),
        (
            _,
            Expr::LitExpr(SmartLiteral {
                literal: Literal::StringLiteral{value, ..},
                ..
            }),
        ) => memory.metadata.get_vec(value).unwrap().last(),
        _ => None,
    }
}

pub fn get_memory_action(
    memory: &Memory,
    name: &Expr,
    expr: &Expr,
) -> Result<SmartLiteral, ErrorInfo> {
    match expr {
        Expr::FunctionExpr(ReservedFunction::Normal(SmartIdent { ident, interval }, exp))
            if ident == GET_VALUE => memorytype_to_literal(memory_get(memory, name, exp), interval.clone()),
        Expr::FunctionExpr(ReservedFunction::Normal(SmartIdent { ident, interval }, exp))
            if ident == FIRST => memorytype_to_literal(memory_first(memory, name, exp), interval.clone()),
        e => Err(
            ErrorInfo{
                message: "Error in memory action".to_owned(),
                interval: interval_from_expr(e)
            }
        ),
    }
}