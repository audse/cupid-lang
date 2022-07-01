use std::rc::Rc;
use cupid_lex::lexer::*;
use cupid_parse::parse::*;
use cupid_ast::expr::Expr;
use cupid_debug::error::Error;
use cupid_env::environment::Env;
use cupid_semantics::{analyze_scope::AnalyzeScope, resolve_type_names::ResolveTypeNames, resolve_names::ResolveNames, infer_types::InferTypes, check_types::CheckTypes, check_flow::CheckFlow, lint::Lint};

fn main() {
    let def = parse("type int = [] let x : int = 1");
    println!("{def:#?}")
}

#[allow(unused)]
fn parse(string: &str) -> Option<Vec<Expr>> {
    Parser::new(Rc::new(string.to_string())).parse(Lexer::new().lex(string))
}

#[allow(unused)]
fn compile(string: &str) -> Result<Vec<Expr>, Error> {
    let mut env = Env::default();
    Parser::new(Rc::new(string.to_string())).parse(Lexer::new().lex(string)).unwrap()
        // .resolve_packages(&mut env)?
        .analyze_scope(&mut env)?
        .resolve_type_names(&mut env)?
        .resolve_names(&mut env)?
        .infer_types(&mut env)?
        .check_types(&mut env)?
        .check_flow(&mut env)?
        .lint(&mut env)
}
