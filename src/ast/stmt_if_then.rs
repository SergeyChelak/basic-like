use crate::interpreter::InterpreterContext;

use super::{Expression, Statement};

pub struct IfThenStatement {
    condition: Box<dyn Expression>,
    label: String,
}

impl IfThenStatement {
    pub fn new(condition: Box<dyn Expression>, label: String) -> Self {
        Self { condition, label }
    }
}

impl Statement for IfThenStatement {
    fn execute(&self, context: &mut InterpreterContext) {
        let Some(position) = context.label(&self.label) else {
            return;
        };
        let val = self.condition.evaluate(context).to_number();
        if val != 0.0 {
            context.set_statement_index(position);
        }
    }
}
