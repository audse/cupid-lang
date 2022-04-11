use std::fs::{write, read_to_string};
// extern crate test;
// use test::bench::Bencher;
mod grammar_parser;
pub use grammar_parser::*;
mod parser_generator;
pub use parser_generator::generate_parser;
mod cupid;
pub use self::cupid::{Parser as CupidParser, Node as ParseNode};
mod grammar;


const BASE_PATH: &str = "src/parser/parser_base.rs";
const PLACEHOLDER: &str = "/*RULES*/";

pub fn generate(grammar_path: &str, destination_path: &str) {
	let base = read_to_string(BASE_PATH).unwrap();
	let body = read_to_string(grammar_path).unwrap();
	let mut parser = GrammarParser::new(body);
	let rules = parser.grammar();
	let result = generate_parser(rules);
	let _ok = write(destination_path, base.replace(PLACEHOLDER, result.as_str()));
}

pub fn test_generator() {
	generate("src/grammar/cupid-lang.grammar", "src/parser/cupid.rs");
	// generate("src/grammar/grammar.grammar", "src/parser/grammar.rs");
	// test_cupid_parser();
}