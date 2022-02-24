use super::ast::Expr;
use super::scanner::Scanner;
use super::token::{Token, TokenType};

const MAX_BINDING_POWER: i32 = 40;
const MIN_BINDING_POWER: i32 = -1;

pub struct Parser {
    scanner: Scanner,
    curr_tok: Token,
}

impl Parser {
    pub fn new(mut scanner: Scanner) -> Self {
        let curr_tok = scanner.scan();
        Parser { scanner, curr_tok }
    }

    pub fn parse(&mut self) -> Expr {
        let expr = self.parse_expression(0);
        self.curr_tok = self.scanner.scan();
        if self.curr_tok.kind != TokenType::Eof {
            panic!("Expected for find Eof, found {}", self.curr_tok);
        }
        expr
    }

    fn parse_expression(&mut self, rbp: i32) -> Expr {
        let mut left = self.nud(self.curr_tok.clone());
        self.curr_tok = self.scanner.scan();

        while rbp < Parser::lbp(self.curr_tok.kind) {
            let tok = self.curr_tok.clone();
            self.curr_tok = self.scanner.scan();

            if Parser::is_right_associative(tok.kind) {
                let right = Box::new(self.parse_expression(Parser::lbp(tok.kind) - 1));
                left = self.led(Box::new(left), tok.kind, right);
            } else {
                let right = Box::new(self.parse_expression(Parser::lbp(tok.kind)));
                left = self.led(Box::new(left), tok.kind, right);
            }
        }
        left
    }

    fn lbp(kind: TokenType) -> i32 {
        match kind {
            TokenType::Plus | TokenType::Minus => 10,
            TokenType::Mul | TokenType::Div => 20,
            TokenType::Pow => 30,
            TokenType::Mod => 40,
            _ => MIN_BINDING_POWER,
        }
    }

    fn is_right_associative(kind: TokenType) -> bool {
        match kind {
            TokenType::Pow => true,
            _ => false,
        }
    }

    fn nud(&mut self, tok: Token) -> Expr {
        match tok.kind {
            TokenType::Number => Expr::IntLit(tok.spelling.trim().parse::<i64>().unwrap()),
            TokenType::LParen => {
                self.curr_tok = self.scanner.scan();
                let expr = self.parse_expression(0);
                if self.curr_tok.kind != TokenType::RParen {
                    panic!("Missing parens");
                }
                expr
            }
            TokenType::Plus | TokenType::Minus => {
                let kind = tok.kind;
                self.curr_tok = self.scanner.scan();
                Expr::UnaryExpr(kind, Box::new(self.parse_expression(MAX_BINDING_POWER)))
            }
            _ => panic!("Invalid token for nud {}", tok),
        }
    }

    fn led(&mut self, left: Box<Expr>, kind: TokenType, right: Box<Expr>) -> Expr {
        Expr::BinaryExpr(left, kind, right)
    }
}
