#![allow(unused)]
#![cfg(test)]

use cupid_util::*;
use super::*;


#[test]
fn test_decl() -> TestResult {
    pass(decl("x"), &mut env())?;
    Ok(())
}

#[test]
fn test_ident() -> TestResult {
    let mut env = env();
    ident("x").resolve_packages(&mut env)?
        .analyze_type_scope(&mut env)?
        .resolve_type_names(&mut env)?
        .analyze_scope(&mut env)?;
    Ok(())
}

#[test]
fn test_undefined_ident() -> TestResult {
    let ident = pass(ident("x"), &mut env());
    assert!(ident.is(ERR_NOT_FOUND));
    Ok(())
}

#[test]
fn test_already_declared() -> TestResult {
    let decl = pass_all([decl("a"), decl("a")], &mut env());
    assert!(decl.is(ERR_ALREADY_DEFINED));
    Ok(())
}

#[test]
fn test_outside_scope() -> TestResult {
    let mut env = env();
    env.scope.add_closure(env::Context::Block);
    env.inside_closure(1, |env| {
        let decl = decl_val("x", int(1));
        let decl = pass(decl, env)?;
        Ok(())
    })?;
    env.inside_closure(0, |env| {
        assert!(env.read::<Address>(&Query::select(ident("x"))).is(ERR_NOT_IN_SCOPE));
        Ok(())
    })
}

#[test]
fn test_undefined_typ() -> TestResult {
    let mut env = env();
    let decl = typed_decl("x", "newtype");
    assert!(pass(decl, &mut env).is(ERR_NOT_FOUND));
    Ok(())
}

#[test]
fn test_decl_none_typ() -> TestResult {
    let mut env = env();
    add_typ(&mut env, int_typ());

    let mut decl = decl_val("x", int(1));
    let address = env.insert::<pre_analysis::Expr>(Query::insert().write_expr(decl.clone()));
    decl.attr.address = address;

    let mut decl = pass(decl, &mut env)?;
    let decl_type: &Type = env.read::<env::SymbolType>(&Query::select(decl.address()))?.try_into()?;
    assert_eq!(decl_type.name.name.to_string(), "none".to_string());
    Ok(())
}