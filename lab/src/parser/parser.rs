// for test reasons, exported symbols might not be used, suppress warnings
#![allow(dead_code)]
#![allow(unused_imports)]

use crate::ast::*;
use std::collections::HashSet;

lalrpop_mod!(pub l1, "/parser/l1.rs");

pub type Parser = l1::ProgramParser;

#[test]
fn test_expr_num() {
    assert!(l1::ExprParser::new().parse("1").is_ok());
}

#[test]
fn test_decl() {
    let res = l1::ProgramParser::new().parse("int main() {}").unwrap();
    if let AstNode::Prog(p) = *res.node {
        assert_eq!(p.stmts.len(), 0);
    } else {
        panic!("expected Prog parsed, but parsed otherwise");
    }
    assert!(l1::ProgramParser::new().parse("int main() { int a; }").is_ok());
    assert!(l1::ProgramParser::new().parse("int main() { int a = 1; }").is_ok());
    assert!(l1::ProgramParser::new().parse("int main() { int a = 2 * 3 + 1 * 5; }").is_ok());
}
