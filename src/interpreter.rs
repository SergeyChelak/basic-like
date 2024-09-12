use crate::{ast::value::Value, parser::Parser, tokenizer::Tokenizer};
use std::collections::HashMap;

pub struct Interpreter {
    labels: HashMap<String, u32>,
    variables: HashMap<String, Value>,
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
        let mut parser = Parser::new(tokens);

        // interpret
        // self.current_statement = 0;

        todo!()
    }
}
