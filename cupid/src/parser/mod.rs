use std::fs::{write, read_to_string};
mod grammar_parser;
pub use grammar_parser::*;
mod parser_generator;
pub use parser_generator::generate_parser;
mod cupid;
pub use cupid::Parser as CupidParser;

const BASE_PATH: &str = "src/grammar/parser_base.rs";
const PLACEHOLDER: &str = "/*RULES*/";

pub fn generate(grammar_path: &str, destination_path: &str) {
	let base = read_to_string(BASE_PATH).unwrap();
	let body = read_to_string(grammar_path).unwrap();
	let mut parser = GrammarParser::new(body);
	let rules = parser.grammar();
	let result = generate_parser(rules);
	write(destination_path, base.replace(PLACEHOLDER, result.as_str()));
}

pub fn test_generator() {
	// generate("src/grammar/grammar.grammar", "src/parser/grammar.rs");
	// generate("src/grammar/cupid-lang.grammar", "src/parser/cupid.rs");
	let mut parser = CupidParser::new("func(1, 2, 3, 4, 5)".to_string());
	println!("{:#?}", parser._expression(None));
}