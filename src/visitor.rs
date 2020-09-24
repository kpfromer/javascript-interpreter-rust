use crate::ast::*;
use crate::callstack::*;

pub trait Visitor<T> {
  fn visit_name(&mut self, n: &Name);
  fn visit_stmt(&mut self, s: &Stmt);
  fn visit_expr(&mut self, e: &Expr) -> T;

  // debugging
  fn to_callstack(self) -> ActivationRecord<i64>;
}

pub struct Interpreter {
  pub varName: Option<String>,
  pub callstack: ActivationRecord<i64>,
}

impl Visitor<ActivationRecordValue<i64>> for Interpreter {
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
          self.callstack.set(varName.clone(), value)
        } else {
          panic!("No var name!")
        }
        self.varName = None;
        // unimplemented!()
      }
    }
  }

  // todo: create enum for different primitive return types, that allows for type checking! (object, )
  fn visit_expr(&mut self, expression: &Expr) -> ActivationRecordValue<i64> {
    match &*expression {
      Expr::IntLit(number) => ActivationRecordValue::IntValue(*number as i64),
      // todo: fix!
      Expr::BoolLit(boolean) => {
        if *boolean {
          ActivationRecordValue::BooleanValue(true)
        } else {
          ActivationRecordValue::BooleanValue(false)
        }
      }
      Expr::Variable(name) => {
        let variable = self.callstack.get(name);
        if let Some(variable) = variable {
          if let ActivationRecordValue::IntValue(value) = variable {
            return ActivationRecordValue::IntValue(*value as i64);
          } else {
            panic!();
          }
        } else {
          panic!();
        }
      }
      Expr::UnaryAdd(ref item) => self.visit_expr(item) as ActivationRecordValue<i64>,
      Expr::UnarySub(ref item) => {
        let expr = self.visit_expr(item);
        return match expr {
          ActivationRecordValue::IntValue(int) => ActivationRecordValue::IntValue(-int),
          _ => panic!("Invalid type"),
        };
        // (-self.visit_expr(item)) as ActivationRecordValue<i64>
      }
      Expr::Add(ref lhs, ref rhs) => {
        // (self.visit_expr(lhs) + self.visit_expr(rhs)) as i64

        // if let ActivationRecordValue::IntValue(left) = self.visit_expr(lhs) && let ActivationRecordValue::IntValue(right) = self.visit_expr(rhs) {
        //   return ActivationRecordValue::IntValue(left + right);
        // }

        if let ActivationRecordValue::IntValue(left) = self.visit_expr(lhs) {
          if let ActivationRecordValue::IntValue(right) = self.visit_expr(rhs) {
            return ActivationRecordValue::IntValue(left + right);
          }
        }
        panic!();
      }
      Expr::Sub(ref lhs, ref rhs) => {
        // (self.visit_expr(lhs) - self.visit_expr(rhs)) as i64
        if let ActivationRecordValue::IntValue(left) = self.visit_expr(lhs) {
          if let ActivationRecordValue::IntValue(right) = self.visit_expr(rhs) {
            return ActivationRecordValue::IntValue(left - right);
          }
        }
        panic!();
      }
      Expr::Multiply(ref lhs, ref rhs) => {
        // (self.visit_expr(lhs) * self.visit_expr(rhs)) as i64
        if let ActivationRecordValue::IntValue(left) = self.visit_expr(lhs) {
          if let ActivationRecordValue::IntValue(right) = self.visit_expr(rhs) {
            return ActivationRecordValue::IntValue(left * right);
          }
        }
        panic!();
      }
      Expr::Divide(ref lhs, ref rhs) => {
        // (self.visit_expr(lhs) / self.visit_expr(rhs)) as i64
        if let ActivationRecordValue::IntValue(left) = self.visit_expr(lhs) {
          if let ActivationRecordValue::IntValue(right) = self.visit_expr(rhs) {
            return ActivationRecordValue::IntValue(left / right);
          }
        }
        panic!();
      }
    }
  }

  fn to_callstack(self) -> ActivationRecord<i64> {
    return self.callstack;
  }
}
