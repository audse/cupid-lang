use std::fs::{write, read_to_string};
// extern crate test;
// use test::bench::Bencher;
mod grammar_parser;
pub use grammar_parser::*;
mod parser_generator;
pub use parser_generator::generate_parser;
mod cupid;
pub use self::cupid::{Parser as CupidParser, Node as ParseNode};
use crate::Cow;
mod grammar;

// mod parser_macros;
// pub use parser_macros::*;

// mod cupid_macro;
// pub use cupid_macro::{Parser as MacroParser, Node as ParseNode};

const BASE_PATH: &str = "src/parser/parser_base.rs";
const PLACEHOLDER: &str = "/*RULES*/";

pub fn read(grammar_path: &str) -> (Cow<'static, str>, Cow<'static, str>) {
	(
		read_to_string(BASE_PATH).unwrap().into(), 
		read_to_string(grammar_path).unwrap().into()
	)
}

pub fn generate(grammar_path: &str, destination_path: &str) {
	let (base, body) = read(grammar_path);
	let mut parser: GrammarParser = GrammarParser::new(body);
	let rules = parser.grammar();
	let result = generate_parser(rules);
	let _ok = write(destination_path, base.replace(PLACEHOLDER, &result));
}

pub fn test_generator() {
	generate("src/grammar/cupid-lang.grammar", "src/parser/cupid.rs");
}