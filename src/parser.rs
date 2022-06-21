use crate::lexer;
use crate::token;
use crate::ast;

struct Parser {
    lexer: lexer::Lexer,
    cur_token: token::Token,
    peek_token: token::Token,
}


impl Parser {
    fn new(mut lex: lexer::Lexer) -> Self {
        let cur = lex.next_token();
        let nxt = lex.next_token();
        Self { lexer: lex, cur_token: cur, peek_token: nxt }
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_program(&self) -> ast::Program {
        ast::Program{ statements: vec![] }
    }
}