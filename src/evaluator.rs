use super::ast::Expr;
use super::token::TokenType;
use super::visitor::Visitor;

pub struct Evaluator;

impl Evaluator {
    pub fn evaluate(&mut self, ast: &Expr) -> i64 {
        self.visit_expr(ast)
    }
}

impl Visitor<i64> for Evaluator {
    fn visit_expr(&mut self, e: &Expr) -> i64 {
        match *e {
            Expr::IntLit(n) => n,
            Expr::UnaryExpr(op, ref e) => match op {
                TokenType::Plus => self.visit_expr(e),
                TokenType::Minus => -self.visit_expr(e),
                _ => unreachable!(),
            },

            Expr::BinaryExpr(ref lhs, op, ref rhs) => match op {
                TokenType::Plus => self.visit_expr(lhs) + self.visit_expr(rhs),
                TokenType::Minus => self.visit_expr(lhs) - self.visit_expr(rhs),
                TokenType::Mul => self.visit_expr(lhs) * self.visit_expr(rhs),
                TokenType::Div => self.visit_expr(lhs) / self.visit_expr(rhs),
                TokenType::Mod => self.visit_expr(lhs) % self.visit_expr(rhs),
                TokenType::Pow => self.visit_expr(lhs).pow(self.visit_expr(rhs) as u32),
                _ => unreachable!(),
            },
        }
    }
}
