#![allow(unused)]

use cupid::{
    cupid_lex::lexer::*,
    cupid_parse::parse::*,
};

use cupid_passes::{package_resolution::ResolvePackages,  env::environment::Env, type_scope_analysis::AnalyzeTypeScope, type_name_resolution::ResolveTypeNames, scope_analysis::AnalyzeScope, name_resolution::ResolveNames, type_inference::pass::InferTypes, type_checking::CheckTypes, flow_checking::CheckFlow, linting::Lint};

fn main() {
    let def = parse("type bool = [ true : int, false ]");
    println!("{def:#?}")
}

fn parse(string: &str) -> Option<Vec<cupid_passes::pre_analysis::Expr>> {
    Parser::new().parse(Lexer::new().lex(string))
}

fn compile(string: &str) -> Result<Vec<cupid_passes::name_resolution::Expr>, (usize, usize)> {
    let mut env = Env::default();
    Parser::new().parse(Lexer::new().lex(string)).unwrap()
        .resolve_packages(&mut env)?
        .analyze_type_scope(&mut env)?
        .resolve_type_names(&mut env)?
        .analyze_scope(&mut env)?
        .resolve_names(&mut env)
        // .infer_types(&mut env)
        // .check_types(&mut env)?
        // .check_flow(&mut env)?
        // .lint(&mut env)
}