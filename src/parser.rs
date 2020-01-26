use crate::ast::*;
use crate::lexer::*;
use std::mem;

pub struct Parser<'a> {
  pub lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
  // fn eat<T: TokenKind>(&self mut, TokenKind kind) -> T {
  //   if let T =
  // }

  // Literals
  pub fn number(&mut self) -> Expr {
    let token = self.lexer.getNextToken();
    if let TokenKind::IntegerConstant(value) = token.kind {
      return Expr::IntLit(value);
    } else {
      println!("Invalid {:?}", token);
      panic!();
    }
  }

  // Expressions
  // factor: IntegerConstant | Add factor | Minus factor
  pub fn factor(&mut self) -> Expr {
    let token = self.lexer.getNextToken();
    match token.kind {
      TokenKind::IntegerConstant(value) => Expr::IntLit(value),
      // TokenKind::Add => Expr::
      _ => panic!(),
    }
  }

  // factor ((Multiply | Divide) factor)*
  pub fn multDivFactor(&mut self) -> Expr {
    let mut factor1 = self.factor();
    loop {
      match self.lexer.peek(&1) {
        Ok(token) => match token.kind {
          TokenKind::Multiply => {
            self.lexer.getNextToken();
            factor1 = Expr::Multiply(Box::new(factor1), Box::new(self.factor()));
          }
          TokenKind::Divide => {
            self.lexer.getNextToken();
            factor1 = Expr::Divide(Box::new(factor1), Box::new(self.factor()));
          }
          _ => break,
        },
        Err(_) => break,
      }
    }
    return factor1;
  }

  // multDivFactor ((Plus | Minus) multDivFactor)*
  pub fn addSubFactor(&mut self) -> Expr {
    let mut factor1 = self.multDivFactor();
    loop {
      match self.lexer.peek(&1) {
        Ok(token) => match token.kind {
          TokenKind::Plus => {
            self.lexer.getNextToken();
            factor1 = Expr::Add(Box::new(factor1), Box::new(self.multDivFactor()));
          }
          TokenKind::Minus => {
            self.lexer.getNextToken();
            factor1 = Expr::Sub(Box::new(factor1), Box::new(self.multDivFactor()));
          }
          _ => break,
        },
        Err(_) => break,
      }
    }
    return factor1;
  }

  // Program Structure

  // Statements

  // Function/Procedure Setup
}
