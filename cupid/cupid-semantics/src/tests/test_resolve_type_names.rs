#![cfg(test)]
use super::test_utils::*;
use crate::{Error, analyze_scope::AnalyzeScope, resolve_type_names::ResolveTypeNames};

#[test]
fn test_resolve_type_def() -> Result<(), Error> {
    let (mut env, type_def) = (env(), type_def("int"));
    type_def
        .analyze_scope(&mut env)?
        .resolve_type_names(&mut env)?;
    Ok(())
}

#[test]
fn test_is_defined() -> Result<(), Error> {
    let (mut env, type_def) = (env(), type_def("int"));
    type_def
        .analyze_scope(&mut env)?
        .resolve_type_names(&mut env)?;
    assert!(read_ident("int", &mut env).is_some());
    Ok(())
}

#[test]
fn test_is_undefined() -> Result<(), Error> {
    let mut env = env();
    assert!(read_ident("int", &mut env).is_none());
    Ok(())
}