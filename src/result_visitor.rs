use crate::astold::*;
use crate::callstack::*;
use crate::errors::*;

pub trait ResultVisitor<RETURN, NUMBER> {
    fn visit_name(&mut self, n: &Name) -> Result<(), SyntaxError>;
    fn visit_stmt(&mut self, s: &Stmt) -> Result<(), SyntaxError>;
    fn visit_expr(&mut self, e: &Expr) -> Result<RETURN, SyntaxError>;
    fn visit_body(&mut self, body: &Body) -> Result<(), SyntaxError>;

    // debugging - destroys the visitor and returns callstack
    fn to_callstack(self) -> ActivationRecord<NUMBER>;
}

pub struct ResultInterpreter {
    pub varName: Option<String>,
    pub callstack: ActivationRecord<i64>,
}

impl ResultVisitor<ActivationRecordValue<i64>, i64> for ResultInterpreter {
    fn visit_name(&mut self, n: &Name) -> Result<(), SyntaxError> {
        self.varName = Some(n.value.clone());
        return Ok(());
    }
    fn visit_stmt(&mut self, s: &Stmt) -> Result<(), SyntaxError> {
        match *s {
            Stmt::Expr(ref e) => {
                self.visit_expr(e)?;
                return Ok(());
            }
            Stmt::Let(ref name, ref expr) => {
                self.visit_name(name)?;
                let value = self.visit_expr(expr)?;
                if let Some(ref varName) = self.varName {
                    self.callstack.set(varName.clone(), value);
                    self.varName = None;
                    return Ok(());
                } else {
                    // TODO: dont use string in error
                    // "No var name."
                    return Err(SyntaxError {});
                }
            }
        }
    }

    // todo: create enum for different primitive return types, that allows for type checking! (object, )
    fn visit_expr(&mut self, expression: &Expr) -> Result<ActivationRecordValue<i64>, SyntaxError> {
        match &*expression {
            Expr::IntLit(number) => Ok(ActivationRecordValue::IntValue(*number as i64)),
            // todo: fix!
            Expr::BoolLit(boolean) => {
                if *boolean {
                    Ok(ActivationRecordValue::BooleanValue(true))
                } else {
                    Ok(ActivationRecordValue::BooleanValue(false))
                }
            }
            Expr::Variable(name) => {
                let variable = self.callstack.get(name);
                if let Some(variable) = variable {
                    return Ok((*variable).clone());
                } else {
                    return Err(SyntaxError {});
                }
            }
            Expr::UnaryAdd(ref item) => Ok(self.visit_expr(item)? as ActivationRecordValue<i64>),
            Expr::UnarySub(ref item) => {
                let expr = self.visit_expr(item)?;
                return match expr {
                    ActivationRecordValue::IntValue(int) => {
                        Ok(ActivationRecordValue::IntValue(-int))
                    }
                    // TODO: dont use string for errors
                    // "Invalid type"
                    _ => Err(SyntaxError {}),
                };
                // (-self.visit_expr(item)) as ActivationRecordValue<i64>
            }
            Expr::Add(ref lhs, ref rhs) => {
                // (self.visit_expr(lhs) + self.visit_expr(rhs)) as i64
                if let ActivationRecordValue::IntValue(left) = self.visit_expr(lhs)? {
                    if let ActivationRecordValue::IntValue(right) = self.visit_expr(rhs)? {
                        return Ok(ActivationRecordValue::IntValue(left + right));
                    }
                }
                return Err(SyntaxError {});
            }
            Expr::Sub(ref lhs, ref rhs) => {
                // (self.visit_expr(lhs) - self.visit_expr(rhs)) as i64
                if let ActivationRecordValue::IntValue(left) = self.visit_expr(lhs)? {
                    if let ActivationRecordValue::IntValue(right) = self.visit_expr(rhs)? {
                        return Ok(ActivationRecordValue::IntValue(left - right));
                    }
                }
                return Err(SyntaxError {});
            }
            Expr::Multiply(ref lhs, ref rhs) => {
                // TODO: find better way: https://stackoverflow.com/questions/53235477/does-rust-2018-support-if-let-chaining
                // (self.visit_expr(lhs) * self.visit_expr(rhs)) as i64
                if let ActivationRecordValue::IntValue(left) = self.visit_expr(lhs)? {
                    if let ActivationRecordValue::IntValue(right) = self.visit_expr(rhs)? {
                        return Ok(ActivationRecordValue::IntValue(left * right));
                    }
                }
                return Err(SyntaxError {});
            }
            Expr::Divide(ref lhs, ref rhs) => {
                // (self.visit_expr(lhs) / self.visit_expr(rhs)) as i64
                if let ActivationRecordValue::IntValue(left) = self.visit_expr(lhs)? {
                    if let ActivationRecordValue::IntValue(right) = self.visit_expr(rhs)? {
                        return Ok(ActivationRecordValue::IntValue(left / right));
                    }
                }
                return Err(SyntaxError {});
            }
        }
    }

    fn visit_body(&mut self, body: &Body) -> Result<(), SyntaxError> {
        for statement in &body.statements {
            self.visit_stmt(&statement)?;
        }
        return Ok(());
    }

    fn to_callstack(self) -> ActivationRecord<i64> {
        return self.callstack;
    }
}
