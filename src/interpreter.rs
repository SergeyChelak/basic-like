use std::collections::HashMap;

trait Expression {
    fn evaluate(&mut self) -> dyn Value;
}

trait Value {
    fn to_string(&self) -> String;

    fn to_number(&self) -> f32;
}

pub struct Interpreter {
    labels: HashMap<String, u32>,
    variables: HashMap<String, Box<dyn Value>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            labels: Default::default(),
            variables: Default::default(),
        }
    }

    pub fn interpret(&mut self, source: &str) {
        // tokenize

        // parse

        // interpret
        todo!()
    }
}
