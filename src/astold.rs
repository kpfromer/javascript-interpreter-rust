// TODO: convert to function
pub struct Body {
  pub statements: Vec<Stmt>,
}

pub enum Stmt {
  Expr(Expr),
  Let(Name, Expr),
}

pub struct Name {
  pub value: String,
}

pub enum Expr {
  IntLit(u32),
  BoolLit(bool),
  Variable(String),
  UnaryAdd(Box<Expr>),
  UnarySub(Box<Expr>),
  Add(Box<Expr>, Box<Expr>),
  Sub(Box<Expr>, Box<Expr>),
  Multiply(Box<Expr>, Box<Expr>),
  Divide(Box<Expr>, Box<Expr>),
}
