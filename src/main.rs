use pratt_parser_rs::parser::Parser;
use pratt_parser_rs::scanner::Scanner;

use std::io;

fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    input.trim().to_string()
}

fn main() {
    println!("Welcome to Pratt parsing for arithmetic expressions");

    let mut parser = Parser::new(Scanner::new(get_input()));
    let ast = parser.parse();
    println!("{}", ast);
}
