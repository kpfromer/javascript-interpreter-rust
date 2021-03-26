extern crate unicode_segmentation;

use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

mod astold;
mod callstack;
mod errors;
mod lexer;
mod parser;
mod result_visitor;
mod runner;
mod visitor;

use astold::*;
use callstack::*;
use parser::*;
use runner::*;
use std::fs;
use visitor::*;

// testing
use insta::assert_debug_snapshot;
fn main() {
    let runner = PascalRunner {
        file: String::from("test.txt"),
    };
    let callstack = runner.run();
    println!("{}", callstack)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpreter_let_stmt() {
        let contents = String::from("let a = 20; let b = 20 * 3;");
        let file_content =
            UnicodeSegmentation::graphemes(&contents[..], true).collect::<Vec<&str>>();

        let lexer = lexer::Lexer {
            file: file_content,
            pos: 0,
        };
        let mut parser = Parser { lexer };

        let mut callstack = ActivationRecord {
            name: String::from("global"),
            level: 0,
            parent: None,
            records: HashMap::new(),
        };
        callstack.set(String::from("a"), ActivationRecordValue::IntValue(10));
        let mut interpreter = Interpreter {
            varName: None,
            callstack,
        };
        interpreter.visit_stmt(&parser.let_statement());
        interpreter.visit_stmt(&parser.let_statement());

        callstack = interpreter.to_callstack();

        assert_debug_snapshot!(callstack);
    }
}
