use crate::data::primitive::{PrimitiveArray, PrimitiveBoolean, PrimitiveObject};
use crate::data::{ast::*, position::Position, tokens::*, Literal};
use crate::error_format::*;
use crate::parser::{
    operator::parse_operator, parse_comments::comment, parse_idents::parse_idents_assignation,
    tools::*,
};

use nom::error::{ContextError, ParseError};
use nom::{bytes::complete::tag, sequence::preceded, IResult};
use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn interval_from_expr(expr: &Expr) -> Interval {
    match expr {
        Expr::Scope {
            range: range_interval,
            ..
        } => *range_interval,
        Expr::ComplexLiteral(_e, range_interval) => *range_interval,
        Expr::MapExpr {
            interval: range_interval,
            ..
        } => *range_interval,
        Expr::VecExpr(_e, range_interval) => *range_interval,
        Expr::ObjectExpr(fnexpr) => interval_from_reserved_fn(fnexpr),
        Expr::InfixExpr(_i, expr, _e) => interval_from_expr(expr), // RangeInterval ?
        Expr::PostfixExpr(_p, expr) => interval_from_expr(expr),   // RangeInterval ?
        Expr::PathExpr { literal, .. } => interval_from_expr(literal),
        Expr::ForEachExpr(_, _, _, _, range_interval) => *range_interval,
        Expr::WhileExpr(_, _, range_interval) => *range_interval,
        Expr::IdentExpr(ident) => ident.interval.to_owned(),
        Expr::LitExpr { literal, .. } => literal.interval.to_owned(),
        Expr::IfExpr(ifstmt) => interval_from_if_stmt(ifstmt),
    }
}

pub fn interval_from_if_stmt(ifstmt: &IfStatement) -> Interval {
    match ifstmt {
        IfStatement::IfStmt { ref cond, .. } => interval_from_expr(cond),
        IfStatement::ElseStmt(_e, range_interval) => *range_interval,
    }
}

pub fn interval_from_reserved_fn(reserved_fn: &ObjectType) -> Interval {
    match reserved_fn {
        ObjectType::Goto(_g, interval) => interval.to_owned(),
        ObjectType::Previous(_p, interval) => interval.to_owned(),
        ObjectType::Use(expr) => interval_from_expr(expr),
        ObjectType::Do(DoType::Update(_assign, expr, ..)) => interval_from_expr(expr),
        ObjectType::Do(DoType::Exec(expr)) => interval_from_expr(expr),
        ObjectType::Say(expr) => interval_from_expr(expr),
        ObjectType::Debug(_expr, interval) => interval.to_owned(),
        ObjectType::Log { interval, .. } => interval.to_owned(),
        ObjectType::Return(expr) => interval_from_expr(expr),
        ObjectType::Remember(ident, ..) => ident.interval.to_owned(),
        ObjectType::Forget(_, interval) => interval.to_owned(),
        ObjectType::Assign(_assign, ident, ..) => interval_from_expr(ident),
        ObjectType::As(ident, ..) => ident.interval.to_owned(),
        ObjectType::BuiltIn(Function { interval, .. }) => interval.to_owned(),
        ObjectType::Hold(interval) => interval.to_owned(),
        ObjectType::HoldSecure(interval) => interval.to_owned(),
        ObjectType::Break(interval) => interval.to_owned(),
        ObjectType::Continue(interval) => interval.to_owned(),
    }
}

