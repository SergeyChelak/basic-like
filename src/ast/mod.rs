use value::Value;

use crate::interpreter::InterpreterContext;

pub mod expr_operator;
pub mod expr_variable;
pub mod statement;
pub mod value;

pub trait Expression {
    fn evaluate(&self, context: &InterpreterContext) -> Value;
}
