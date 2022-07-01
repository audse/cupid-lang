#![cfg(test)]
use super::test_utils::*;
use crate::{Error, analyze_scope::AnalyzeScope, resolve_type_names::ResolveTypeNames, resolve_names::ResolveNames};

#[test]
fn test_is_defined() -> Result<(), Error> {
    let (mut env, decl) = (env(), decl("x"));
    decl.analyze_scope(&mut env)?
        .resolve_type_names(&mut env)?
        .resolve_names(&mut env)?;
    assert!(read_ident("x", &mut env).is_some());
    Ok(())
}

#[test]
fn test_is_undefined() -> Result<(), Error> {
    let (mut env, ident) = (env(), id("x"));
    assert!(ident.analyze_scope(&mut env)?
        .resolve_type_names(&mut env)?
        .resolve_names(&mut env).is_err());
    Ok(())
}

// #[test]
// fn test_namespaced_id() -> Result<(), Error> {
//     let (mut env, mut type_def, mut ident) = (env(), type_def("int"), namespaced_id("add", "int"));

//     let mut method = decl_val("add", fun(["left", "right"]));
//     method.ident.namespace = Some(Box::new(id("int")));
//     type_def.value.methods.push(method);

//     type_def = type_def.analyze_scope(&mut env)?;
//     ident = ident.analyze_scope(&mut env)?;
//     type_def = type_def.resolve_type_names(&mut env)?;
//     ident = ident.resolve_type_names(&mut env)?;
//     type_def.resolve_names(&mut env)?;
//     ident.resolve_names(&mut env)?;
    
//     Ok(())
// }