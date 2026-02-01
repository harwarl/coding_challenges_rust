use regex::Regex;
use crate::types::Token;
use crate::types::TokenType;

pub fn tokenizer(text: &str) -> Vec<Token> {
    let re_digits_bool = Regex::new(r"[\d\w]").unwrap();
    let re_whitespace = Regex::new(r"\s").unwrap();

    // dynamically sized array
    let mut tokens: Vec<Token> = Vec::new();

    let mut iter = text.chars().peekable();

    // for c in text.chars() {
    while let Some(c) = iter.next() {
        match c {
            '{' => {
                tokens.push(Token {
                    token_type: TokenType::BraceOpen,
                    value: "{".to_string(),
                });
            }
            '}' => {
                tokens.push(Token {
                    token_type: TokenType::BraceClosed,
                    value: "}".to_string(),
                });
            }
            '[' => {
                tokens.push(Token {
                    token_type: TokenType::BracketOpen,
                    value: "[".to_string(),
                });
            }
            ']' => {
                tokens.push(Token {
                    token_type: TokenType::BracketClosed,
                    value: "]".to_string(),
                });
            }
            ':' => {
                tokens.push(Token {
                    token_type: TokenType::Colon,
                    value: ":".to_string(),
                });
            }
            ',' => {
                tokens.push(Token {
                    token_type: TokenType::Comma,
                    value: ",".to_string(),
                });
            }
            '"' => {
                let mut value = String::new();
                let mut is_terminated = false;
                while let Some(&next_char) = iter.peek() {
                    iter.next();
                    if next_char == '"' {
                        is_terminated = true;
                        break;
                    }
                    value.push(next_char);
                }

                if !is_terminated {
                    panic!("String did not terminate");
                }

                tokens.push(Token {
                    token_type: TokenType::String,
                    value: value.to_string(),
                })
            }
            _ => {
                // check if its whitespace
                if re_whitespace.is_match(c.clone().to_string().as_str()) {
                    continue;
                }

                // check if it matches letters or numbers
                re_digits_bool.is_match(c.clone().to_string().as_str());

                let mut value = String::new();
                value.push(c);
                while let Some(&next_char) = iter.peek() {
                    iter.next();
                    if !re_digits_bool.is_match(next_char.to_string().as_str()) {
                        break;
                    }
                    value.push(next_char);
                }

                match value.as_str() {
                    "true" => tokens.push(Token {
                        token_type: TokenType::True,
                        value,
                    }),
                    "false" => tokens.push(Token {
                        token_type: TokenType::False,
                        value,
                    }),
                    "null" => tokens.push(Token {
                        token_type: TokenType::Null,
                        value,
                    }),
                    _ => match value.parse::<f64>() {
                        Ok(v) => {
                            tokens.push(Token {
                                token_type: TokenType::Number,
                                value: v.to_string(),
                            });
                        }
                        Err(e) => {
                            panic!("Unexpected value {}", e)
                        }
                    },
                }
            }
        }
    }

    println!("{:?}", tokens);
    tokens
}
