use crate::parser::ast::{Expr, IfStatement, Interval, ObjectType, RangeInterval};

pub fn interval_from_expr(expr: &Expr) -> Interval {
    match expr {
        Expr::Block {
            range: RangeInterval { start, .. },
            ..
        } => start.clone(),
        Expr::ComplexLiteral(_e, RangeInterval { start, .. }) => start.clone(),
        Expr::VecExpr(_e, RangeInterval { start, .. }) => start.clone(),
        Expr::ObjectExpr(fnexpr) => interval_from_reserved_fn(fnexpr),
        Expr::InfixExpr(_i, expr, _e) => interval_from_expr(expr), // RangeInterval
        Expr::BuilderExpr(expr, _e) => interval_from_expr(expr),
        Expr::ForExpr(_, _, _, _, RangeInterval { start, .. }) => start.clone(),
        Expr::IdentExpr(ident) => ident.interval.to_owned(),
        Expr::LitExpr(literal) => literal.get_interval(),
        Expr::IfExpr(ifstmt) => interval_from_if_stmt(ifstmt),
    }
}

pub fn interval_from_if_stmt(ifstmt: &IfStatement) -> Interval {
    match ifstmt {
        IfStatement::IfStmt { ref cond, .. } => interval_from_expr(cond),
        IfStatement::ElseStmt(_e, range) => range.start.clone(),
    }
}

pub fn interval_from_reserved_fn(reservedfn: &ObjectType) -> Interval {
    match reservedfn {
        ObjectType::Goto(_g, ident) => ident.interval.to_owned(),
        ObjectType::Use(expr) => interval_from_expr(expr),
        ObjectType::Say(expr) => interval_from_expr(expr),
        ObjectType::Remember(ident, ..) => ident.interval.to_owned(),
        ObjectType::Assign(ident, ..) => ident.interval.to_owned(),
        ObjectType::As(ident, ..) => ident.interval.to_owned(),
        ObjectType::Import { step_name, .. } => step_name.interval.to_owned(),
        ObjectType::Normal(ident, ..) => ident.interval.to_owned(),
    }
}
