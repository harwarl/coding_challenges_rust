use std::{collections::HashMap, thread::current};

use crate::types::{Token, TokenType};

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn parse(&mut self) {
        if self.tokens.is_empty() {
            panic!("No tokens to parse");
        }

        self.parse_value();
    }

    fn parse_value(&mut self) {
        match self.peek().token_type {
            TokenType::BraceOpen => {

            }
            _ => {}
        }
    }

    // return the parsed Object
    fn parse_object(&mut self)  -> HashMap<String, ()>{
        let mut node: HashMap<String, ()>= HashMap::new();
        
        self.advance();
        
        while self.peek().token_type != TokenType::BraceClosed {
            match self.peek().token_type {
                TokenType::String => {
                    let key = self.peek().clone().value;
                    self.advance();
                    let value = self.parse_value();
                    node.insert(key, value);
                },
                _ => {
                    panic!("Expected String Key in Object!")
                }
            }
            self.advance();
            if self.peek().token_type == TokenType::Comma { self.advance(); };
        }

        node
    }

    

    // Returns current token
    fn peek(&mut self) -> &Token {
        &self.tokens[self.current]
    }

    // Advance current number and return token
    fn advance(&mut self) -> &Token {
        self.current += 1;
        &self.tokens[self.current]
    }
}

// Converts the tokenizer json to an Object that's mappable
pub fn parser(tokens: Vec<Token>) {
    let mut parser = Parser { tokens, current: 0 };

    parser.parse();
}
