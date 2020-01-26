pub enum Stmt {
  Expr(Expr),
  Let(Name, Expr),
}

pub struct Name {
  value: String,
}

pub enum Expr {
  IntLit(u32),
  Add(Box<Expr>, Box<Expr>),
  Sub(Box<Expr>, Box<Expr>),
  Multiply(Box<Expr>, Box<Expr>),
  Divide(Box<Expr>, Box<Expr>),
}
