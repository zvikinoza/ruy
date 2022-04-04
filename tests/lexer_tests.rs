extern crate ruy;
pub use ruy::token;

#[cfg(test)]
mod lexer_tests {
    #[test]
    fn test_next_token() {
        use ruy::token::*;
        use TokenType::*;
        let input = "=+(){},;";
        let tests = [
            Token::new(Assign, &"=".to_string() ),
            Token::new(Plus, &"+".to_string() ),
            Token::new(LParen, &"(".to_string() ),
            Token::new(RParen, &")".to_string() ),
            Token::new(LBrace, &"{".to_string() ),
            Token::new(RBrace, &"}".to_string() ),
            Token::new(Comma, &",".to_string() ),
            Token::new(Semicolon, &";".to_string())
        ];
        let lexer = ruy::lexer::Lexer::new(input.to_string());
        for test in tests.iter(){
            let tok = lexer.next_token();
            assert_eq!(tok.token_type, test.token_type);
            assert_eq!(tok.literal, test.literal);
        }
        assert!(true);
    }
}