use crate::data::{ast::*, tokens::*};
use crate::interpreter::variable_handler::interval::interval_from_expr;

use nom::InputTake;

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn get_step_offset(
    name: &str,
    offsets: &[(String, usize)],
) -> ((String, usize), Option<(String, usize)>) {
    let mut step_info = None;
    let mut index = 0;

    for (i, (step_name, offset)) in offsets.iter().enumerate() {
        if step_name == name {
            step_info = Some((step_name.to_owned(), offset.to_owned()));
            index = i;
            break;
        }
    }

    match step_info {
        Some(step_info) => {
            if offsets.len() > index + 1 {
                let next_step = offsets[index + 1].clone();
                (step_info, Some(next_step))
            } else {
                (step_info, None)
            }
        }
        None => unreachable!(),
    }
}

fn get_skip_offset(skip_offsets: &[usize], offset: usize) -> Option<usize> {
    for skip_offset in skip_offsets.iter() {
        if *skip_offset > offset {
            return Some(*skip_offset);
        }
    }
    None
}

fn get_next_offset(
    offset: usize,
    next_step: Option<(String, usize)>,
    skip_offsets: &[usize],
) -> Option<usize> {
    match next_step {
        Some((_, next_step_offset)) => match get_skip_offset(skip_offsets, offset) {
            Some(skip_offset) => {
                if skip_offset > next_step_offset {
                    Some(skip_offset)
                } else {
                    Some(next_step_offset)
                }
            }
            None => Some(next_step_offset),
        },
        None => get_skip_offset(skip_offsets, offset),
    }
}

fn get_offsets(ast: &Flow) -> (Vec<(String, usize)>, Vec<usize>) {
    let mut offsets = vec![];
    let mut skip_offsets = vec![];

    for (instruction_type, block) in ast.flow_instructions.iter() {
        match instruction_type {
            InstructionScope::StepScope(name) => {
                let interval = interval_from_expr(block);
                offsets.push((name.to_owned(), interval.offset))
            }
            InstructionScope::FunctionScope { .. } | InstructionScope::ImportScope(_) => {
                let interval = interval_from_expr(block);
                skip_offsets.push(interval.offset)
            }
        }
    }
    offsets.sort_by(|(_, a), (_, b)| a.cmp(b));
    skip_offsets.sort_by(|a, b| a.cmp(b));

    (offsets, skip_offsets)
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn get_step<'a>(step_name: &'a str, flow: &'a str, ast: &'a Flow) -> String {
    let (offsets, skip_offsets) = get_offsets(ast);
    let span = Span::new(flow);

    let ((_, offset), next_step) = get_step_offset(step_name, &offsets);
    let (new, _) = span.take_split(offset);
    match get_next_offset(offset, next_step, &skip_offsets) {
        Some(skip_offset) => {
            let (_, old) = new.take_split(skip_offset - offset);
            old.fragment().to_string()
        }
        None => new.fragment().to_string(),
    }
}
