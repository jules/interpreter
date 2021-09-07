pub mod lexer;
pub mod tokens;

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::tokens::{Token, TokenType};

    #[test]
    fn test_next_token() {
        let input = "let five = 5;
        let ten = 10;
        let add = function(x, y) {
            x + y;
        };
        let result = add(five, ten);";

        let mut lexer = Lexer::new(input);

        let tokens = {
            let mut tokens = vec![];

            loop {
                let token = lexer.next_token();
                if token.t == TokenType::EOF {
                    break;
                }

                tokens.push(token);
            }

            tokens
        };

        let expected_tokens = vec![
            Token::new(TokenType::Let, String::from("let")),
            Token::new(TokenType::Ident, String::from("five")),
            Token::new(TokenType::Assign, String::from("=")),
            Token::new(TokenType::Int, String::from("5")),
            Token::new(TokenType::Semicolon, String::from(";")),
            Token::new(TokenType::Let, String::from("let")),
            Token::new(TokenType::Ident, String::from("ten")),
            Token::new(TokenType::Assign, String::from("=")),
            Token::new(TokenType::Int, String::from("10")),
            Token::new(TokenType::Semicolon, String::from(";")),
            Token::new(TokenType::Let, String::from("let")),
            Token::new(TokenType::Ident, String::from("add")),
            Token::new(TokenType::Assign, String::from("=")),
            Token::new(TokenType::Function, String::from("function")),
            Token::new(TokenType::LParen, String::from("(")),
            Token::new(TokenType::Ident, String::from("x")),
            Token::new(TokenType::Comma, String::from(",")),
            Token::new(TokenType::Ident, String::from("y")),
            Token::new(TokenType::RParen, String::from(")")),
            Token::new(TokenType::LBrace, String::from("{")),
            Token::new(TokenType::Ident, String::from("x")),
            Token::new(TokenType::Plus, String::from("+")),
            Token::new(TokenType::Ident, String::from("y")),
            Token::new(TokenType::Semicolon, String::from(";")),
            Token::new(TokenType::RBrace, String::from("}")),
            Token::new(TokenType::Semicolon, String::from(";")),
            Token::new(TokenType::Let, String::from("let")),
            Token::new(TokenType::Ident, String::from("result")),
            Token::new(TokenType::Assign, String::from("=")),
            Token::new(TokenType::Ident, String::from("add")),
            Token::new(TokenType::LParen, String::from("(")),
            Token::new(TokenType::Ident, String::from("five")),
            Token::new(TokenType::Comma, String::from(",")),
            Token::new(TokenType::Ident, String::from("ten")),
            Token::new(TokenType::RParen, String::from(")")),
            Token::new(TokenType::Semicolon, String::from(";")),
        ];

        assert!(tokens
            .iter()
            .zip(expected_tokens.iter())
            .all(|(t, e)| *t == *e));
    }
}
