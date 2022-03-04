#![feature(bench_black_box)]
use lexer::Lexer;
use parser::Parser;

mod errors;
mod lexer;
mod nodes;
mod parser;
mod tokens;

fn main() {
    let source = include_str!("../example/test.tn");
    let lexer = Lexer::new(source);
    let parser = Parser::new(lexer);
    let node = parser.parse();
    let a = node.unwrap();
    dbg!(&a);
}
