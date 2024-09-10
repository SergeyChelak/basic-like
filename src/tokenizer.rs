use std::{collections::HashMap, mem::swap};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    Word,
    Number,
    String,
    Label,
    Line,
    Equals,
    Operator,
    OpenParenthesis,
    CloseParenthesis,
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub text: String,
    pub t_type: TokenType,
}

impl Token {
    fn new(text: &str, t_type: TokenType) -> Self {
        Self {
            text: text.to_string(),
            t_type,
        }
    }

    fn with_char(ch: char, t_type: TokenType) -> Self {
        Self {
            text: ch.to_string(),
            t_type,
        }
    }

    pub fn eof() -> Self {
        Self::new("", TokenType::Eof)
    }
}

#[derive(Debug)]
enum State {
    Default,
    Word,
    Number,
    String,
    Comment,
}

pub struct Tokenizer {
    char_token_map: HashMap<char, TokenType>,
    accumulator: String,
    tokens: Vec<Token>,
    state: State,
}

impl Tokenizer {
    pub fn new() -> Self {
        Self {
            char_token_map: Self::char_token_map(),
            accumulator: String::new(),
            tokens: Default::default(),
            state: State::Default,
        }
    }

    fn char_token_map() -> HashMap<char, TokenType> {
        let tuples = [
            ('\n', TokenType::Line),
            ('=', TokenType::Equals),
            ('+', TokenType::Operator),
            ('-', TokenType::Operator),
            ('*', TokenType::Operator),
            ('/', TokenType::Operator),
            ('<', TokenType::Operator),
            ('>', TokenType::Operator),
            ('(', TokenType::OpenParenthesis),
            (')', TokenType::CloseParenthesis),
        ];
        tuples.into_iter().collect()
    }

    pub fn tokenize(&mut self, source: &str) -> Vec<Token> {
        self.state = State::Default;
        self.accumulator.clear();
        self.tokens.clear();
        for ch in source.chars() {
            loop {
                let mut redo = false;
                match self.state {
                    State::Default => {
                        if let Some(t_type) = self.char_token_map.get(&ch) {
                            self.tokens.push(Token::with_char(ch, *t_type))
                        } else if ch.is_alphabetic() {
                            self.accumulator.push(ch);
                            self.state = State::Word;
                        } else if ch.is_ascii_digit() {
                            self.accumulator.push(ch);
                            self.state = State::Number;
                        } else if ch == '"' {
                            self.state = State::String;
                        } else if ch == '\'' {
                            self.state = State::Comment;
                        }
                    }
                    State::Word => {
                        if ch.is_alphanumeric() {
                            self.accumulator.push(ch);
                        } else if ch == ':' {
                            self.push_accumulator(TokenType::Label);
                        } else {
                            self.push_accumulator(TokenType::Word);
                            redo = true;
                        }
                    }
                    State::Number => {
                        // HACK: Negative numbers and floating points aren't supported.
                        // To get a negative number, just do 0 - <your number>.
                        // To get a floating point, divide.
                        if ch.is_ascii_digit() {
                            self.accumulator.push(ch);
                        } else {
                            self.push_accumulator(TokenType::Number);
                            redo = true;
                        }
                    }
                    State::String => {
                        if ch == '"' {
                            self.push_accumulator(TokenType::String);
                        } else {
                            self.accumulator.push(ch);
                        }
                    }
                    State::Comment => {
                        if ch == '\n' {
                            self.state = State::Default;
                        }
                    }
                }
                if !redo {
                    break;
                }
            }
        }
        self.flush();
        let mut result = vec![];
        swap(&mut result, &mut self.tokens);
        result
    }

    fn flush(&mut self) {
        if !self.accumulator.is_empty() {
            match self.state {
                State::Number => self.push_accumulator(TokenType::Number),
                State::Word => self.push_accumulator(TokenType::Word),
                State::String => self.push_accumulator(TokenType::String),
                _ => {}
            }
        }
        self.state = State::Default;
        self.accumulator.clear();
    }

    fn push_accumulator(&mut self, t_type: TokenType) {
        self.tokens.push(Token::new(&self.accumulator, t_type));
        self.accumulator.clear();
        self.state = State::Default;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tokenize_comment() {
        let script = "' this is a comment";
        let mut tokenizer = Tokenizer::new();
        let tokens = tokenizer.tokenize(script);
        assert!(tokens.is_empty());
    }

    #[test]
    fn tokenize_digit() {
        let script = "12345";
        let mut tokenizer = Tokenizer::new();
        let tokens = tokenizer.tokenize(script);
        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0].t_type, TokenType::Number));
        assert_eq!(tokens[0].text, "12345");
    }

    #[test]
    fn tokenize_word() {
        let script = "abc";
        let mut tokenizer = Tokenizer::new();
        let tokens = tokenizer.tokenize(script);
        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0].t_type, TokenType::Word));
        assert_eq!(tokens[0].text, "abc");
    }

    #[test]
    fn tokenize_label() {
        let script = "abc:";
        let mut tokenizer = Tokenizer::new();
        let tokens = tokenizer.tokenize(script);
        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0].t_type, TokenType::Label));
        assert_eq!(tokens[0].text, "abc");
    }

    #[test]
    fn tokenize_word_with_digits() {
        let script = "abc123";
        let mut tokenizer = Tokenizer::new();
        let tokens = tokenizer.tokenize(script);
        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0].t_type, TokenType::Word));
        assert_eq!(tokens[0].text, "abc123");
    }

    #[test]
    fn tokenize_string() {
        let script = "\"string string string\"";
        let mut tokenizer = Tokenizer::new();
        let tokens = tokenizer.tokenize(script);
        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0].t_type, TokenType::String));
        assert_eq!(tokens[0].text, "string string string");
    }

    #[test]
    fn tokenize_char_operators() {
        let mut tokenizer = Tokenizer::new();
        for ch in "+-*/<>".chars() {
            let script = ch.to_string();
            let tokens = tokenizer.tokenize(&script);
            assert_eq!(tokens.len(), 1);
            assert!(matches!(tokens[0].t_type, TokenType::Operator));
        }
    }

    #[test]
    fn tokenize_expression() {
        let script = "2+2=4";
        let mut tokenizer = Tokenizer::new();
        let tokens = tokenizer.tokenize(script);
        assert_eq!(tokens.len(), 5);
        assert!(matches!(tokens[0].t_type, TokenType::Number));
        assert!(matches!(tokens[1].t_type, TokenType::Operator));
        assert!(matches!(tokens[2].t_type, TokenType::Number));
        assert!(matches!(tokens[3].t_type, TokenType::Equals));
        assert!(matches!(tokens[4].t_type, TokenType::Number));
    }
}
