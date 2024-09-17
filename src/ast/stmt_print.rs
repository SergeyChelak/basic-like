use crate::interpreter::InterpreterContext;

use super::{Expression, Statement};

pub struct PrintStatement {
    expression: Box<dyn Expression>,
}

impl PrintStatement {
    pub fn new(expression: Box<dyn Expression>) -> Self {
        Self { expression }
    }
}

impl Statement for PrintStatement {
    fn execute(&self, context: &mut InterpreterContext) {
        let val = self.expression.evaluate(context).to_string();
        println!("{val}")
    }
}
