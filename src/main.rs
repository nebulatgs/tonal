use lexer::Lexer;
use parser::Parser;

mod lexer;
mod tokens;
mod parser;
mod nodes;

fn main() {
    let source = include_str!("../example/test.tn");
    let lexer = Lexer::new(source);
    let parser = Parser::new(lexer);
    let node = parser.parse();
    dbg!(&node);
    // while let Some(token) = lexer.lex() {
    //     println!("{:?}", token);
    // }
}
