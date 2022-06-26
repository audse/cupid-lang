#![cfg(test)]
#![allow(unused)]
use cupid_ast::expr::{Expr, block::Block};
use cupid_util::Bx;

use crate::{Error, analyze_scope::AnalyzeScope, resolve_type_names::ResolveTypeNames, resolve_names::ResolveNames, infer_types::InferTypes, check_types::CheckTypes};
use super::test_utils::*;

macro_rules! do_passes {
    ($env:ident => $($x:ident),*) => {
        $( $x = $x.analyze_scope(&mut $env)?; )*
        $( $x = $x.resolve_type_names(&mut $env)?; )*
        $( $x = $x.resolve_names(&mut $env)?; )*
        $( $x = $x.infer_types(&mut $env)?; )*
        $( $x = $x.check_types(&mut $env)?; )*
    };
}

#[test]
fn test_decl_types_ok() -> Result<(), Error> {
    let (mut env, mut type_def, mut decl) = (env(), type_def("int"), typed_decl("x", "int"));
    type_def.attr.source = write_source(&mut env);
    let mut i = int(1);
    i.attr.source = write_source(&mut env);
    decl.value = Expr::from(i).bx();
    do_passes![env => type_def, decl];
    Ok(())
}

#[test]
fn test_decl_types_err() -> Result<(), Error> {
    let (mut env, mut type_def, mut decl) = (env(), type_def("string"), typed_decl("x", "string"));
    type_def.attr.source = write_source(&mut env);
    let mut i = int(1);
    i.attr.source = write_source(&mut env);
    decl.value = Expr::from(i).bx();
    let catch = || -> Result<(), Error> {
        do_passes![env => type_def, decl];
        Ok(())
    };
    assert!(catch().is_err());
    Ok(())
}

#[test]
fn test_fun_type_match() -> Result<(), Error> {
    let (mut env, mut type_def, mut fun) = (env(), type_def("int"), fun(["x", "y"]));
    type_def.attr.source = write_source(&mut env);
    fun.return_type_annotation = Some(id("int"));
    fun.attr.source = write_source(&mut env);
    fun.body = Block { expressions: vec![int(1).into()], ..Default::default() };
    fun.body.attr.source = write_source(&mut env);
    do_passes![env => type_def, fun];
    Ok(())
}

#[test]
fn test_fun_type_mismatch() -> Result<(), Error> {
    let (mut env, mut type_def, mut fun) = (env(), type_def("string"), fun(["x", "y"]));
    type_def.attr.source = write_source(&mut env);
    fun.return_type_annotation = Some(id("string"));
    fun.attr.source = write_source(&mut env);
    fun.body = Block { expressions: vec![int(1).into()], ..Default::default() };
    fun.body.attr.source = write_source(&mut env);
    let catch = || -> Result<(), Error> {
        do_passes![env => type_def, fun];
        Ok(())
    };
    assert!(catch().is_err());
    Ok(())
}