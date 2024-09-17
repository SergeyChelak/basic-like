use crate::interpreter::InterpreterContext;

use super::{value::Value, Expression};

pub struct VariableExpression {
    name: String,
}

impl VariableExpression {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl Expression for VariableExpression {
    fn evaluate(&self, context: &InterpreterContext) -> Value {
        context
            .variable(&self.name)
            .unwrap_or(&Value::Number(0.0))
            .clone()
    }
}
