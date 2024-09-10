use std::collections::HashMap;

use crate::{
    ast::Statement,
    tokenizer::{Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
    labels: HashMap<String, usize>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
            labels: Default::default(),
        }
    }

    pub fn parse(&mut self) -> Vec<Box<dyn Statement>> {
        let mut statements = Vec::new();
        loop {
            while self.match_type(TokenType::Line) {}

            if self.match_type(TokenType::Label) {
                // Mark the index of the statement after the label
                self.labels.insert(self.last(1).text, statements.len());
            } else if self.match_types(TokenType::Word, TokenType::Equals) {
                let name = self.last(2).text;
            }
        }
        statements
    }

    /// Gets a previously consumed token, indexing backwards. last(1) will
    /// be the token just consumed, last(2) the one before that, etc.
    fn last(&self, offset: usize) -> Token {
        self.tokens[self.position - offset].clone()
    }

    /// Consumes the next token if it's the given type.
    fn match_type(&mut self, t_type: TokenType) -> bool {
        if self.get(0).t_type != t_type {
            return false;
        }
        self.position += 1;
        true
    }

    /// Consumes the next two tokens if they are the given type (in order).
    /// Consumes no tokens if either check fails.
    fn match_types(&mut self, type1: TokenType, type2: TokenType) -> bool {
        if self.get(0).t_type != type1 {
            return false;
        }
        if self.get(1).t_type != type2 {
            return false;
        }
        self.position += 2;
        true
    }

    /// Gets an unconsumed token, indexing forward. get(0) will be the next
    /// token to be consumed, get(1) the one after that, etc.
    fn get(&self, offset: usize) -> Token {
        self.tokens
            .get(self.position + offset)
            .unwrap_or(&Token::eof())
            .clone()
    }
}
