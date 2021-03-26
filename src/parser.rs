use crate::astold::*;
use crate::lexer::*;

use std::process;

pub struct Parser<'a> {
    pub lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    // fn eat<T: TokenKind>(&self mut, TokenKind kind) -> T {
    //   if let T =
    // }

    // Literals
    pub fn number(&mut self) -> Expr {
        let token = self.lexer.get_next_token();
        if let TokenKind::IntegerConstant(value) = token.kind {
            return Expr::IntLit(value);
        } else {
            println!("Invalid {:?}", token);
            panic!();
        }
    }

    // Expressions
    // factor: IntegerConstant | Id | Plus factor | Minus factor | LeftParen addSubFactor RightParen
    pub fn factor(&mut self) -> Expr {
        let token = self.lexer.get_next_token();
        match token.kind {
            TokenKind::IntegerConstant(value) => Expr::IntLit(value),
            TokenKind::True => Expr::BoolLit(true),
            TokenKind::False => Expr::BoolLit(false),
            TokenKind::Id(value) => Expr::Variable(value),
            TokenKind::Plus => Expr::UnaryAdd(Box::new(self.factor())),
            TokenKind::Minus => Expr::UnarySub(Box::new(self.factor())),
            TokenKind::LeftParen => {
                let fact = self.add_sub_factor();
                if self.lexer.get_next_token().kind != TokenKind::RightParen {
                    panic!();
                }
                return fact;
            }
            unknown => {
                println!("Unknown Token: {:?}", unknown);
                panic!();
            }
        }
    }

    // factor ((Multiply | Divide) factor)*
    pub fn mult_div_factor(&mut self) -> Expr {
        let mut factor = self.factor();

        while let Ok(token) = self.lexer.peek(1) {
            match token.kind {
                TokenKind::Multiply => {
                    self.lexer.get_next_token();
                    factor = Expr::Multiply(Box::new(factor), Box::new(self.factor()));
                }
                TokenKind::Divide => {
                    self.lexer.get_next_token();
                    factor = Expr::Divide(Box::new(factor), Box::new(self.factor()));
                }
                _ => break,
            }
        }

        factor
    }

    // multDivFactor ((Plus | Minus) multDivFactor)*
    pub fn add_sub_factor(&mut self) -> Expr {
        let mut factor = self.mult_div_factor();

        while let Ok(token) = self.lexer.peek(1) {
            match token.kind {
                TokenKind::Plus => {
                    self.lexer.get_next_token();
                    factor = Expr::Add(Box::new(factor), Box::new(self.mult_div_factor()));
                }
                TokenKind::Minus => {
                    self.lexer.get_next_token();
                    factor = Expr::Sub(Box::new(factor), Box::new(self.mult_div_factor()));
                }
                _ => break,
            }
        }

        factor
    }

    // Program Structure

    // Statements
    pub fn name(&mut self) -> Name {
        let token = self.lexer.get_next_token();
        if let TokenKind::Id(value) = token.kind {
            return Name { value };
        }
        println!("Invalid {:?}", token);
        panic!();
    }

    // let NAME = EXPR ;
    pub fn let_statement(&mut self) -> Stmt {
        let token = self.lexer.get_next_token();
        if let TokenKind::Id(value) = token.kind {
            if value == "let" {
                let name = self.name();
                assert_eq!(self.lexer.get_next_token().kind, TokenKind::Equal);
                let stmt = Stmt::Let(name, self.add_sub_factor());
                assert_eq!(self.lexer.get_next_token().kind, TokenKind::Semi);
                return stmt;
            }
        }
        // TODO: remove
        println!("Error in valid letStatement at location {}", token.location);
        process::exit(1);
        // panic!("let statement does not start with let");
    }

    // ast node?
    pub fn statement_body(&mut self) -> Body {
        let mut statements = Vec::new();

        loop {
            let peeked = self.lexer.peek(1);
            if let Ok(value) = peeked {
                if value.kind != TokenKind::EndOfFile {
                    statements.push(self.let_statement());
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        return Body { statements };
    }

    // Function/Procedure Setup
}
