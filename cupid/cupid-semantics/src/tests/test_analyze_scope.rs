#![cfg(test)]
use super::test_utils::*;

use crate::{Error, analyze_scope::AnalyzeScope};

macro_rules! has_scope {
    ($node:expr, $scope:expr) => { assert!($node.attr.scope == $scope) };
}

#[test]
fn test_decl() -> Result<(), Error> {
    let (mut env, mut decl) = (env(), decl("x"));
    has_scope!(decl.ident, 0);
    decl = decl.analyze_scope(&mut env)?;
    has_scope!(decl.ident, 0);
    Ok(())
}

#[test]
fn test_fun() -> Result<(), Error> {
    let (mut env, mut fun) = (env(), fun(["x", "y"]));
    has_scope!(fun, 0);
    fun = fun.analyze_scope(&mut env)?;
    has_scope!(fun, 1);
    Ok(())
}

#[test]
fn test_type() -> Result<(), Error> {
    let (mut env, mut typ) = (env(), typ("int"));
    has_scope!(typ, 0);
    typ = typ.analyze_scope(&mut env)?;
    has_scope!(typ, 1);
    Ok(())
}

#[test]
fn test_type_field() -> Result<(), Error> {
    let (mut env, mut typ) = (env(), typ("array"));
    typ.fields.push(decl("element"));
    has_scope!(typ.fields[0], 0);
    typ = typ.analyze_scope(&mut env)?;
    has_scope!(typ.fields[0], 1);
    Ok(())
}

#[test]
fn test_trait() -> Result<(), Error> {
    let (mut env, mut t) = (env(), traits("add"));
    has_scope!(t, 0);
    t = t.analyze_scope(&mut env)?;
    has_scope!(t, 1);
    Ok(())
}

#[test]
fn test_trait_method() -> Result<(), Error> {
    let (mut env, mut t) = (env(), traits("add"));
    t.methods.push(fun_decl("add", fun(["left", "right"])));
    has_scope!(t.methods[0], 0);
    assert!(t.methods[0].value.attr().scope == 0);
    t = t.analyze_scope(&mut env)?;
    has_scope!(t.methods[0], 1);
    assert!(t.methods[0].value.attr().scope == 2);
    Ok(())
}