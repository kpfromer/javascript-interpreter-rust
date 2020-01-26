use crate::ast::*;

pub trait Visitor<T> {
  fn visit_name(&mut self, n: &Name) -> T;
  fn visit_stmt(&mut self, s: &Stmt) -> T;
  fn visit_expr(&mut self, e: &Expr) -> T;
}

pub struct Interpreter;
impl Visitor<i64> for Interpreter {
  fn visit_name(&mut self, n: &Name) -> i64 {
    panic!()
  }
  fn visit_stmt(&mut self, s: &Stmt) -> i64 {
    match *s {
      Stmt::Expr(ref e) => self.visit_expr(e),
      Stmt::Let(..) => unimplemented!(),
    }
  }

  fn visit_expr(&mut self, e: &Expr) -> i64 {
    match *e {
      Expr::IntLit(n) => n as i64,
      Expr::Add(ref lhs, ref rhs) => (self.visit_expr(lhs) + self.visit_expr(rhs)) as i64,
      Expr::Sub(ref lhs, ref rhs) => (self.visit_expr(lhs) - self.visit_expr(rhs)) as i64,
      Expr::Multiply(ref lhs, ref rhs) => (self.visit_expr(lhs) * self.visit_expr(rhs)) as i64,
      Expr::Divide(ref lhs, ref rhs) => (self.visit_expr(lhs) / self.visit_expr(rhs)) as i64,
    }
  }
}
