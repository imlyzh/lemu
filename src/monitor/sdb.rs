

enum SDB {
    H,
    C,
    Q,
    Si(usize),
    Info(SUBCMD),
    X(usize, Expr),
    W(Expr),
    D(usize),
}

enum SUBCMD {
    Reg,
    Mem,
    Csr,
}

enum Expr {
    Reg(String),
    Num(usize),
    Deref(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Eq(Box<Expr>, Box<Expr>),
    Ne(Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
}