fn evaluate_infix(
    flow_name: &str,
    infix: &Infix,
    lhs: Result<Literal, ErrorInfo>,
    rhs: Result<Literal, ErrorInfo>,
) -> Result<Literal, ErrorInfo> {
    match (infix, lhs, rhs) {
        (Infix::Equal, Ok(lhs), Ok(rhs)) => Ok(PrimitiveBoolean::get_literal(
            lhs.primitive == rhs.primitive,
            lhs.interval,
        )),
        (Infix::NotEqual, Ok(lhs), Ok(rhs)) => Ok(PrimitiveBoolean::get_literal(
            lhs.primitive != rhs.primitive,
            lhs.interval,
        )),
        (Infix::GreaterThanEqual, Ok(lhs), Ok(rhs)) => Ok(PrimitiveBoolean::get_literal(
            lhs.primitive >= rhs.primitive,
            lhs.interval,
        )),
        (Infix::LessThanEqual, Ok(lhs), Ok(rhs)) => Ok(PrimitiveBoolean::get_literal(
            lhs.primitive <= rhs.primitive,
            lhs.interval,
        )),
        (Infix::GreaterThan, Ok(lhs), Ok(rhs)) => Ok(PrimitiveBoolean::get_literal(
            lhs.primitive > rhs.primitive,
            lhs.interval,
        )),
        (Infix::LessThan, Ok(lhs), Ok(rhs)) => Ok(PrimitiveBoolean::get_literal(
            lhs.primitive < rhs.primitive,
            lhs.interval,
        )),

        (Infix::Addition, Ok(lhs), Ok(rhs)) => {
            let primitive = lhs.primitive + rhs.primitive;
            match primitive {
                Ok(primitive) => Ok(Literal {
                    content_type: primitive.get_type().to_string(),
                    primitive,
                    additional_info: None,
                    interval: lhs.interval,
                    secure_variable: false,
                }),
                Err(err) => Err(gen_error_info(Position::new(lhs.interval, flow_name), err)),
            }
        }
        (Infix::Subtraction, Ok(lhs), Ok(rhs)) => {
            let primitive = lhs.primitive - rhs.primitive;

            match primitive {
                Ok(primitive) => Ok(Literal {
                    content_type: primitive.get_type().to_string(),
                    primitive,
                    additional_info: None,
                    interval: lhs.interval,
                    secure_variable: false,
                }),
                Err(err) => Err(gen_error_info(Position::new(lhs.interval, flow_name), err)),
            }
        }
        (Infix::Divide, Ok(lhs), Ok(rhs)) => {
            let primitive = lhs.primitive / rhs.primitive;

            match primitive {
                Ok(primitive) => Ok(Literal {
                    content_type: primitive.get_type().to_string(),
                    primitive,
                    additional_info: None,
                    interval: lhs.interval,
                    secure_variable: false,
                }),
                Err(err) => Err(gen_error_info(Position::new(lhs.interval, flow_name), err)),
            }
        }

        (Infix::Multiply, Ok(lhs), Ok(rhs)) => {
            let primitive = lhs.primitive * rhs.primitive;

            match primitive {
                Ok(primitive) => Ok(Literal {
                    content_type: primitive.get_type().to_string(),
                    primitive,
                    additional_info: None,
                    interval: lhs.interval,
                    secure_variable: false,
                }),
                Err(err) => Err(gen_error_info(Position::new(lhs.interval, flow_name), err)),
            }
        }
        (Infix::Remainder, Ok(lhs), Ok(rhs)) => {
            let primitive = lhs.primitive % rhs.primitive;

            match primitive {
                Ok(primitive) => Ok(Literal {
                    content_type: primitive.get_type().to_string(),
                    primitive,
                    additional_info: None,
                    interval: lhs.interval,
                    secure_variable: false,
                }),
                Err(err) => Err(gen_error_info(Position::new(lhs.interval, flow_name), err)),
            }
        }

        (Infix::Or, Ok(lhs), Ok(rhs)) => Ok(PrimitiveBoolean::get_literal(
            lhs.primitive.as_bool() | rhs.primitive.as_bool(),
            lhs.interval,
        )),
        (Infix::And, Ok(lhs), Ok(rhs)) => Ok(PrimitiveBoolean::get_literal(
            lhs.primitive.as_bool() & rhs.primitive.as_bool(),
            lhs.interval,
        )),

        (Infix::Match, Ok(lhs), Ok(_)) | (Infix::NotMatch, Ok(lhs), Ok(_)) => Err(gen_error_info(
            Position::new(lhs.interval, "flow"),
            "invalid operation in constant declaration".to_owned(),
        )),

        (_, Err(e), ..) | (.., Err(e)) => Err(e),
    }
}

