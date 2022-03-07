

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SDB {
    H,
    C,
    Q,
    Si(usize),
    Info(SUBCMD),
    X(usize, Expr),
    W(Expr),
    D(usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SUBCMD {
    Reg,
    Mem,
    Csr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Reg(String),
    Num(u64),
    Deref(Box<Expr>),

    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),

    Leq(Box<Expr>, Box<Expr>),
    Lt(Box<Expr>, Box<Expr>),
    Geq(Box<Expr>, Box<Expr>),
    Gt(Box<Expr>, Box<Expr>),
    Eq(Box<Expr>, Box<Expr>),
    Ne(Box<Expr>, Box<Expr>),

    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
}