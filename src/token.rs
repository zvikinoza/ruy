
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    Illegal,
    Eof, 

    // identifiers
    Identifier,
    Int,

    // Operators
    Assign, 
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    LT,
    GT,
    Eq,
    NotEq,

    // Delimiters
    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    // keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, lit: String) -> Self {
        Token { token_type, literal: lit }
    }

    pub fn lookup_ident(ident: &str) -> TokenType {
        use TokenType::*;
        match ident {
            "fn" => Function,
            "let" => Let,
            "true" => True,
            "false" => False,
            "if" => If,
            "else" => Else,
            "return" => Return,
            _ => Identifier
        }
    }
}