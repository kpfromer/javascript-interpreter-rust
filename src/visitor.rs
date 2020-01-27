use crate::ast::*;
use crate::callstack::*;

pub trait Visitor<T> {
  fn visit_name(&mut self, n: &Name) -> T;
  fn visit_stmt(&mut self, s: &Stmt) -> T;
  fn visit_expr(&mut self, e: &Expr) -> T;
}

pub struct Interpreter {
  pub callstack: ActivationRecord,
}

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
    match &*e {
      Expr::IntLit(n) => *n as i64,
      Expr::Variable(name) => {
        let variable = self.callstack.get(name);
        if let Some(variable) = variable {
          if let ActivationRecordValue::IntValue(value) = variable {
            return *value as i64;
          } else {
            panic!();
          }
        } else {
          panic!();
        }
      }
      Expr::UnaryAdd(ref item) => self.visit_expr(item) as i64,
      Expr::UnarySub(ref item) => (-self.visit_expr(item)) as i64,
      Expr::Add(ref lhs, ref rhs) => (self.visit_expr(lhs) + self.visit_expr(rhs)) as i64,
      Expr::Sub(ref lhs, ref rhs) => (self.visit_expr(lhs) - self.visit_expr(rhs)) as i64,
      Expr::Multiply(ref lhs, ref rhs) => (self.visit_expr(lhs) * self.visit_expr(rhs)) as i64,
      Expr::Divide(ref lhs, ref rhs) => (self.visit_expr(lhs) / self.visit_expr(rhs)) as i64,
    }
  }
}
