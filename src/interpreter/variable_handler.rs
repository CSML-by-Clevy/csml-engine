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
                    literal: Literal::string(c.text.to_string()),
                    interval,
                }
            ),
            PayLoad { content_type: ref t, content: ref c }
                if t == "float" => match c.text.to_string().parse::<f64>() {
                Ok(float) => Ok(SmartLiteral {
                    literal: Literal::float(float),
                    interval,
                }),
                Err(..) => Err(ErrorInfo {
                    message: format!("event value {} is not of type float", c.text),
                    interval,
                }),
            },
            PayLoad { content_type: ref t, content: ref c }
                if t == "int" => match c.text.to_string().parse::<i64>() {
                Ok(int) => Ok(SmartLiteral {
                    literal: Literal::int(int),
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
        Expr::ObjectExpr(fnexpr)                            => interval_from_reserved_fn(fnexpr),
        Expr::InfixExpr(_i, expr, _e)                       => interval_from_expr(expr), // RangeInterval
        Expr::BuilderExpr(expr, _e)                         => interval_from_expr(expr),
        Expr::IdentExpr(ident)                              => ident.interval.to_owned(),
        Expr::LitExpr(literal)                              => literal.interval.to_owned(),
        Expr::IfExpr(ifstmt)                                => interval_from_if_stmt(ifstmt),
    }
}

pub fn interval_from_if_stmt(ifstmt: &IfStatement) -> Interval {
    match ifstmt {
        IfStatement::IfStmt {ref cond, ..}  => interval_from_expr(cond),
        IfStatement::ElseStmt(_e, range)    => range.start.clone(),
    }
}

pub fn interval_from_reserved_fn(reservedfn: &ObjectType) -> Interval { 
    match reservedfn {
        ObjectType::Goto(_g, ident)       => ident.interval.to_owned(),
        ObjectType::Use(expr)             => interval_from_expr(expr),
        ObjectType::Say(expr)             => interval_from_expr(expr),
        ObjectType::Remember(ident, ..)   => ident.interval.to_owned(),
        ObjectType::Assign(ident, ..)     => ident.interval.to_owned(), 
        ObjectType::As(ident, ..)         => ident.interval.to_owned(),
        ObjectType::Import{step_name, ..} => step_name.interval.to_owned(),
        ObjectType::Normal(ident, ..)     => ident.interval.to_owned(),
    }
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
        var if var == RETRIES => Ok(SmartLiteral{literal: Literal::int(data.memory.retries), interval: name.interval.to_owned()}),
        _ => match data.step_vars.get(&name.ident) {
            Some( val) => gen_smartliteral(val, name.interval, &name.index),
            None => search_var_memory(data.memory, name),
        },
    }
}

// TODO: remove when change message system
fn extract_value_from_literal(literal: Literal) -> String {
    match &literal {
        Literal::ObjectLiteral{properties} => {
            if let Some(value) = properties.get("text") {
                extract_value_from_literal(value.clone())
            }
            else {
                literal.to_owned().to_string()
            }
        },
        literal => literal.to_owned().to_string()
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
                new_string.push_str( &extract_value_from_literal(var.literal))
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
        literal: Literal::string(new_string),
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
                message: "unknown variable in Ident err get_var_from_ident".to_owned(),
                interval: interval_from_expr(e)
            }
        )
    }
}

// TODO: tmp fn gen_literal_form_exp
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

// TODO: tmp fn find_value_in_object
fn find_value_in_object(literal: &Literal, expr: &Expr, interval: &Interval) -> Result<SmartLiteral, ErrorInfo> {
    let map = match literal {
        Literal::ObjectLiteral{properties} => properties,
        Literal::FunctionLiteral{value, ..} => {
            let literal: &Literal = value;
            match literal {
                Literal::ObjectLiteral{properties} => properties,
                _ => return Err(
                    ErrorInfo{
                        message: "Error ... bad type".to_owned(),
                        interval: interval_from_expr(expr)
                    }
                )
            }
        }
        _ => return Err(
            ErrorInfo{
                message: "Error: Bad Expression in object builder ".to_owned(),
                interval: interval.to_owned()
            }
        )
    };

    match expr {
        Expr::BuilderExpr(elem, expr) => {
            let elem :&Expr = elem;
            if let Expr::IdentExpr(ident, ..) = elem {
                let literal = match map.get(&ident.ident) {
                    Some(val) => val,
                    None => return Err(
                        ErrorInfo{
                            message: format!("Error: Key {} not found in object", ident.ident),
                            interval: ident.interval.to_owned()
                        }
                    )
                };
                find_value_in_object(literal, expr, interval)
            } else {
                return Err(
                    ErrorInfo{
                        message: "Error in Object builder".to_owned(),
                        interval: interval.to_owned()
                    }
                )
            }
        },
        Expr::IdentExpr(ident, ..) => {
            match map.get(&ident.ident) {
                Some(literal) => Ok( SmartLiteral{literal: literal.to_owned(), interval: interval.to_owned()} ),
                None => return Err(
                    ErrorInfo{
                        message: format!("Error: Key {} not found in object", ident.ident),
                        interval: ident.interval.to_owned()
                    }
                )
            }
        },
        e   => Err(
            ErrorInfo{
                message: "Error: Bad Expression in object builder ".to_owned(),
                interval: interval_from_expr(e)
            }
        )
    }
}

