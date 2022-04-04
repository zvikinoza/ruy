use crate::token::Token;



pub struct Lexer {
    input: String,
    read_position: usize,
    position: usize,
    ch: char
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let l = Lexer { input, position: 0, read_position: 0, ch: char::from_u32(0).expect("fail to conver null char to `char`")};
        l.read_char();
        return l;
    }

    pub fn read_char(&self) -> char {
        // dummy impl, to satisfy lserver
        if self.read_position >= self.input.len() {
            return '1';
        } else if self.position > 12 {
            return 'b';
        } 
        return self.ch;
    }

    pub fn next_token(&self) -> Token {
        // dummy impl, to satisfy lserver
        Token { token_type: crate::token::TokenType::Assign, literal: "=".to_string() }
    }
}