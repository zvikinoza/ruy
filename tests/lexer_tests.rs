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
            Token::new(Assign, "=".to_string()),
            Token::new(Plus, "+".to_string()),
            Token::new(LParen, "(".to_string()),
            Token::new(RParen, ")".to_string()),
            Token::new(LBrace, "{".to_string()),
            Token::new(RBrace, "}".to_string()),
            Token::new(Comma, ",".to_string()),
            Token::new(Semicolon, ";".to_string())
        ];
        let mut lexer = ruy::lexer::Lexer::new(input.to_string());
        for test in tests.iter(){
            let tok = lexer.next_token();
            assert_eq!(tok.token_type, test.token_type);
            assert_eq!(tok.literal, test.literal);
        }
        assert!(true);
    }

    #[test]
    pub fn test_next_token_on_real_code() {
        use ruy::token::*;
        use TokenType::*;
        let input = "let five = 75;
        let ten = 10;
        let add = fn(xx, y) {
            xx + y;
        };
        let result = add(five, ten);
        ";
        let tests = vec![
            Token::new(Let, "let".to_string()),
            Token::new(Identifier, "five".to_string()),
            Token::new(Assign, "=".to_string()),
            Token::new(Int, "75".to_string()),
            Token::new(Semicolon, ";".to_string()),
            Token::new(Let, "let".to_string()),
            Token::new(Identifier, "ten".to_string()),
            Token::new(Assign, "=".to_string()),
            Token::new(Int, "10".to_string()),
            Token::new(Semicolon, ";".to_string()),
            Token::new(Let, "let".to_string()),
            Token::new(Identifier, "add".to_string()),
            Token::new(Assign, "=".to_string()),
            Token::new(Function, "fn".to_string()),
            Token::new(LParen, "(".to_string()),
            Token::new(Identifier, "xx".to_string()),
            Token::new(Comma, ",".to_string()),
            Token::new(Identifier, "y".to_string()),
            Token::new(RParen, ")".to_string()),
            Token::new(LBrace, "{".to_string()),
            Token::new(Identifier, "xx".to_string()),
            Token::new(Plus, "+".to_string()),
            Token::new(Identifier, "y".to_string()),
            Token::new(Semicolon, ";".to_string()),
            Token::new(RBrace, "}".to_string()),
            Token::new(Semicolon, ";".to_string()),
            Token::new(Let, "let".to_string()),
            Token::new(Identifier, "result".to_string()),
            Token::new(Assign, "=".to_string()),
            Token::new(Identifier, "add".to_string()),
            Token::new(LParen, "(".to_string()),
            Token::new(Identifier, "five".to_string()),
            Token::new(Comma, ",".to_string()),
            Token::new(Identifier, "ten".to_string()),
            Token::new(RParen, ")".to_string()),
            Token::new(Semicolon, ";".to_string()),
            Token::new(Eof, "".to_string()),
        ];
        let mut lexer = ruy::lexer::Lexer::new(input.to_string());
        for test in tests.iter(){
            let tok = lexer.next_token();
            assert_eq!(tok.token_type, test.token_type);
            assert_eq!(tok.literal, test.literal);

        }
    }
}