pub fn gen_literal_form_builder(expr: &Expr, data: &mut Data) -> Result<SmartLiteral, ErrorInfo> {
    match expr {
        Expr::BuilderExpr(elem, expr) if search_str(PAST, elem) => {
            get_memory_action(data.memory, elem, expr)
        }
        Expr::BuilderExpr(elem, expr) if search_str(MEMORY, elem) => {
            get_memory_action(data.memory, elem, expr)
        }
        Expr::BuilderExpr(elem, expr) if search_str(METADATA, elem) => {
            get_memory_action(data.memory, elem, expr)
        }
        Expr::BuilderExpr(elem, expr) => {
            let elem :&Expr = elem;
            if let Expr::IdentExpr(ident) = elem {
                let literal = get_var(ident.clone(), data)?.literal;
                find_value_in_object(&literal, expr, &ident.interval)
            } else {
                Err(
                    ErrorInfo{
                        message: "Error in Object builder".to_owned(),
                        interval: interval_from_expr(elem)
                    }
                )
            }
        }
        Expr::ComplexLiteral(vec, ..) => Ok(get_string_from_complexstring(vec, data)),
        Expr::IdentExpr(ident, ..) => get_var(ident.clone(), data),
        e => Err(
            ErrorInfo{
                message: "Error in Expression builder".to_owned(),
                interval: interval_from_expr(e)
            }
        ),
    }
}

//TODO: return Warning or Error component
fn gen_smartliteral(literal: &Literal, interval: Interval, opt: &Option<i64>) -> Result<SmartLiteral, ErrorInfo> {
    match (literal, opt) {
        (Literal::ArrayLiteral{ref items}, Some(int)) => {
            match items.get(*int as usize) {
                Some(value) => Ok(SmartLiteral{literal: value.to_owned(), interval}),
                None => Err(ErrorInfo{
                    message: format!("Error Array don't have {} index", int),
                    interval
                })
            }
        }, 
        (_, Some(_)) => Err(ErrorInfo{
            message: "Error value is not of type Array".to_owned(),
            interval
        }),
        (literal, None) => Ok(SmartLiteral{literal: literal.to_owned(), interval})
    }
}

pub fn memorytype_to_literal(
    memtype: Option<&MemoryType>,
    interval: Interval,
    index: &Option<i64>,
) -> Result<SmartLiteral, ErrorInfo> {
    match memtype {
        Some(elem) => gen_smartliteral(&elem.value, interval, index),
        None => Err(
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
            memorytype_to_literal(memory.metadata.get(var), name.interval.clone(), &name.index)
        }
        var if memory.current.contains_key(var) => {
            memorytype_to_literal(memory.current.get(var), name.interval.clone(), &name.index)
        }
        var if memory.past.contains_key(var) => {
            memorytype_to_literal(memory.past.get(var), name.interval.clone(), &name.index)
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
        Expr::ObjectExpr(ObjectType::Normal(SmartIdent{ident, interval, index }, expr))
            if ident == GET_VALUE => memorytype_to_literal(memory_get(memory, name, expr), interval.clone(), index),
        Expr::ObjectExpr(ObjectType::Normal(SmartIdent{ident, interval, index }, expr))
            if ident == FIRST => memorytype_to_literal(memory_first(memory, name, expr), interval.clone(), index),
        e => Err(
            ErrorInfo{
                message: "Error in memory action".to_owned(),
                interval: interval_from_expr(e)
            }
        ),
    }
}
