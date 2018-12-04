#[derive(Debug, Clone)]
pub enum Stmt {
    Decl(Box<Decl>),
    Simp(Box<Lvalue>, Asnop, Box<Expr>),
    Return(Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Decl {
    Ident(String),
    IdentInit(String, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Lvalue {
    Ident(String),
    Lvalue(Box<Lvalue>),
}

#[derive(Debug, Clone)]
pub enum Expr {
    /// sub expression (<expr>)
    Expr(Box<Expr>),
    /// int const
    Num(u32),
    /// identifier
    Ident(String),
    /// <expr> <binop> <expr>
    Binop(Box<Expr>, Binop, Box<Expr>),
    /// - <expr>
    Minus(Box<Expr>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Binop {
    Add,
    Minus,
    Mul,
    Div,
    Mod,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Asnop {
    /// =
    Equal,
    /// +=
    AddEq,
    /// -=
    MinEq,
    /// *=
    MulEq,
    /// /=
    DivEq,
    /// %=
    ModEq,
}
