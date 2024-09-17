use std::io;

use crate::{
    ast::value::{Double, Value},
    interpreter::InterpreterContext,
};

use super::Statement;

pub struct InputStatement {
    name: String,
}

impl InputStatement {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl Statement for InputStatement {
    fn execute(&self, context: &mut InterpreterContext) {
        let mut buffer = String::new();
        // TODO: replace stdin to input from context
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read from stdin");

        let value = if let Ok(val) = buffer.parse::<Double>() {
            Value::number(val)
        } else {
            Value::string(buffer)
        };
        context.put_variable(self.name.clone(), value);
    }
}
