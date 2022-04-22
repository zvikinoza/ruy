
#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    Illegal,
    Eof, 

    // identifiers
    Identifier,
    Int,

    // Operators
    Assign, 
    Plus,

    // Delimiters
    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    // keywords
    Function,
    Let
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, lit: String) -> Self {
        Token { token_type, literal: lit }
    }
}