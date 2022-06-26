#![cfg(test)]
use cupid_ast::expr::Expr;
use cupid_env::environment::Env;

use super::test_utils::*;
use crate::{Error, analyze_scope::AnalyzeScope, resolve_type_names::ResolveTypeNames, resolve_names::ResolveNames, infer_types::InferTypes};

fn passes<T: AnalyzeScope + ResolveTypeNames + ResolveNames + InferTypes>(expr: T, env: &mut Env) -> Result<T, Error> {
    expr.analyze_scope(env)?
        .resolve_type_names(env)?
        .resolve_names(env)?
        .infer_types(env)
}

macro_rules! do_passes {
    ($env:ident => $($x:ident),*) => {
        $( $x = $x.analyze_scope(&mut $env)?; )*
        $( $x = $x.resolve_type_names(&mut $env)?; )*
        $( $x = $x.resolve_names(&mut $env)?; )*
        $( $x = $x.infer_types(&mut $env)?; )*
    };
}

#[test]
fn test_infer_decl() -> Result<(), Error> {
    let (mut env, mut decl) = (env(), decl("x"));
    decl.attr.source = write_source(&mut env);
    decl = passes(decl, &mut env)?;
    assert!(type_is(decl.attr, "none", &mut env));
    Ok(())
}

#[test]
fn test_infer_int() -> Result<(), Error> {
    let (mut env, mut val) = (env(), int(1));
    val.attr.source = write_source(&mut env);
    val = passes(val, &mut env)?;
    assert!(type_is(val.attr, "int", &mut env));
    Ok(())
}

#[test]
fn test_infer_not_int() -> Result<(), Error> {
    let (mut env, mut val) = (env(), int(1));
    val.attr.source = write_source(&mut env);
    val = passes(val, &mut env)?;
    assert!(!type_is(val.attr, "string", &mut env));
    Ok(())
}

#[test]
fn test_infer_typed_decl() -> Result<(), Error> {
    let (mut env, mut int_def, mut decl) = (env(), type_def("int"), typed_decl("x", "int"));
    int_def.attr.source = write_source(&mut env);
    decl.attr.source = write_source(&mut env);
    do_passes![env => int_def, decl];
    assert!(type_is(decl.type_annotation.unwrap().attr, "type", &mut env));
    Ok(())
}