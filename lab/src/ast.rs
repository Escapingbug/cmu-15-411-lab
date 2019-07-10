#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ty {
    Int,
}

#[derive(Debug, Clone)]
pub struct Anno {
    /// Type
    ty: Ty,
}

/// annotated ast node
#[derive(Debug, Clone)]
pub struct AnnoAstNode {
    pub anno: Option<Anno>,
    pub node: Box<AstNode>
}

impl From<AstNode> for AnnoAstNode {
    fn from(node: AstNode) -> Self {
        Self {
            anno: None,
            node: Box::new(node)
        }
    }
}

impl From<Box<AstNode>> for Box<AnnoAstNode> {
    fn from(node: Box<AstNode>) -> Self {
        Box::new(AnnoAstNode {
            anno: None,
            node: node
        })
    }
}

#[derive(Debug, Clone)]
pub enum AstNode {
    Prog(Prog),
    Stmt(Stmt),
    Decl(Decl),
    Lvalue(Lvalue),
    Expr(Expr),
    Binop(Binop),
    Asnop(Asnop),
}

#[derive(Debug, Clone)]
pub struct Prog {
    pub stmts: Vec<Box<AnnoAstNode>>
}

impl From<Prog> for AnnoAstNode {
    fn from(p: Prog) -> Self {
        AstNode::Prog(p).into()
    }
}

#[derive(Debug, Clone)]
pub enum Stmt {
    /// Decl(Decl)
    Decl(Box<AnnoAstNode>),
    /// Simp(Lvalue, Asnop, Expr)
    Simp(Box<AnnoAstNode>, Box<AnnoAstNode>, Box<AnnoAstNode>),
    /// Return(Expr)
    Return(Box<AnnoAstNode>),
}

impl From<Stmt> for AnnoAstNode {
    fn from(s: Stmt) -> Self {
        AstNode::Stmt(s).into() 
    }
}

#[derive(Debug, Clone)]
pub enum Decl {
    Ident(String),
    /// IdentInit(String, Expr)
    IdentInit(String, Box<AnnoAstNode>),
}

impl From<Decl> for AnnoAstNode {
    fn from(d: Decl) -> Self {
        AstNode::Decl(d).into()
    }
}

#[derive(Debug, Clone)]
pub enum Lvalue {
    Ident(String),
    /// "(" Lvalue ")" --> Lvalue(Lvalue)
    Lvalue(Box<AnnoAstNode>),
}

impl From<Lvalue> for AnnoAstNode {
    fn from(l: Lvalue) -> Self {
        AstNode::Lvalue(l).into()
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    /// sub expression (<expr>)  Expr(Expr)
    Expr(Box<AnnoAstNode>),
    /// int const
    Num(u32),
    /// identifier
    Ident(String),
    /// <expr> <binop> <expr>  Binop(Expr, Binop, Expr)
    Binop(Box<AnnoAstNode>, Box<AnnoAstNode>, Box<AnnoAstNode>),
    /// - <expr>  Minus(Expr)
    Minus(Box<AnnoAstNode>),
}

impl From<Expr> for AnnoAstNode {
    fn from(e: Expr) -> Self {
        AstNode::Expr(e).into()
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

impl From<Binop> for AnnoAstNode {
    fn from(op: Binop) -> Self {
        AstNode::Binop(op).into()
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

impl From<Asnop> for AnnoAstNode {
    fn from(op: Asnop) -> Self {
        AstNode::Asnop(op).into()
    }
}
