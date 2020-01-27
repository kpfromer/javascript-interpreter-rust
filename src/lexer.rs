use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum TokenKind {
  Program,
  Begin,
  End,
  Dot,
  Semi,
  Colon,
  Comma,
  Var,

  Assign,

  Equal,
  NotEqual,
  And,
  Or,
  Less,
  LessEqual,
  Greater,
  GreaterEqual,
  Not,
  Plus,
  Minus,
  Multiply,
  Divide,
  LeftParen,
  RightParen,

  Id(String),
  True,
  False,
  IntegerConstant(u32),

  Integer,
  Boolean,
  Function,
  Procedure,

  EndOfFile,
}

// enum TokenValue {
//   Integer(u32),
//   None,
// }

// enum Token {}

#[derive(Debug)]
pub struct Token {
  pub kind: TokenKind,
  pub location: usize,
}

pub struct Lexer<'a> {
  pub file: Vec<&'a str>,
  pub pos: usize,
}

// impl std::fmt::Display for Lexer {
//   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//     write!(f, "(value a: {}, value b: {})", self.a, self.b)
//   }
// }

impl<'a> Lexer<'a> {
  fn advance(&mut self) -> bool {
    self.pos += 1;
    return true;
  }
  fn skipWhitespace(&mut self) {
    while *self.file.get(self.pos).expect("Expect value.") == " " {
      self.pos += 1;
    }
  }
  fn skipComment(&mut self) {
    while *self.file.get(self.pos).expect("Expect value.") != "}" {
      self.pos += 1;
    }
    self.pos += 1;
  }
  pub fn _id(&mut self) -> Token {
    let location = self.pos;

    let mut result = String::new();

    while self.pos < self.file.len()
      && self
        .file
        .get(self.pos)
        .expect("An id")
        .chars()
        .all(char::is_alphanumeric)
    {
      result.push_str(self.file.get(self.pos).expect("An id"));
      self.pos += 1;
    }

    // TODO: reserved keywords

    return Token {
      kind: TokenKind::Id(result),
      location,
    };
  }

  pub fn getNextToken(&mut self) -> Token {
    loop {
      let value: &'a str;

      if let Some(val) = self.file.get(self.pos) {
        value = val;
      } else {
        break Token {
          kind: TokenKind::EndOfFile,
          location: self.pos,
        };
      }

      if value == " " {
        self.skipWhitespace();
        continue;
      } else if value == "{" {
        self.skipComment();
        continue;
      } else if value == "\n" {
        self.pos += 1;
        continue;
      }

      if let Ok(_) = value.parse::<u32>() {
        return self.number();
      }

      if value.chars().all(char::is_alphabetic) {
        return self._id();
      }

      let location = self.pos;

      if value == "+" {
        self.pos += 1;
        return Token {
          location,
          kind: TokenKind::Plus,
        };
      } else if value == "-" {
        self.pos += 1;
        return Token {
          location,
          kind: TokenKind::Minus,
        };
      } else if value == "*" {
        self.pos += 1;
        return Token {
          location,
          kind: TokenKind::Multiply,
        };
      } else if value == "/" {
        self.pos += 1;
        return Token {
          location,
          kind: TokenKind::Divide,
        };
      }

      println!("Value: {}", value);
      panic!();
    }
  }
  pub fn peek(&mut self, number: &i32) -> Result<Token, &'static str> {
    let oldPos = self.pos;
    let mut token: Result<Token, &'static str> = Err("No token.");
    for _ in 0..*number {
      token = Ok(self.getNextToken());
    }
    self.pos = oldPos;
    return token;
  }
  fn number(&mut self) -> Token {
    let location = self.pos;
    let mut numbVec: Vec<u32> = Vec::new();
    while self.pos < self.file.len() {
      if let Ok(number) = self.file.get(self.pos).expect("A value").parse::<u32>() {
        numbVec.push(number);
        self.pos += 1;
      } else {
        break;
      }
    }
    let mut number: u32 = 0;
    for (i, numb) in numbVec.iter().enumerate() {
      number += numb * 10u32.pow((numbVec.len() - i - 1) as u32);
    }
    return Token {
      kind: TokenKind::IntegerConstant(number),
      location,
    };
  }
}
