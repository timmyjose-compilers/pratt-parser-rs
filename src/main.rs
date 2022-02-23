use pratt_parser_rs::scanner::Scanner;
use pratt_parser_rs::token::*;

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

    let input = get_input();
    let mut scanner = Scanner::new(input);
    let mut tok;
    loop {
        tok = scanner.scan();
        println!("{}", tok);
        if tok.kind == TokenType::Eof {
            break;
        }
    }
}
