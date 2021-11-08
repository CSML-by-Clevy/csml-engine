use crate::data::{ast::*, Literal};

////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURES
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq)]
pub enum ExitCondition {
    Goto,
    End,
    Error,
    Break,
    Continue,
    Hold,
    Return(Literal),
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn count_if_commands(if_statement: &mut IfStatement, index: &mut usize) {
    match if_statement {
        IfStatement::IfStmt {
            consequence: scope,
            then_branch,
            last_action_index,
            ..
        } => {
            count_scope_commands(scope, index);
            if *index >= 1 {
                *last_action_index = *index -1;
            }

            if let Some(else_scope) = then_branch {
                count_if_commands(else_scope, index)
            }
        }
        IfStatement::ElseStmt(scope, ..) => count_scope_commands(scope, index),
    }
}

fn count_scope_commands(scope: &mut Block, index: &mut usize) {
    for (command, info) in scope.commands.iter_mut() {
        count_commands(command, index, info);
    }
}

pub fn count_commands(command: &mut Expr, index: &mut usize, info: &mut InstructionInfo) {
    let start_index = *index;

    match command {
        Expr::ObjectExpr(..) => {
            info.index = *index;
            *index = *index + 1
        },

        Expr::IfExpr(if_statement) => {
            info.index = *index;
            count_if_commands(if_statement, index)
        }
        Expr::ForEachExpr(_ident, _index, _expr, block, _range) => {
            info.index = *index;
            count_scope_commands(block, index)
        }
        Expr::WhileExpr(_expr, block, _range) => {
            info.index = *index;
            count_scope_commands(block, index)
        }
        _ => {}
    }

    if *index > start_index + 1 {
        info.total = *index - (start_index + 1);
    }
}