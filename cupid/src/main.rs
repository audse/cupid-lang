#![allow(unused)]

use cupid::{
    cupid_lex::lexer::*,
    cupid_parse::parse::*,
};
use cupid_ast::expr::Expr;
use cupid_env::environment::Env;
use cupid_semantics::{analyze_scope::AnalyzeScope, Error, resolve_type_names::ResolveTypeNames, resolve_names::ResolveNames, infer_types::InferTypes, check_types::CheckTypes, check_flow::CheckFlow, lint::Lint};

fn main() {
    let def = parse("type int = [] let x : int = 1");
    println!("{def:#?}")
}

fn parse(string: &str) -> Option<Vec<Expr>> {
    Parser::new().parse(Lexer::new().lex(string))
}

fn compile(string: &str) -> Result<Vec<Expr>, Error> {
    let mut env = Env::default();
    Parser::new().parse(Lexer::new().lex(string)).unwrap()
        // .resolve_packages(&mut env)?
        // .analyze_type_scope(&mut env)?
        .analyze_scope(&mut env)?
        .resolve_type_names(&mut env)?
        .resolve_names(&mut env)?
        .infer_types(&mut env)?
        .check_types(&mut env)?
        .check_flow(&mut env)?
        .lint(&mut env)
}