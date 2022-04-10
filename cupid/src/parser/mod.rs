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
use crate::semantics::to_tree;
use crate::{Tree, Scope};


const BASE_PATH: &str = "src/parser/parser_base.rs";
const PLACEHOLDER: &str = "/*RULES*/";

pub fn generate(grammar_path: &str, destination_path: &str) {
	let base = read_to_string(BASE_PATH).unwrap();
	let body = read_to_string(grammar_path).unwrap();
	let mut parser = GrammarParser::new(body);
	let rules = parser.grammar();
	let result = generate_parser(rules);
	let ok = write(destination_path, base.replace(PLACEHOLDER, result.as_str()));
}

pub fn test_generator() {
	generate("src/grammar/cupid-lang.grammar", "src/parser/cupid.rs");
	// generate("src/grammar/grammar.grammar", "src/parser/grammar.rs");
	// test_cupid_parser();
}

// #[bench]
// pub fn test_cupid_parser(_b: &mut Bencher) {
// 	pub use cupid::Parser as CupidParser;
// 	let mut parser = CupidParser::new("trueValue".to_string());
// 	println!("{:#?}", parser._expression(None));
// }

pub fn test_cupid_parser() {
	let mut parser = CupidParser::new("1 + 2 + 3".to_string());
	let parse_tree = parser._expression(None);
	// println!("{:#?}", parse_tree);
	let semantics = to_tree(&parse_tree.unwrap().0);
	let mut scope = Scope::new(None);
	println!("{:#?}", semantics.resolve(&mut scope));
}