use crate::interpreter::InterpreterContext;

use super::Statement;

pub struct GotoStatement {
    label: String,
}

impl GotoStatement {
    pub fn new(label: String) -> Self {
        Self { label }
    }
}

impl Statement for GotoStatement {
    fn execute(&self, context: &mut InterpreterContext) {
        let Some(index) = context.label(&self.label) else {
            return;
        };
        context.set_statement_index(index);
    }
}
