use crate::eval::eval;
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::io::{self, Write};

pub fn start() {
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("could not read input");

        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        if parser.errors.len() > 0 {
            parser.errors.iter().for_each(|e| {
                println!("{:?}", e);
            })
        }

        let evaluated = eval(program);
        println!("{}", evaluated.inspect());
    }
}
