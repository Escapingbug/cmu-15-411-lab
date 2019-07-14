//! Semantic analyzer

use crate::types::Ty;
use crate::ast::*;
use crate::symbol_tab::SymbolTab;
use std::fmt;

const RESERVED_KEYWORDS: [&'static str; 20] = [
    "struct",
    "typedef",
    "if",
    "else",
    "while",
    "for",
    "continue",
    "break",
    "return",
    "assert",
    "true",
    "false",
    "NULL",
    "alloc",
    "alloc_array",
    "int",
    "bool",
    "void",
    "char",
    "string",
];

#[derive(Debug)]
pub enum SemanticError {
    /// used reserved keywords
    UsedReservedKeywords(String),
    /// symbol(identifier) is used but not previously defined
    UndeclaredSymbol(String),
    /// symbol is already declared
    RedeclareSymbol(String),
}

impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SemanticError::UsedReservedKeywords(k) => write!(f, "used reserved keywords {}", k),
            SemanticError::UndeclaredSymbol(s) => write!(f, "symbol {} is used but not defined", s),
            SemanticError::RedeclareSymbol(s) => write!(f, "symbol {} is already declared", s),
        }
    }
}

impl std::error::Error for SemanticError {}


type Result = std::result::Result<(), SemanticError>;

fn check_reserved(s: &str) -> Result {
    for reserved in RESERVED_KEYWORDS.iter() {
        if s == *reserved {
            return Err(SemanticError::UndeclaredSymbol(s.to_string()));
        }
    }

    Ok(())
}

fn check_prog<'a, 'b>(prog: &'a mut Prog, symbol_tab: &'b mut SymbolTab<'a>) -> Result {
    for stmt in prog.stmts.iter_mut() {
        check(stmt, symbol_tab)?;
    }

    Ok(())
}

fn check_stmt<'a, 'b>(stmt: &'a mut Stmt, symbol_tab: &'b mut SymbolTab<'a>) -> Result {
    match stmt {
        Stmt::Decl(decl) => {
            check(decl, symbol_tab)?;
        },
        Stmt::Simp(l, _, r) => {
            check(l, symbol_tab)?;
            check(r, symbol_tab)?;
        },
        Stmt::Return(e) => {
            check(e, symbol_tab)?;
        }
    }

    Ok(())
}

fn check_decl<'a, 'b>(decl: &'a mut Decl, symbol_tab: &'b mut SymbolTab<'a>) -> Result {
    match decl {
        Decl::Ident(ref s) => {
            check_reserved(s)?;
            if let Some(_) = symbol_tab.lookup(&s) {
                return Err(SemanticError::RedeclareSymbol(s.clone()));
            }
            symbol_tab.insert(s, Ty::Int);
        }
        Decl::IdentInit(ref s, e) => {
            check_reserved(s)?;
            if let Some(_) = symbol_tab.lookup(&s) {
                return Err(SemanticError::RedeclareSymbol(s.clone()))?;
            }
            // check right part
            check(e, symbol_tab)?;
            symbol_tab.insert(s, Ty::Int);
        }
    }

    Ok(())
}

fn check_lvalue<'a, 'b>(lvalue: &'a mut Lvalue, symbol_tab: &'b mut SymbolTab<'a>) -> Result {
    match lvalue {
        Lvalue::Ident(s) => {
            if let Some(_) = symbol_tab.lookup(&s) {
                Ok(())
            } else {
                Err(SemanticError::UndeclaredSymbol(s.clone()))
            }
        },
        Lvalue::Lvalue(l) => check(l, symbol_tab),
    }
}

fn check_expr<'a>(expr: &'a mut Expr, symbol_tab: &mut SymbolTab<'a>) -> Result {
    match expr {
        Expr::Expr(e) => {
            check(e, symbol_tab)?;
        },
        Expr::Ident(s) => {
            match symbol_tab.lookup(&s) {
                None => return Err(SemanticError::UndeclaredSymbol(s.clone())),
                _ => {}
            }
        }
        Expr::Binop(l, _, r) => {
            check(l, symbol_tab)?;
            check(r, symbol_tab)?;
        }
        Expr::Minus(e) => {
            check(e, symbol_tab)?;
        }
        // number has nothing to check
        Expr::Num(_) => {},
    }

    Ok(())
}


/// recusively semantic check on ast
pub fn check<'a>(ast: &'a mut Box<AnnoAstNode>, symbol_tab: &mut SymbolTab<'a>) -> Result {

    use AstNode::*;
    match *ast.node {
        Prog(ref mut p) => check_prog(p, symbol_tab),
        Stmt(ref mut s) => check_stmt(s, symbol_tab),
        Decl(ref mut d) => check_decl(d, symbol_tab),
        Lvalue(ref mut l) => check_lvalue(l, symbol_tab),
        Expr(ref mut e) => check_expr(e, symbol_tab),
        _ => Ok(())
    }
}
