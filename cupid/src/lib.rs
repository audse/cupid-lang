// #![feature(test)]

// External
#[allow(unused_imports)]
use colored::*;
use wasm_bindgen::prelude::*;

mod errors;
pub use errors::*;

mod file_handler;
pub use file_handler::*;

mod utils;
pub use utils::*;

// mod tokens;
// pub use tokens::*;

mod tree;
pub use tree::*;

mod parser;
pub use parser::*;

mod semantics;
pub use semantics::*;

mod tests;
pub use tests::*;

mod tokenizer;
pub use tokenizer::*;

// #[wasm_bindgen]
// extern {
// 	pub fn alert(s: &str);
// }

#[wasm_bindgen]
pub fn run(string: &str) -> String {
	let mut file_handler = FileHandler::from(string);
	let result: Vec<String> = file_handler
		.run_and_return()
		.iter()
		.map(|v| v.to_string())
		.collect();
	result.join("\n")
}

#[wasm_bindgen]
pub fn run_and_collect_logs(string: &str) -> String {
	let mut file_handler = FileHandler::from(string);
	
	let parse_tree = file_handler.parser._file(None);        
	let semantics = to_tree(&parse_tree.unwrap().0);
	
	let mut values: Vec<String> = vec![];
	if let Expression::File(f) = semantics {
		for exp in f {
			let exp_val = exp.resolve(&mut file_handler.scope);
			match &exp_val {
				Value::Error(error) => values.push(error.to_string()),
				_ => ()
			};
			match exp {
				Expression::Logger(_) => values.push(exp_val.to_string()),
				_ => ()
			};
		}
	}
	values.join("\n")
}