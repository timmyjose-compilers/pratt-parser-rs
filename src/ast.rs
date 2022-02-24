use super::token::TokenType;
use std::fmt;

#[derive(Debug)]
pub enum Expr {
    IntLit(i64),
    UnaryExpr(TokenType, Box<Expr>),
    BinaryExpr(Box<Expr>, TokenType, Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Expr::IntLit(n) => format!("{}", n),
                Expr::UnaryExpr(op, ref e) => format!("({}{})", op, e),
                Expr::BinaryExpr(ref lhs, op, ref rhs) => format!("({} {} {})", lhs, op, rhs),
            },
        )
    }
}
