#![cfg(test)]
#![allow(unused)]
use cupid_ast::expr::Expr;
use cupid_env::environment::Env;
use cupid_lex::lexer::Lexer;
use cupid_parse::parse::Parser;
use crate::{Error, analyze_scope::AnalyzeScope, resolve_type_names::ResolveTypeNames, resolve_names::ResolveNames, infer_types::InferTypes, check_types::CheckTypes, check_flow::CheckFlow, lint::Lint};

macro_rules! do_passes {
    ($env:ident => $($x:ident),*) => {
        || -> Result<(), Error> {
            $( $x = $x.analyze_scope(&mut $env)?; )*
            $( $x = $x.resolve_type_names(&mut $env)?; )*
            $( $x = $x.resolve_names(&mut $env)?; )*
            $( $x = $x.infer_types(&mut $env)?; )*
            $( $x = $x.check_types(&mut $env)?; )*
            $( $x = $x.check_flow(&mut $env)?; )*
            $( $x = $x.lint(&mut $env)?; )*
            Ok(())
        }
    };
}

fn parse(string: &str) -> (Option<Vec<Expr>>, Env) {
    let mut parser = Parser::new();
    (parser.parse(Lexer::new().lex(string)), parser.env)
}

#[test]
fn test_type_def() -> Result<(), Error> {
    let (mut expr, mut env) = parse("
        type int = [] 
        let x : int = 1 
        x
    ");
    assert!(do_passes!(env => expr)().is_ok());
    Ok(())
}

#[test]
fn test_type_def_undefined() -> Result<(), Error> {
    let (mut expr, mut env) = parse("
        type int = [] 
        let x : dec = 1.5
        x
    ");
    assert!(do_passes!(env => expr)().is_err());
    Ok(())
}

#[test]
fn test_type_def_wrong_type() -> Result<(), Error> {
    let (mut expr, mut env) = parse("
        type int = [] 
        let x : int = 1.5
        x
    ");
    assert!(do_passes!(env => expr)().is_err());
    Ok(())
}

#[test]
fn test_type_def_unused() -> Result<(), Error> {
    let (mut expr, mut env) = parse("
        type int = [] 
        let x : int = 1
    ");
    assert!(do_passes!(env => expr)().is_err());
    Ok(())
}