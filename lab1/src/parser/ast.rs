#[derive(Debug, Clone)]
pub enum AstNode {
    Stmt(Stmt),
    Decl(Decl),
    Lvalue(Lvalue),
    Expr(Expr),
    Binop(Binop),
    Asnop(Asnop),
}

#[derive(Debug, Clone)]
pub enum Stmt {
    /// Decl(Decl)
    Decl(Box<AstNode>),
    /// Simp(Lvalue, Asnop, Expr)
    Simp(Box<AstNode>, Box<AstNode>, Box<AstNode>),
    /// Return(Expr)
    Return(Box<AstNode>),
}

impl From<Stmt> for AstNode {
    fn from(s: Stmt) -> Self {
        AstNode::Stmt(s) 
    }
}

#[derive(Debug, Clone)]
pub enum Decl {
    Ident(String),
    /// IdentInit(String, Expr)
    IdentInit(String, Box<AstNode>),
}

impl From<Decl> for AstNode {
    fn from(d: Decl) -> Self {
        AstNode::Decl(d)
    }
}

#[derive(Debug, Clone)]
pub enum Lvalue {
    Ident(String),
    /// "(" Lvalue ")" --> Lvalue(Lvalue)
    Lvalue(Box<AstNode>),
}

impl From<Lvalue> for AstNode {
    fn from(l: Lvalue) -> Self {
        AstNode::Lvalue(l)
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    /// sub expression (<expr>)  Expr(Expr)
    Expr(Box<AstNode>),
    /// int const
    Num(u32),
    /// identifier
    Ident(String),
    /// <expr> <binop> <expr>  Binop(Expr, Binop, Expr)
    Binop(Box<AstNode>, Box<AstNode>, Box<AstNode>),
    /// - <expr>  Minus(Expr)
    Minus(Box<AstNode>),
}

impl From<Expr> for AstNode {
    fn from(e: Expr) -> Self {
        AstNode::Expr(e)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Binop {
    Add,
    Minus,
    Mul,
    Div,
    Mod,
}

impl From<Binop> for AstNode {
    fn from(op: Binop) -> Self {
        AstNode::Binop(op)
    }
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

impl From<Asnop> for AstNode {
    fn from(op: Asnop) -> Self {
        AstNode::Asnop(op)
    }
}
