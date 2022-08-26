use cupid_ast::expr::Expr;
use cupid_debug::error::Error;
use cupid_env::environment::Env;
use cupid_lex::lexer::*;
use cupid_parse::parse::*;
use cupid_semantics::{
    analyze_scope::{AnalyzeScope, CreateScope},
    check_flow::CheckFlow,
    check_types::CheckTypes,
    infer_types::InferTypes,
    lint::Lint,
    resolve_names::ResolveNames,
    resolve_type_names::ResolveTypeNames,
};
use cupid_transpile::transpile_vec;
use std::rc::Rc;

mod tests;

const TYPEDEF: &str = r"
type int = []
";
const TRAITDEF: &str = r"
trait add = [
    add_it: left : int, right : int => left # todo
]
";
const DECL: &str = r"
let x : int = 1
x
";
const FUN: &str = r"
let sq = num : int => { x }
sq (1)
";

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    match compile(
        "
        trait equal (t) = [
            is: left : t, right : t => false
        ]
        1 is 1 # produces 1.is(1)
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
            .resolve_names(&mut env)
        // .infer_types(&mut env)?
        // .check_types(&mut env)?
        // .check_flow(&mut env)?
        // .lint(&mut env)
    }()
    .map_err(|e| {
        println!("{:#?}", env.closures);
        e
    })
    .map(|v| {
        println!("{:#?}", env.closures[&0]);
        v
    })
}

#[allow(unused)]
fn transpile(string: &str) -> Result<String, Error> {
    let parsed = parse(string).unwrap();
    // let compiled = compile(string)?;
    Ok(transpile_vec(&parsed).join("\n"))
}
