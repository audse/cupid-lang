use std::fs::{write, read_to_string};
use crate::*;

mod cupid_package;
pub use cupid_package::BaseParser as PackageParser;

mod cupid_parser;
pub use cupid_parser::BaseParser as CupidParser;

mod grammar;
pub use grammar::*;

mod grammar_parser;
pub use grammar_parser::*;

mod node;
pub use node::*;

mod parser_generator;
pub use parser_generator::*;

mod parser;
pub use parser::*;

const BASE_PATH: &str = "src/parser/base_parser.rs";
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
	generate("src/grammar/cupid-lang.grammar", "src/parser/cupid_parser.rs");
}

pub fn generate_package_parser() {
	generate("src/grammar/cupid-package.grammar", "src/parser/cupid_package.rs");
}