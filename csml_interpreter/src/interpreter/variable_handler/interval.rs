use crate::data::ast::{DoType, Expr, Function, IfStatement, Interval, ObjectType};

pub fn interval_from_expr(expr: &Expr) -> Interval {
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
        Expr::PostfixExpr(_p, expr) => interval_from_expr(expr), // RangeInterval ?
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
        ObjectType::Return(expr) => interval_from_expr(expr),
        ObjectType::Remember(ident, ..) => ident.interval.to_owned(),
        ObjectType::Forget(_, interval) => interval.to_owned(),
        ObjectType::Assign(_assign, ident, ..) => interval_from_expr(ident),
        ObjectType::As(ident, ..) => ident.interval.to_owned(),
        ObjectType::BuiltIn(Function { interval, .. }) => interval.to_owned(),
        ObjectType::Hold(interval) => interval.to_owned(),
        ObjectType::Break(interval) => interval.to_owned(),
        ObjectType::Continue(interval) => interval.to_owned(),
    }
}
