use std::collections::HashMap;

use value::Value;

use crate::interpreter::InterpreterContext;

pub mod expr_operator;
pub mod expr_variable;
pub mod stmt_assign;
pub mod stmt_goto;
pub mod stmt_if_then;
pub mod stmt_input;
pub mod stmt_print;
pub mod value;
pub trait Statement {
    // TODO: add result return value
    fn execute(&self, context: &mut InterpreterContext);
}

pub trait Expression {
    fn evaluate(&self, context: &InterpreterContext) -> Value;
}
