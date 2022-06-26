#![cfg(test)]
use cupid_ast::expr::Expr;
use cupid_env::environment::Env;
use cupid_util::Bx;

use super::test_utils::*;
use crate::{Error, analyze_scope::AnalyzeScope, resolve_type_names::ResolveTypeNames, resolve_names::ResolveNames, infer_types::InferTypes, check_types::CheckTypes, check_flow::CheckFlow, lint::Lint};

macro_rules! do_passes {
    ($env:ident => $($x:ident),*) => {
        $( $x = $x.analyze_scope(&mut $env)?; )*
        $( $x = $x.resolve_type_names(&mut $env)?; )*
        $( $x = $x.resolve_names(&mut $env)?; )*
        $( $x = $x.infer_types(&mut $env)?; )*
        $( $x = $x.check_types(&mut $env)?; )*
        $( $x = $x.check_flow(&mut $env)?; )*
        $( $x = $x.lint(&mut $env)?; )*
    };
}

#[test]
fn test_unused_variable() -> Result<(), Error> {
    let (mut env, mut type_def, mut decl, mut int) = (env(), type_def("int"), decl("x"), int(1));
    type_def.attr.source = write_source(&mut env);
    decl.attr.source = write_source(&mut env);
    int.attr.source = write_source(&mut env);
    decl.value = Expr::from(int).bx();
    let catch = || -> Result<(), Error> {
        do_passes![env => type_def, decl];
        Ok(())
    };
    assert!(catch().is_err());
    Ok(())
}

#[test]
fn test_used_variable() -> Result<(), Error> {
    let (mut env, mut type_def, mut decl, mut int, mut ident) = (env(), type_def("int"), decl("x"), int(1), id("x"));
    type_def.attr.source = write_source(&mut env);
    decl.attr.source = write_source(&mut env);
    int.attr.source = write_source(&mut env);
    decl.value = Expr::from(int).bx();
    ident.attr.source = write_source(&mut env);
    do_passes![env => type_def, decl, ident];
    Ok(())
}