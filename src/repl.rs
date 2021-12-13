use crate::eval::eval;
use crate::eval::Environment;
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::io::{self, Write};

/// Start the REPL.
pub fn start() {
    let mut environment = Environment::new();

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
        if !parser.errors.is_empty() {
            parser.errors.iter().for_each(|e| {
                println!("{:?}", e);
            })
        }

        let evaluated = eval(program, &mut environment);
        println!("{}", evaluated.inspect());
    }
}
