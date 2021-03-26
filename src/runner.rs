extern crate unicode_segmentation;

use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

use std::fs;

use crate::callstack::*;
use crate::lexer::*;
use crate::parser::*;
use crate::result_visitor::*;

pub trait Runner {
  fn run(&self) -> ActivationRecord<i64>;
}

pub struct PascalRunner {
  pub file: String,
}

impl Runner for PascalRunner {
  fn run(&self) -> ActivationRecord<i64> {
    let callstack = ActivationRecord {
      name: String::from("global"),
      level: 0,
      parent: None,
      records: HashMap::new(),
    };

    let mut interpreter = ResultInterpreter {
      varName: None,
      callstack,
    };

    let contents =
      fs::read_to_string(self.file.as_str()).expect("Something went wrong reading the file");
    let fileContent = UnicodeSegmentation::graphemes(&contents[..], true).collect::<Vec<&str>>();

    let lexer = Lexer {
      file: fileContent,
      pos: 0,
    };

    let mut parser = Parser { lexer };

    // TODO: run statement body

    match interpreter
      .visit_body(&parser.statement_body())
       {
         Err(error) => {
           println!("Syntax Error {}", error);
         },
         _ => {}
       }

    return interpreter.to_callstack();
  }
}

// extern crate unicode_segmentation;

// use std::collections::HashMap;
// use unicode_segmentation::UnicodeSegmentation;

// use std::fs;

// use crate::callstack::*;
// use crate::lexer::*;
// use crate::parser::*;
// use crate::visitor::*;

// pub trait Runner {
//   fn run(&self) -> ActivationRecord<i64>;
// }

// pub struct PascalRunner {
//   pub file: String,
// }

// impl Runner for PascalRunner {
//   fn run(&self) -> ActivationRecord<i64> {
//     let callstack = ActivationRecord {
//       name: String::from("global"),
//       level: 0,
//       parent: None,
//       records: HashMap::new(),
//     };

//     let mut interpreter = Interpreter {
//       varName: None,
//       callstack,
//     };

//     let contents =
//       fs::read_to_string(self.file.as_str()).expect("Something went wrong reading the file");
//     let fileContent = UnicodeSegmentation::graphemes(&contents[..], true).collect::<Vec<&str>>();

//     let lexer = Lexer {
//       file: fileContent,
//       pos: 0,
//     };

//     let mut parser = Parser { lexer };

//     // TODO: run statement body

//     interpreter.visit_body(&parser.statement_body());

//     return interpreter.to_callstack();
//   }
// }
