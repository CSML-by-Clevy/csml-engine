use crate::data::ast::*;

fn clean_interval(interval: &mut Interval) {
    interval.line = 0;
    interval.column = 0;
}

fn clean_if(if_statement: &mut IfStatement) {
    match if_statement {
        IfStatement::IfStmt {
            cond,
            consequence,
            then_branch,
        } => {
            clean_expr_intervals(cond);

            for (expr, _) in consequence.commands.iter_mut() {
                clean_expr_intervals(expr);
            }

            if let Some((branch, _)) = then_branch {
                clean_if(branch);
            }
        }
        IfStatement::ElseStmt(block, range) => {
            clean_interval(&mut range.start);
            clean_interval(&mut range.end);

            for (expr, _) in block.commands.iter_mut() {
                clean_expr_intervals(expr);
            }
        }
    }
}

fn clean_objects(object_type: &mut ObjectType) {
    match object_type {
        ObjectType::Goto(_, interval)
        | ObjectType::Break(interval)
        | ObjectType::Continue(interval)
        | ObjectType::Hold(interval) => clean_interval(interval),

        ObjectType::Say(expr) | ObjectType::Return(expr) | ObjectType::Use(expr) => {
            clean_expr_intervals(expr)
        }

        ObjectType::Do(do_type) => match do_type {
            DoType::Update(expr1, expr2) => {
                clean_expr_intervals(expr1);
                clean_expr_intervals(expr2);
            }
            DoType::Exec(expr) => clean_expr_intervals(expr),
        },

        ObjectType::Remember(identifier, expr) | ObjectType::As(identifier, expr) => {
            clean_interval(&mut identifier.interval);
            clean_expr_intervals(expr);
        }
        ObjectType::Assign(expr1, expr2) => {
            clean_expr_intervals(expr1);
            clean_expr_intervals(expr2);
        }
        ObjectType::Import {
            step_name,
            as_name,
            file_path,
        } => {
            clean_interval(&mut step_name.interval);
            if let Some(as_name) = as_name {
                clean_interval(&mut as_name.interval);
            }
            if let Some(file_path) = file_path {
                clean_interval(&mut file_path.interval);
            }
        }
        ObjectType::Normal(function) => {
            clean_interval(&mut function.interval);
            clean_expr_intervals(&mut function.args);
        }
    }
}

fn clean_path(path: &mut PathState) {
    match path {
        PathState::ExprIndex(expr) => clean_expr_intervals(expr),
        PathState::Func(func) => {
            clean_interval(&mut func.interval);
            clean_expr_intervals(&mut func.args);
        }
        _ => {}
    }
}

fn clean_expr_intervals(expr: &mut Expr) {
    match expr {
        Expr::Scope { scope, range, .. } => {
            clean_interval(&mut range.start);
            clean_interval(&mut range.end);

            for (expr, _) in scope.commands.iter_mut() {
                clean_expr_intervals(expr);
            }
        }
        Expr::ForEachExpr(identifier, opt_ident, expr, block, range) => {
            clean_interval(&mut identifier.interval);
            if let Some(opt_ident) = opt_ident {
                clean_interval(&mut opt_ident.interval);
            }
            clean_expr_intervals(expr);
            for (expr, _) in block.commands.iter_mut() {
                clean_expr_intervals(expr);
            }
            clean_interval(&mut range.start);
            clean_interval(&mut range.end);
        }
        Expr::ComplexLiteral(vec, range) | Expr::VecExpr(vec, range) => {
            clean_interval(&mut range.start);
            clean_interval(&mut range.end);

            for expr in vec.iter_mut() {
                clean_expr_intervals(expr);
            }
        }
        Expr::MapExpr(map, range) => {
            clean_interval(&mut range.start);
            clean_interval(&mut range.end);

            for (_, expr) in map.iter_mut() {
                clean_expr_intervals(expr);
            }
        }
        Expr::InfixExpr(_, expr1, expr2) => {
            clean_expr_intervals(expr1);
            clean_expr_intervals(expr2);
        }
        Expr::ObjectExpr(object_type) => clean_objects(object_type),
        Expr::IfExpr(if_condition) => clean_if(if_condition),
        Expr::PathExpr { literal, path } => {
            clean_expr_intervals(literal);

            for (interval, path) in path.iter_mut() {
                clean_interval(interval);
                clean_path(path);
            }
        }
        Expr::IdentExpr(identifier) => {
            clean_interval(&mut identifier.interval);
        }
        Expr::LitExpr(literal) => clean_interval(&mut literal.interval),
    }
}

pub fn clean_step_intervals(mut step: Block) -> Block {
    for (expr, _) in step.commands.iter_mut() {
        clean_expr_intervals(expr)
    }

    step
}
