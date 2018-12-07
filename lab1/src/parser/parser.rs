// for test reasons, exported symbols might not be used, suppress warnings
#![allow(dead_code)]
#![allow(unused_imports)]

use super::error::Result;
use super::ast::*;
use std::collections::HashSet;

lalrpop_mod!(pub l1, "/parser/l1.rs");

#[test]
fn test_expr_num() {
    assert!(l1::ExprParser::new().parse("1").is_ok());
}
#[test]
fn test_expr_simple_binop() {
    let expr = l1::ExprParser::new().parse("1 + 1").unwrap();
    assert_eq!(format!("{:?}", expr), "Expr(Binop(Expr(Num(1)), Binop(Add), Expr(Num(1))))");
}

#[test]
fn test_expr_two_prec() {
    // with two precedences
    let expr = l1::ExprParser::new().parse("1 + 2 * 3").unwrap();
    assert_eq!(format!("{:?}", expr), "Expr(Binop(Expr(Num(1)), Binop(Add), Expr(Binop(Expr(Num(2)), Binop(Mul), Expr(Num(3))))))");
}

#[test]
fn test_expr_paren() {
    // with parentheses
    let expr = l1::ExprParser::new().parse("1*3+(2-1)").unwrap();
    assert_eq!(format!("{:?}", expr), "Expr(Binop(Expr(Binop(Expr(Num(1)), Binop(Mul), Expr(Num(3)))), Binop(Add), Expr(Binop(Expr(Num(2)), Binop(Minus), Expr(Num(1))))))");
}

#[test]
fn test_expr_paren_change_prec() {
    let expr = l1::ExprParser::new().parse("1*(3+2)").unwrap();
    assert_eq!(format!("{:?}", expr), "Expr(Binop(Expr(Num(1)), Binop(Mul), Expr(Binop(Expr(Num(3)), Binop(Add), Expr(Num(2))))))");
}

#[test]
fn test_expr_unary() {
    let expr = l1::ExprParser::new().parse("-(1 * 2 + 3)").unwrap();
    assert_eq!(format!("{:?}", expr), "Expr(Minus(Expr(Binop(Expr(Binop(Expr(Num(1)), Binop(Mul), Expr(Num(2)))), Binop(Add), Expr(Num(3))))))");
}

#[test]
fn test_expr_ident() {
    let expr = l1::ExprParser::new().parse("a").unwrap();
    assert_eq!(format!("{:?}", expr), "Expr(Ident(\"a\"))");
    let expr = l1::ExprParser::new().parse("3 % _1").unwrap();
    assert_eq!(format!("{:?}", expr), "Expr(Binop(Expr(Num(3)), Binop(Mod), Expr(Ident(\"_1\"))))");
}

#[test]
fn test_decl() {
    assert!(l1::ProgramParser::new().parse("int main() {}").unwrap().len() == 0);
    assert!(l1::ProgramParser::new().parse("int main() { int a; }").is_ok());
    assert!(l1::ProgramParser::new().parse("int main() { int a = 1; }").is_ok());
    assert!(l1::ProgramParser::new().parse("int main() { int a = 2 * 3 + 1 * 5; }").is_ok());
}

pub struct Parser;

impl Default for Parser {
    fn default() -> Self {
        Self {}
    }
}

impl Parser {
    pub fn new() -> Self {
        Parser{}
    }

    pub fn parse(s: &str) -> Result<Vec<Box<AstNode>>> {
        l1::ProgramParser::new().parse(s).map_err(|e| e.into())
    }

    pub fn check(ast: Vec<Box<AstNode>>) -> Result<Vec<Box<AstNode>>> {
        let mut symbol_table = HashSet::new();
        for node in ast {
            match *node {
                // when declare, insert
                AstNode::Stmt(Stmt::Decl(decl)) => {
                    match *decl {
                        AstNode::Decl(Decl::Ident(id)) => {
                            symbol_table.insert(id);
                        },
                        AstNode::Decl(Decl::IdentInit(id, _)) => {
                            symbol_table.insert(id);
                        }
                        _ => {}
                    }
                }

                // TODO when use, check
                _ => {}
            }
        }

        println!("{:?}", symbol_table);
        unimplemented!()
    }
}
