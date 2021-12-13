pub mod ast;
pub mod eval;
pub mod lexer;
pub mod parser;
pub mod repl;
pub mod tokens;

fn main() {
    repl::start();
}
