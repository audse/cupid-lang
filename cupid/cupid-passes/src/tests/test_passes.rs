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
