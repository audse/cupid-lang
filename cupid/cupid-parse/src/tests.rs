#![cfg(test)]

use cupid_ast::{expr::{Expr, value::{Value, Val}}, stmt::{decl::{Mut, Decl}, Stmt, type_def::TypeDef}};
use cupid_lex::lexer::Lexer;
use crate::parse::Parser;

fn setup(string: &str) -> Option<Vec<Expr>> {
    let mut lexer = Lexer::new();
    let tokens = lexer.lex(string);
    let mut parser = Parser::new();
    parser.parse(tokens)
}

fn expr(string: &str) -> Expr {
    let mut expr = setup(string).expect("expected expression");
    expr.pop().expect("expected expression")
}

fn decl(string: &str) -> Decl {
    let decl = expr(string);
    if let Expr::Stmt(Stmt::Decl(decl)) = decl {
        decl 
    } else {
        panic!("expected declaration")
    }
}

fn type_def(string: &str) -> TypeDef {
    let def = expr(string);
    if let Expr::Stmt(Stmt::TypeDef(def)) = def {
        def 
    } else {
        panic!("expected type definition")
    }
}

fn val(string: &str) -> Value {
    let val = expr(string);
    if let Expr::Value(val) = val {
        val 
    } else {
        panic!("expected value")
    }
}

#[test]
fn test_number() {
    let val = val("1");
    assert!(matches!(val, Value { inner: Val::VInteger(1, ..), .. }));
}

#[test]
fn test_decimal() {
    let val = val("1.5");
    assert!(matches!(val, Value { inner: Val::VDecimal(1, 5, ..), .. }));
}

#[test]
fn test_string() {
    let val = val("'abc'");
    if let Value { inner: Val::VString(s, ..), .. } = val {
        assert!(&*s == "abc");
    } else {
        panic!("expected string")
    }
}

#[test]
fn test_bool_true() {
    let val = val("true");
    assert!(matches!(val, Value { inner: Val::VBoolean(true, ..), .. }))
}

#[test]
fn test_bool_false() {
    let val = val("false");
    assert!(matches!(val, Value { inner: Val::VBoolean(false, ..), .. }))
}

#[test]
fn test_none() {
    let val = val("none");
    assert!(matches!(val, Value { inner: Val::VNone, .. }))
}

#[test]
fn test_decl_immutable() {
    let decl = decl("let x = false");
    assert!(decl.mutable == Mut::Immutable);
}

#[test]
fn test_decl_mutable() {
    let decl = decl("let mut x = false");
    assert!(decl.mutable == Mut::Mutable);
}

#[test]
fn test_decl_typed() {
    let decl = decl("let mut x : int = 1");
    let typ = decl.type_annotation.expect("expected type");
    assert!(&*typ.name == "int")
}

#[test]
fn test_decl_typed_single_generic() {
    let decl = decl("let mut x : array (int) = none");
    let typ = decl.type_annotation.expect("expected type");
    assert!(&*typ.name == "array" && &*typ.generics[0].name == "int")
}

#[test]
fn test_decl_typed_generics() {
    let decl = decl("let mut x : map (int, int) = none");
    let typ = decl.type_annotation.expect("expected type");
    assert!(
        &*typ.name == "map" 
            && &*typ.generics[0].name == "int" 
            && &*typ.generics[1].name == "int"
    )
}

#[test]
fn test_type_def() {
    let def = type_def("type int = []");
    assert!(&*def.ident.name == "int" && def.value.fields.len() == 0);
}

#[test]
fn test_type_def_field() {
    let def = type_def("type array = [int]");
    assert!(def.value.fields.len() == 1 && matches!(*def.value.fields[0].value, Expr::Empty));
}

#[test]
fn test_type_def_named_field() {
    let def = type_def("type map = [key : int, val : int]");
    assert!(def.value.fields.len() == 2 && matches!(*def.value.fields[0].value, Expr::Ident(_)));
}

#[test]
fn test_type_def_sum() {
    let def = type_def("sum bool = [true, false]");
    assert!(def.value.fields.len() == 2);
}

#[test]
fn test_multiple_expr() {
    let exprs = setup("type int = [] let x : int = 1");
    assert!(exprs.unwrap().len() == 2);
}