fn evaluate_condition(
    infix: &Infix,
    expr1: &Expr,
    expr2: &Expr,
    flow_name: &str,
) -> Result<Literal, ErrorInfo> {
    match (expr1, expr2) {
        (Expr::InfixExpr(i1, ex1, ex2), Expr::InfixExpr(i2, exp_1, exp_2)) => evaluate_infix(
            &flow_name,
            infix,
            evaluate_condition(i1, ex1, ex2, flow_name),
            evaluate_condition(i2, exp_1, exp_2, flow_name),
        ),
        (Expr::InfixExpr(i1, ex1, ex2), exp) => evaluate_infix(
            &flow_name,
            infix,
            evaluate_condition(i1, ex1, ex2, flow_name),
            constant_expr_to_lit(exp, flow_name),
        ),
        (exp, Expr::InfixExpr(i1, ex1, ex2)) => evaluate_infix(
            &flow_name,
            infix,
            constant_expr_to_lit(exp, flow_name),
            evaluate_condition(i1, ex1, ex2, flow_name),
        ),
        (exp_1, exp_2) => evaluate_infix(
            &flow_name,
            infix,
            constant_expr_to_lit(exp_1, flow_name),
            constant_expr_to_lit(exp_2, flow_name),
        ),
    }
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn parse_constant<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
) -> IResult<Span<'a>, Vec<Instruction>, E>
where
    E: ParseError<Span<'a>> + ContextError<Span<'a>>,
{
    let (s, name) = preceded(comment, get_string)(s)?;

    let (s, ..) = get_tag(name, CONST)(s)?;

    let (s, name) = parse_idents_assignation(s)?;
    let (s, _) = preceded(comment, tag(ASSIGN))(s)?;
    let (s, expr) = preceded(comment, parse_operator)(s)?;

    Ok((
        s,
        vec![Instruction {
            instruction_type: InstructionScope::Constant(name.ident),
            actions: expr,
        }],
    ))
}

pub fn constant_expr_to_lit(expr: &Expr, flow_name: &str) -> Result<Literal, ErrorInfo> {
    match expr {
        Expr::MapExpr {
            object,
            interval: range_interval,
            ..
        } => {
            let mut map = HashMap::new();

            for (key, value) in object.iter() {
                map.insert(key.to_owned(), constant_expr_to_lit(&value, flow_name)?);
            }

            Ok(PrimitiveObject::get_literal(
                &map,
                range_interval.to_owned(),
            ))
        }
        Expr::VecExpr(vec, range_interval) => {
            let mut array = vec![];
            for value in vec.iter() {
                array.push(constant_expr_to_lit(value, flow_name)?)
            }

            Ok(PrimitiveArray::get_literal(
                &array,
                range_interval.to_owned(),
            ))
        }
        Expr::PostfixExpr(pretfix, expr) => {
            let value = match constant_expr_to_lit(expr, flow_name) {
                Ok(literal) => literal.primitive.as_bool(),
                Err(_) => false,
            };
            let interval = interval_from_expr(expr);

            if pretfix.len() % 2 == 0 {
                Ok(PrimitiveBoolean::get_literal(value, interval))
            } else {
                Ok(PrimitiveBoolean::get_literal(!value, interval))
            }
        }
        Expr::InfixExpr(infix, exp_1, exp_2) => {
            Ok(evaluate_condition(infix, exp_1, exp_2, flow_name)?)
        }
        Expr::LitExpr { literal, .. } => Ok(literal.clone()),

        Expr::ComplexLiteral(vec, interval) => {
            if let (1, Some(Expr::LitExpr { literal, .. })) = (vec.len(), vec.get(0)) {
                Ok(literal.clone())
            } else {
                Err(gen_error_info(
                    Position::new(interval.clone(), flow_name),
                    ERROR_INVALID_CONSTANT_EXPR.to_owned(),
                ))
            }
        }

        e => Err(gen_error_info(
            Position::new(interval_from_expr(e), flow_name),
            ERROR_INVALID_CONSTANT_EXPR.to_owned(),
        )),
    }
}
