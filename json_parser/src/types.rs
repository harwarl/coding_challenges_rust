#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenType {
    BraceOpen,
    BraceClosed,
    BracketOpen,
    BracketClosed,
    String,
    Number,
    Comma,
    Colon,
    True,
    False,
    Null,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) value: String,
}
