use crate::ast::*;
use crate::callstack::*;

pub trait Visitor<T> {
  fn visit_name(&mut self, n: &Name);
  fn visit_stmt(&mut self, s: &Stmt);
  fn visit_expr(&mut self, e: &Expr) -> T;

  // debugging
  fn to_callstack(self) -> ActivationRecord<T>;
}

pub struct Interpreter {
  pub varName: Option<String>,
  pub callstack: ActivationRecord<i64>,
}

impl Visitor<i64> for Interpreter {
  fn visit_name(&mut self, n: &Name) {
    self.varName = Some(n.value.clone());
    // panic!()
  }
  fn visit_stmt(&mut self, s: &Stmt) {
    match *s {
      Stmt::Expr(ref e) => {
        self.visit_expr(e);
      }
      Stmt::Let(ref name, ref expr) => {
        self.visit_name(name);
        let value = self.visit_expr(expr);
        if let Some(ref varName) = self.varName {
          self
            .callstack
            .set(varName.clone(), ActivationRecordValue::IntValue(value))
        } else {
          panic!("No var name!")
        }
        self.varName = None;
        // unimplemented!()
      }
    }
  }

  // todo: create enum for different primitive return types, that allows for type checking! (object, )
  fn visit_expr(&mut self, expression: &Expr) -> i64 {
    match &*expression {
      Expr::IntLit(number) => *number as i64,
      // todo: fix!
      Expr::BoolLit(boolean) => {
        if *boolean {
          1
        } else {
          0
        }
      }
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

  fn to_callstack(self) -> ActivationRecord<i64> {
    return self.callstack;
  }
}
