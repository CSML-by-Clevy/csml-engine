mod actions;
mod for_loop;
mod functions;
mod if_statment;

pub use actions::match_actions;
pub use for_loop::for_loop;
pub use functions::match_functions;
pub use if_statment::{evaluate_condition, solve_if_statments};
