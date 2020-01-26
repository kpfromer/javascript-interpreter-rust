extern crate unicode_segmentation;

mod ast;
mod lexer;
mod parser;
mod visitor;

use ast::*;
use parser::*;
use std::fs;
use visitor::*;

use unicode_segmentation::UnicodeSegmentation;

fn main() {
    println!("Hello, world!");
    let mut interpreter = Interpreter;
    let one = Expr::IntLit(1);
    let two = Expr::IntLit(2);
    let add = Expr::Add(Box::new(one), Box::new(two));
    println!("Int: {}", interpreter.visit_expr(&add));

    let contents = fs::read_to_string("test.txt").expect("Something went wrong reading the file");
    println!("Contents: {}", contents);

    let fileContent = UnicodeSegmentation::graphemes(&contents[..], true).collect::<Vec<&str>>();

    let mut lexer = lexer::Lexer {
        file: fileContent,
        pos: 0,
    };

    let mut parser = Parser { lexer };

    // let num1 = Box::new(parser.number());
    // let num2 = Box::new(parser.number());

    // println!("Add {}", interpreter.visit_expr(&Expr::Add(num1, num2)));
    println!("Add {}", interpreter.visit_expr(&parser.addSubFactor()));

    // println!("Read value {:?}", lexer.peek(&1).expect("A token exists."));

    // loop {
    //     let token = lexer.getNextToken();
    //     println!("Read value {:?}", token);
    //     if let lexer::TokenKind::EndOfFile = token.kind {
    //         break;
    //     }
    // }
}
