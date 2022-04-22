use std::io;
use std::io::{BufRead, Write};
use crate::lexer::Lexer;
use crate::token::TokenType;

const PROMPT: &str = ">>> ";

pub fn start() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    loop {
        print!("{}", PROMPT);
        io::stdout().flush().expect("Unable to flush to stdout");
        let mut input = String::new();
        handle.read_line(&mut input).expect("Error while reading stdin");
        if input == "q\n" || input == "exit\n" {
            break;
        }
        let mut lexer = Lexer::new(input);
        let mut tok = lexer.next_token();
        while tok.token_type != TokenType::Eof {
            println!("{:?}", tok);
            tok = lexer.next_token()
        }
    }
}
