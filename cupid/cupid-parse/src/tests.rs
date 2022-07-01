#![cfg(test)]

use std::rc::Rc;

use cupid_ast::{expr::{Expr, value::{Value, Val}, function_call::FunctionCall}, stmt::{decl::{Mut, Decl}, Stmt, type_def::TypeDef, trait_def::TraitDef}};
use cupid_lex::lexer::Lexer;
use crate::parse::Parser;

fn setup(string: &str) -> Option<Vec<Expr>> {
    let mut lexer = Lexer::new();
    let tokens = lexer.lex(string);
    let mut parser = Parser::new(Rc::new(string.to_string()));
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

fn fun_call(string: &str) -> FunctionCall {
    let fun_call = expr(string);
    if let Expr::FunctionCall(fun_call) = fun_call {
        fun_call 
    } else {
        panic!("expected function call")
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

fn trait_def(string: &str) -> TraitDef {
    let def = expr(string);
    if let Expr::Stmt(Stmt::TraitDef(def)) = def {
        def 
    } else {
        panic!("expected trait definition")
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
fn test_fun_call() {
    let fun = fun_call("add(1, 2)");
    assert!(&*fun.function.name == "add" && fun.args.len() == 2);
    assert!(matches!(fun.args[0], Expr::Value(Value { inner: Val::VInteger(_), ..})))
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

#[test]
fn test_trait_def() {
    let def = trait_def("trait add = [
        add: left : int, right : int => 1 # todo
    ]");
    assert!(def.value.methods[0].ident.name == "add");
}

#[test]
fn test_trait_multiple_methods() {
    let def = trait_def("trait equal (t) = [
        equal: left : t, right : t => false, # todo
        not_equal: left : t, right : t => true, # todo
    ]");
    assert!(def.value.methods.len() == 2);
    assert!(def.value.methods[1].ident.name == "not_equal");
}

#[test]
fn test_fun_decl() {
    let fun = decl("let sq = num : int => 4 # todo");
    assert!(matches!(*fun.value, Expr::Function(_)));
}


#[test]
fn test_fun_decl_no_params() {
    let fun = decl("let x = _ => {}");
    assert!(matches!(*fun.value, Expr::Function(_)));
}

#[test]
fn test_block() {
    let block = expr("{ let x : int = 1 }");
    assert!(matches!(block, Expr::Block(_)));
}