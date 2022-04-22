use crate::token::Token;
use crate::token::TokenType;
use crate::token::TokenType::{Assign, Comma, Illegal, LBrace, LParen, Plus, RBrace, RParen, Semicolon};

pub struct Lexer {
    input: String,
    read_position: usize,
    position: usize,
    ch: String,
}

impl Lexer {
    const NULL_STRING: &'static str = "\0";

    pub fn new(input: String) -> Self {
        let mut l = Lexer {
            input: input.clone(),
            position: 0,
            read_position: 0,
            ch: Lexer::NULL_STRING.to_string(),
        };
        println!("new {}", l.ch);
        l
    }

    pub fn next_token(&mut self) -> Token {
        self.eat_whitespace();
        self.read_char();
        // let input = "=+(){},;";
        match self.ch.as_str() {
            "=" => Token::new(Assign, self.ch.clone()),
            "+" => Token::new(Plus, self.ch.clone()),
            "(" => Token::new(LParen, self.ch.clone()),
            ")" => Token::new(RParen, self.ch.clone()),
            "{" => Token::new(LBrace, self.ch.clone()),
            "}" => Token::new(RBrace, self.ch.clone()),
            "," => Token::new(Comma, self.ch.clone()),
            ";" => Token::new(Semicolon, self.ch.clone()),
            _ => Token::new(Illegal, Lexer::NULL_STRING.to_string())
        }
        // Token{ token_type: TokenType::Illegal, literal: self.ch }
    }

    pub fn read_char(&mut self) {
        // dummy impl, to satisfy lserver
        if self.read_position >= self.input.len() {
            self.ch = "\n".to_string();
        } else {
            self.ch = match self.input.chars().nth(self.read_position) {
                None => Lexer::NULL_STRING.to_string(),
                Some(t) => t.to_string(),
            }
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn eat_whitespace(&mut self) {
        while self.ch.trim().is_empty() {
            self.read_char();
        }
    }

    pub fn peek_char(&self) -> String {
        if self.read_position >= self.input.len() {
            Lexer::NULL_STRING.to_string()
        } else {
            match self.input.chars().nth(self.read_position) {
                None => Lexer::NULL_STRING.to_string(),
                Some(t) => t.to_string(),
            }
        }
    }

    pub fn read_identifier(&mut self) -> String {
        let start_pos = self.position;
        while self.ch.chars().all(char::is_alphanumeric) {
            self.read_char()
        }
        self.input[start_pos..self.position].to_string()
    }

    pub fn read_number(&mut self) -> String {
        let start_pos = self.position;
        while self.ch.chars().all(char::is_numeric) {
            self.read_char();
        }
        self.input[start_pos..self.position].to_string()
    }
}




















