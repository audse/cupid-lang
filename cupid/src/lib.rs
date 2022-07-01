use wasm_bindgen::prelude::*;
use std::rc::Rc;
use cupid_lex::lexer::*;
use cupid_parse::parse::*;
use cupid_ast::expr::Expr;
use cupid_debug::error::Error;
use cupid_env::environment::Env;
use cupid_semantics::{analyze_scope::AnalyzeScope, resolve_type_names::ResolveTypeNames, resolve_names::ResolveNames, infer_types::InferTypes, check_types::CheckTypes, check_flow::CheckFlow, lint::Lint};


#[allow(unused)]
fn parse(string: &str) -> Option<Vec<Expr>> {
    Parser::new(Rc::new(string.to_string())).parse(Lexer::new().lex(string))
}

#[allow(unused)]
#[wasm_bindgen]
pub fn compile(string: &str) -> Result<JsValue, JsValue> {
    console_error_panic_hook::set_once();
    let mut env = Env::default();
    let exprs = Parser::new(Rc::new(string.to_string())).parse(Lexer::new().lex(string)).unwrap();

    let result = catch_compile(exprs, &mut env);
    result.as_ref().map(|r| JsValue::from_serde(r).unwrap()).map_err(|e| JsValue::from_serde(e).unwrap())
}

fn catch_compile(expr: Vec<Expr>, env: &mut Env) -> Result<Vec<Expr>, Error> {
    expr
        // .resolve_packages(&mut env)?
        .analyze_scope(env)?
        .resolve_type_names(env)?
        .resolve_names(env)?
        .infer_types(env)?
        .check_types(env)?
        .check_flow(env)?
        .lint(env)
}