use std::collections::HashMap;

use crate::token::Tokenizer;

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
    // current_statement: usize,
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
        let mut tokenizer = Tokenizer::new();
        let tokens = tokenizer.tokenize(source);

        // parse

        // interpret
        // self.current_statement = 0;

        todo!()
    }
}
