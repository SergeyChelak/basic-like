use std::{collections::HashMap, io};

use crate::{
    ast::{
        statement::Statement,
        value::{Double, Value},
    },
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
    statements: Vec<Statement>,
    statement_index: usize,
}

impl InterpreterContext {
    pub fn variable(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }

    pub fn label(&self, name: &str) -> Option<usize> {
        self.labels.get(name).copied()
    }

    pub fn put_variable(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    pub fn put_label(&mut self, label: String, position: usize) {
        self.labels.insert(label, position);
    }

    pub fn put_statement(&mut self, statement: Statement) {
        self.statements.push(statement)
    }

    pub fn statements_count(&self) -> usize {
        self.statements.len()
    }

    fn run(&mut self) {
        self.statement_index = 0;
        loop {
            let index = self.statement_index;
            self.statement_index += 1;
            let Some(statement) = self.statements.get(index) else {
                break;
            };
            use Statement::*;
            match statement {
                Assign { name, value } => {
                    let eval = value.evaluate(self);
                    self.put_variable(name.clone(), eval);
                }
                Goto { label } => {
                    if let Some(index) = self.label(label) {
                        self.statement_index = index;
                    }
                }
                IfThen { condition, label } => {
                    if let Some(index) = self.label(label) {
                        let val = condition.evaluate(self).to_number();
                        if val != 0.0 {
                            self.statement_index = index;
                        }
                    }
                }
                Print { expression } => {
                    let eval = expression.evaluate(self).to_text();
                    println!("{eval}")
                }
                Input { name } => {
                    let mut buffer = String::new();
                    // TODO: replace stdin to local variable
                    io::stdin()
                        .read_line(&mut buffer)
                        .expect("Failed to read from stdin");

                    let value = if let Ok(val) = buffer.trim_end().parse::<Double>() {
                        Value::number(val)
                    } else {
                        Value::string(buffer)
                    };
                    self.put_variable(name.clone(), value);
                }
            }
        }
    }
}
