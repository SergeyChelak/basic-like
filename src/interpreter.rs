use std::collections::HashMap;

use crate::{
    ast::{value::Value, Statement},
    parser::Parser,
    tokenizer::Tokenizer,
};

pub fn interpret(source: &str) {
    // tokenize
    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(source);

    let mut context = InterpreterContext::default();
    // parse
    let mut parser = Parser::new(tokens, &mut context);
    parser.parse();

    // interpret
    context.run()
}

#[derive(Default)]
pub struct InterpreterContext {
    labels: HashMap<String, usize>,
    variables: HashMap<String, Value>,
    statements: Vec<Box<dyn Statement>>,
    statement_index: usize,
}

impl InterpreterContext {
    pub fn variable(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }

    pub fn label(&self, name: &str) -> Option<usize> {
        self.labels.get(name).map(|x| *x)
    }

    pub fn put_variable(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    pub fn put_label(&mut self, label: String, position: usize) {
        self.labels.insert(label, position);
    }

    pub fn put_statement(&mut self, statement: Box<dyn Statement>) {
        self.statements.push(statement)
    }

    pub fn statements_count(&self) -> usize {
        self.statements.len()
    }

    pub fn set_statement_index(&mut self, index: usize) {
        self.statement_index = index;
    }

    fn run(&mut self) {
        self.statement_index = 0;
        while self.statement_index < self.statements_count() {
            let index = self.statement_index;
            self.statement_index += 1;
            // let statement = self.statements[index].clone();
            // statement.execute(self);
        }
    }
}
