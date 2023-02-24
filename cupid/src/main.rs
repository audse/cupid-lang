use cupid_ast::expr::Expr;
use cupid_debug::error::Error;
use cupid_env::environment::Env;
use cupid_lex::lexer::*;
use cupid_parse::parse::*;
#[allow(unused_imports)]
use cupid_semantics::{
    analyze_scope::{AnalyzeScope, CreateScope},
    check_flow::CheckFlow,
    check_types::CheckTypes,
    infer_types::InferTypes,
    lint::Lint,
    resolve_names::ResolveNames,
    resolve_type_names::ResolveTypeNames,
};
use std::rc::Rc;

mod tests;

#[allow(unused)]
const TYPEDEF: &str = r"
type int = []
";

#[allow(unused)]
const TRAITDEF: &str = r"
trait add = [
    add!: left : int, right : int => left # todo
]
";

#[allow(unused)]
const DECL: &str = r"
let x : int = 1
x
";

#[allow(unused)]
const FUN: &str = r"
let sq = num : int => { x }
sq (1)
";

/*
TODO
- New struct "Allocate" to help assign, decl, typedef, traitdef
*/

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    match compile(
        "
        type int = []
        trait addition (t) = [
            add!: left : t, right : t => { left }   
        ]
        implement addition for int = []
        1 + 1
    ",
    ) {
        Ok(_) => println!("Ok"),
        Err(err) => eprintln!("{err}"),
    }
}

#[allow(unused)]
fn parse(string: &str) -> Option<Vec<Expr>> {
    Parser::new(Rc::new(string.to_string())).parse(Lexer::new().lex(string))
}

#[allow(unused)]
fn compile(string: &str) -> Result<Vec<Expr>, Error> {
    let mut parser = Parser::new_with(Rc::new(string.to_string()), Env::default());
    let mut parsed = parser.parse(Lexer::new().lex(string)).unwrap();
    let mut env = parser.env;
    for expr in parsed.iter() {
        expr.create_scope(env.get_closure(0), &mut env);
    }
    || -> Result<Vec<Expr>, Error> {
        parsed
            // .resolve_packages(&mut env)?
            .analyze_scope(&mut env)?
            .resolve_type_names(&mut env)?
            .resolve_names(&mut env)?
            .infer_types(&mut env)
        // .check_types(&mut env)?
        // .check_flow(&mut env)?
        // .lint(&mut env)
    }()
}
