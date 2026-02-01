use std::collections::HashMap;

use crate::types::{Token, TokenType, ASTNODE};

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn parse(&mut self) -> ASTNODE {
        if self.tokens.is_empty() {
            panic!("No tokens to parse");
        }

        self.parse_value()
    }

    fn parse_value(&mut self) -> ASTNODE {
        match self.peek().token_type {
            TokenType::BraceOpen => self.parse_object(),
            TokenType::BracketOpen => self.parse_array(),
            TokenType::String => ASTNODE::String(self.peek().clone().value),
            TokenType::Number => match self.peek().clone().value.parse::<f64>() {
                Ok(v) => ASTNODE::Number(v),
                Err(e) => panic!("Error getting parsed value: {:?}", e),
            },
            TokenType::False => ASTNODE::Boolean(false),
            TokenType::True => ASTNODE::Boolean(true),
            TokenType::Null => ASTNODE::Null,
            _ => panic!("Unexpected token type: {:?}", self.peek().clone().token_type)
        }
    }

    // return the parsed Object
    fn parse_object(&mut self) -> ASTNODE {
        let mut map: HashMap<String, ASTNODE> = HashMap::new();
        self.advance();

        while self.peek().token_type != TokenType::BraceClosed {
            match self.peek().token_type {
                TokenType::String => {
                    let key = self.peek().clone().value;
                    // Check if there is a colon
                    let token = self.advance();
                    if token.token_type != TokenType::Colon {
                        panic!("Expected: In key-value pair")
                    }
                    self.advance();

                    let value = self.parse_value();
                    map.insert(key, value);
                }
                _ => {
                    panic!("Expected String Key in Object!")
                }
            }
            let token = self.advance();
            if token.token_type == TokenType::Comma {
                self.advance();
            };
        }

        ASTNODE::Object(map)
    }

    // returns the AST node of the parsed array
    fn parse_array(&mut self) -> ASTNODE {
        let mut array: Vec<ASTNODE> = Vec::new();
        self.advance(); // eat {

        while self.peek().token_type != TokenType::BracketClosed {
            array.push(self.parse_value());

            self.advance();
            let value = self.peek().clone();

            if value.token_type == TokenType::Comma { self.advance(); }  // eat ","
        }
        ASTNODE::Array(array)
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
    let ast = parser.parse();
    println!("{:?}", ast);
}
