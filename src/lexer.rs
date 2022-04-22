use crate::token::Token;

#[derive(Debug)]
pub struct Lexer {
    input: String,
    read_position: usize,
    position: usize,
    ch: String,
}

impl Lexer {
    const NULL_STRING: &'static str = "\0";
    const NULL_CHAR: char = '\0';

    pub fn new(input: String) -> Self {
        let mut l = Lexer {
            input: input.clone(),
            position: 0,
            read_position: 0,
            ch: Lexer::NULL_STRING.to_string(),
        };
        l.read_char();
        l
    }

    pub fn next_token(&mut self) -> Token {
        use crate::token::TokenType::*;
        self.eat_whitespace();

        let token = match self.ch.as_str() {
            "=" => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(Eq, "==".to_string())
                } else {
                    Token::new(Assign, self.ch.clone())
                }
            },
            "!" => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(NotEq, "!=".to_string())
                } else {
                    Token::new(Bang, self.ch.clone())
                }
            },
            "/" => Token::new(Slash, self.ch.clone()),
            "*" => Token::new(Asterisk, self.ch.clone()),
            "<" => Token::new(LT, self.ch.clone()),
            ">" => Token::new(GT, self.ch.clone()),
            "+" => Token::new(Plus, self.ch.clone()),
            "-" => Token::new(Minus, self.ch.clone()),
            "(" => Token::new(LParen, self.ch.clone()),
            ")" => Token::new(RParen, self.ch.clone()),
            "{" => Token::new(LBrace, self.ch.clone()),
            "}" => Token::new(RBrace, self.ch.clone()),
            "," => Token::new(Comma, self.ch.clone()),
            ";" => Token::new(Semicolon, self.ch.clone()),
            Lexer::NULL_STRING => Token::new(Eof, "".to_string()),
            ch if char::is_alphabetic(ch.chars().next().unwrap()) => {
                let ident = self.read_identifier();
                return Token::new(Token::lookup_ident(&ident.as_str()), ident)
            },
            ch if char::is_numeric(ch.chars().next().unwrap()) => {
                let num = self.read_number();
                return Token::new(Int, num)
            },
            _ => Token::new(Illegal, Lexer::NULL_STRING.to_string()),
        };
        self.read_char();
        token
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = Lexer::NULL_STRING.to_string();
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap().to_string();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn eat_whitespace(&mut self) {
        loop {
            if self.ch != " " && self.ch != "\n" && self.ch != "\t" {
                break;
            }
            self.read_char();
        }
    }

    pub fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            Lexer::NULL_CHAR
        } else {
           self.input.chars().nth(self.read_position).unwrap()
        }
    }

    pub fn read_identifier(&mut self) -> String {
        let start_pos = self.position;
        while self.ch.chars().next().unwrap().is_ascii_alphabetic() {
            self.read_char()
        }
        self.input[start_pos..self.position].to_string()
    }

    pub fn read_number(&mut self) -> String {
        let start_pos = self.position;
        while self.ch.chars().next().unwrap().is_ascii_digit() {
            self.read_char();
        }
        self.input[start_pos..self.position].to_string()
    }
}




















