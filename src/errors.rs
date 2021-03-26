use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct SyntaxError {}

impl Error for SyntaxError {}

impl fmt::Display for SyntaxError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Syntax Error")
  }
}
