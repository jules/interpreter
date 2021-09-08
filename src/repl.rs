use crate::lexer::Lexer;
use crate::tokens::TokenType;
use std::io::{self, Write};

pub fn start() {
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("could not read input");

        let mut lexer = Lexer::new(&input);
        loop {
            let token = lexer.next_token();
            if token.t == TokenType::EOF {
                break;
            }

            println!("{:?}", token);
        }
    }
}
