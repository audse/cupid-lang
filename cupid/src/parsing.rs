use std::fs::{write, read_to_string};
use crate::*;

mod generator;
pub use generator::*;

mod node;
pub use node::*;

mod parser_trait;
pub use parser_trait::*;

mod parsers;
pub use parsers::*;

// Grammars
const BUILTIN: &str = "src/grammar/builtin.grammar";
const CUPID: &str = "src/grammar/cupid.grammar";
const PACKAGES: &str = "src/grammar/packages.grammar";
const TYPES: &str = "src/grammar/types.grammar";
const UTILS: &str = "src/grammar/utils.grammar";

const PARSER_BASE: &str = "src/parsing/parsers/base.rs";
const PLACEHOLDER: &str = "/*RULES*/";

pub fn read(grammar_paths: &[&str]) -> (Cow<'static, str>, Cow<'static, str>) {
	let grammars: Vec<String> = grammar_paths
		.iter()
		.map(|path| read_to_string(path).unwrap())
		.collect();
	println!("{grammar_paths:#?}");
	(
		read_to_string(PARSER_BASE).unwrap().into(),
		(grammars.join("\n")).into()
	)
}

pub fn generate(grammar_paths: &[&str], destination_path: Str) {
	let (base, body) = read(grammar_paths);
	let name = name(destination_path.to_owned().into());
	let base = base.replace("BaseParser", &*name);
	
	let mut parser: GrammarParser = GrammarParser::new(name.into(), body, 0);
	let grammar = parser.grammar();
	let result = grammar.stringify();
	_ = write(&*destination_path, base.replace(PLACEHOLDER, &result));
}

fn name(path: Str) -> Str {
	let name = path.split("/").last().unwrap_or("");
	let chars = name.split_at(1);
	let name = chars.1.split_once(".").unwrap();
	format!("{}{}Parser", chars.0.to_uppercase(), name.0).into()
}

pub fn use_generator(which: i32) {
	println!("Running generator...");
	match which {
		1 => generate(&[CUPID, BUILTIN, PACKAGES, TYPES, UTILS], "src/parsing/parsers/cupid.rs".into()),
		2 => generate(&[PACKAGES, UTILS], "src/parsing/parsers/packages.rs".into()),
		3 => generate(&[TYPES, UTILS], "src/parsing/parsers/types.rs".into()),
		_ => panic!("must specify a parser to generate")
	}
}