use super::token::*;

pub struct Scanner {
    input: String,
    curr_idx: usize,
    curr_char: char,
    curr_spelling: String,
}

impl Scanner {
    pub fn new(mut input: String) -> Self {
        input.push(0 as char);
        let mut scanner = Scanner {
            input,
            curr_idx: 0,
            curr_char: 0 as char,
            curr_spelling: String::new(),
        };
        scanner.curr_char = scanner.input.chars().nth(scanner.curr_idx).unwrap();
        scanner
    }

    pub fn scan(&mut self) -> Token {
        if self.curr_char.is_whitespace() {
            self.skip_whitespace();
        }

        self.curr_spelling = String::new();
        let kind = self.scan_token();
        Token::new(kind, self.curr_spelling.to_owned())
    }

    fn scan_token(&mut self) -> TokenType {
        match self.curr_char {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                self.eat_it();
                while self.curr_char.is_ascii_digit() {
                    self.eat_it();
                }
                TokenType::Number
            }
            '+' => {
                self.eat_it();
                TokenType::Plus
            }
            '-' => {
                self.eat_it();
                TokenType::Minus
            }

            '*' => {
                self.eat_it();
                TokenType::Mul
            }

            '/' => {
                self.eat_it();
                TokenType::Div
            }

            '%' => {
                self.eat_it();
                TokenType::Mod
            }

            '^' => {
                self.eat_it();
                TokenType::Pow
            }

            '(' => {
                self.eat_it();
                TokenType::LParen
            }

            ')' => {
                self.eat_it();
                TokenType::RParen
            }

            '\0' => TokenType::Eof,
            _ => panic!("unexpected character {}", self.curr_char),
        }
    }

    fn eat_it(&mut self) {
        if self.curr_idx >= self.input.len() {
            panic!("ate all the characters!");
        }

        self.curr_spelling.push(self.curr_char);
        self.curr_idx += 1;
        self.curr_char = self.input.chars().nth(self.curr_idx).unwrap();
    }

    fn skip_it(&mut self) {
        if self.curr_idx >= self.input.len() {
            panic!("skipped all the characters!");
        }
        self.curr_idx += 1;
        self.curr_char = self.input.chars().nth(self.curr_idx).unwrap();
    }

    fn skip_whitespace(&mut self) {
        while self.curr_char.is_whitespace() {
            self.skip_it();
        }
    }
}
