use crate::{
    ast::{
        expr_operator::OperatorExpression,
        expr_variable::VariableExpression,
        statement::Statement,
        value::{Double, Value},
        Expression,
    },
    interpreter::InterpreterContext,
    tokenizer::{Token, TokenType},
};

pub struct Parser<'a> {
    tokens: Vec<Token>,
    position: usize,
    context: &'a mut InterpreterContext,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>, context: &'a mut InterpreterContext) -> Self {
        Self {
            tokens,
            position: 0,
            context,
        }
    }

    pub fn parse(&mut self) {
        loop {
            while self.match_type(TokenType::Line) {}

            if self.match_type(TokenType::Label) {
                // Mark the index of the statement after the label
                self.context
                    .put_label(self.last(1).text, self.context.statements_count());
            } else if self.match_types(TokenType::Word, TokenType::Equals) {
                let name = self.last(2).text;
                let value = self.expression();
                let statement = Statement::assign(name, value);
                self.context.put_statement(statement);
            } else if self.match_name("print") {
                let statement = Statement::print(self.expression());
                self.context.put_statement(statement);
            } else if self.match_name("input") {
                let name = self.consume_type(TokenType::Word).text;
                let statement = Statement::input(name);
                self.context.put_statement(statement);
            } else if self.match_name("goto") {
                let name = self.consume_type(TokenType::Word).text;
                let statement = Statement::goto(name);
                self.context.put_statement(statement);
            } else if self.match_name("if") {
                let condition = self.expression();
                self.consume_name("then".to_string());
                let label = self.consume_type(TokenType::Word).text;
                let statement = Statement::if_then(condition, label);
                self.context.put_statement(statement);
            } else {
                // Unexpected token (likely EOF), so end
                break;
            }
        }
    }

    fn expression(&mut self) -> Box<dyn Expression> {
        self.operator()
    }

    fn operator(&mut self) -> Box<dyn Expression> {
        let mut expression = self.atomic();

        while self.match_type(TokenType::Operator) || self.match_type(TokenType::Equals) {
            let op = self
                .last(1)
                .text
                .chars()
                .next()
                .expect("Operator can't be empty");
            let right = self.atomic();
            expression = Box::new(OperatorExpression::new(expression, op, right));
        }
        expression
    }

    fn atomic(&mut self) -> Box<dyn Expression> {
        if self.match_type(TokenType::Word) {
            // A word is a reference to a variable
            return Box::new(VariableExpression::new(self.last(1).text));
        }
        if self.match_type(TokenType::Number) {
            let val = self
                .last(1)
                .text
                .parse::<Double>()
                .expect("Failed to convert string to double");
            return Box::new(Value::number(val));
        }
        if self.match_type(TokenType::String) {
            return Box::new(Value::string(self.last(1).text));
        }
        if self.match_type(TokenType::OpenParenthesis) {
            // The contents of a parenthesized expression can be any
            // expression. This lets us "restart" the precedence cascade
            // so that you can have a lower precedence expression inside
            // the parentheses.
            let expr = self.expression();
            self.consume_type(TokenType::CloseParenthesis);
            return expr;
        }
        panic!("Parse error")
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

    fn match_name(&mut self, name: &str) -> bool {
        let token = self.get(0);
        if token.t_type != TokenType::Word {
            return false;
        }
        if token.text != name {
            return false;
        }
        self.position += 1;
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

    /// Consumes the next token if it's the given type. If not, throws an
    /// exception. This is for cases where the parser demands a token of a
    /// certain type in a certain position, for example a matching ) after
    /// an opening (.
    fn consume_type(&mut self, t_type: TokenType) -> Token {
        if self.get(0).t_type != t_type {
            panic!("Expected {:?}", t_type)
        }
        let token = self.tokens[self.position].clone();
        self.position += 1;
        token
    }

    /// Consumes the next token if it's a word with the given name. If not,
    /// throws an exception.
    fn consume_name(&mut self, name: String) -> Token {
        if !self.match_name(&name) {
            panic!("Expected {name}")
        }
        self.last(1)
    }
}
