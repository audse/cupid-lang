use std::fs::{write, read_to_string};
use crate::*;

// Grammars
const BUILTIN: &str = "../cupid-parse/src/grammar/builtin.grammar";
const CUPID: &str = "../cupid-parse/src/grammar/cupid.grammar";
const PACKAGES: &str = "../cupid-parse/src/grammar/packages.grammar";
const TYPES: &str = "../cupid-parse/src/grammar/types.grammar";
const UTILS: &str = "../cupid-parse/src/grammar/utils.grammar";

const PARSER_BASE: &str = "../cupid-parse/src/parsers/base.rs";
const PLACEHOLDER: &str = "/*RULES*/";

pub fn read(grammar_paths: &[&str]) -> (Cow<'static, str>, Cow<'static, str>) {
	println!("{grammar_paths:#?}");
	let grammars: Vec<String> = grammar_paths
		.iter()
		.map(|path| read_to_string(path).unwrap_or_else(|path| panic!("couldn't find {path}")))
		.collect();
	(
		read_to_string(PARSER_BASE).unwrap_or_else(|path| panic!("couldn't find {path}")).into(),
		(grammars.join("\n")).into()
	)
}

pub fn generate(grammar_paths: &[&str], destination_path: Str) {
	let (base, body) = read(grammar_paths);
	let name = name(destination_path.to_owned());
	let base = base.replace("BaseParser", &*name);
	
	let mut parser: GrammarParser = GrammarParser::new(name, body, 0);
	let grammar = parser.grammar();
	let result = grammar.stringify();
	_ = write(&*destination_path, base.replace(PLACEHOLDER, &result));
}

fn name(path: Str) -> Str {
	let name = path.split('/').last().unwrap_or("");
	let chars = name.split_at(1);
	let name = chars.1.split_once('.').unwrap();
	format!("{}{}Parser", chars.0.to_uppercase(), name.0).into()
}

pub fn use_generator(which: i32) {
	println!("Running generator...");
	match which {
		1 => generate(&[CUPID, BUILTIN, PACKAGES, TYPES, UTILS], "../cupid-parse/src/parsers/cupid.rs".into()),
		2 => generate(&[PACKAGES, UTILS], "../cupid-parse/src/parsers/packages.rs".into()),
		3 => generate(&[TYPES, UTILS], "../cupid-parse/src/parsers/types.rs".into()),
		_ => panic!("must specify a parser to generate")
	}
}