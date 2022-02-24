pub trait Visitor<T> {
    fn visit_expr(e: &Expr) -> T;
}
