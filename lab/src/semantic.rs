//! Semantic analyzer

use crate::ast::*;
use std::fmt;
use std::collections::HashSet;

#[derive(Debug)]
pub enum SemanticError {
    /// used reserved keywords
    UsedReservedKeywords(String),
    /// symbol(identifier) is used but not previously defined
    UndeclaredSymbol(String),
}

impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SemanticError::UsedReservedKeywords(k) => write!(f, "used reserved keywords {}", k),
            SemanticError::UndeclaredSymbol(s) => write!(f, "symbol {} is used but not defined", s),
        }
    }
}

impl std::error::Error for SemanticError {}


type Result = std::result::Result<(), SemanticError>;

/// recusively semantic check on ast
pub fn check(ast: &mut Box<AnnoAstNode>) -> Result {

    unimplemented!()

        /*
    use AstNode::*;
    match *ast {
        Prog(p) => {
            let stmts = p.stmts;
            check(stmts)
        },
        Stmt(s) => {
            match s {
                Stmt::Decl()
            }
        }
    }
    */
}
