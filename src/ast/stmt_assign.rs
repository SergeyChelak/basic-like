use crate::interpreter::InterpreterContext;

use super::{Expression, Statement};

pub struct AssignStatement {
    name: String,
    value: Box<dyn Expression>,
}

impl AssignStatement {
    pub fn new(name: String, value: Box<dyn Expression>) -> Self {
        Self { name, value }
    }
}

impl Statement for AssignStatement {
    fn execute(&self, context: &mut InterpreterContext) {
        context.put_variable(self.name.clone(), self.value.evaluate(&context))
    }
}
