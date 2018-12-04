// for test reasons, exported symbols might not be used, suppress warnings
#![allow(dead_code)]
#![allow(unused_imports)]

lalrpop_mod!(pub l1, "/parser/l1.rs");

#[test]
fn test_expr_num() {
    assert!(l1::ExprParser::new().parse("1").is_ok());
}
#[test]
fn test_expr_simple_binop() {
    let expr = l1::ExprParser::new().parse("1 + 1").unwrap();
    assert_eq!(format!("{:?}", expr), "Binop(Num(1), Add, Num(1))");
}

#[test]
fn test_expr_two_prec() {
    // with two precedences
    let expr = l1::ExprParser::new().parse("1 + 2 * 3").unwrap();
    assert_eq!(format!("{:?}", expr), "Binop(Num(1), Add, Binop(Num(2), Mul, Num(3)))");
}

#[test]
fn test_expr_paren() {
    // with parentheses
    let expr = l1::ExprParser::new().parse("1*3+(2-1)").unwrap();
    assert_eq!(format!("{:?}", expr), "Binop(Binop(Num(1), Mul, Num(3)), Add, Binop(Num(2), Minus, Num(1)))");
}

#[test]
fn test_expr_paren_change_prec() {
    let expr = l1::ExprParser::new().parse("1*(3+2)").unwrap();
    assert_eq!(format!("{:?}", expr), "Binop(Num(1), Mul, Binop(Num(3), Add, Num(2)))");
}

#[test]
fn test_expr_ident() {
    let expr = l1::ExprParser::new().parse("a").unwrap();
    assert_eq!(format!("{:?}", expr), "Ident(\"a\")");
    let expr = l1::ExprParser::new().parse("3 % _1").unwrap();
    assert_eq!(format!("{:?}", expr), "Binop(Num(3), Mod, Ident(\"_1\"))");
}

#[test]
fn test_decl() {
    assert!(l1::ProgramParser::new().parse("int main() {}").unwrap().len() == 0);
    assert!(l1::ProgramParser::new().parse("int main() { int a; }").is_ok());
    assert!(l1::ProgramParser::new().parse("int main() { int a = 1; }").is_ok());
    assert!(l1::ProgramParser::new().parse("int main() { int a = 2 * 3 + 1 * 5; }").is_ok());
}
