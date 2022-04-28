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

use serde::{Serialize, Deserialize};

#[wasm_bindgen(raw_module = "/../playground/src/stdlib.js")]
extern {
	pub fn read_file() -> String;
}


#[derive(Serialize, Deserialize)]
pub struct Cupid {
	pub values: Vec<Value>,
	pub semantics: Expression,
	pub errors: Vec<Error>
}

#[wasm_bindgen]
pub fn run(string: &str) -> String {
	let mut file_handler = FileHandler::from(string);
	file_handler.preload_contents(read_file());
	let result: Vec<String> = file_handler
		.run_and_return()
		.iter()
		.map(|v| v.to_string())
		.collect();
	result.join("\n")
}

#[wasm_bindgen]
pub fn run_and_collect_logs(string: &str) -> JsValue {
	let mut file_handler = FileHandler::from(string);
	file_handler.preload_contents(read_file());
	
	let parse_tree = file_handler.parser._file(None);        
	let semantics = to_tree(&parse_tree.unwrap().0);
	
	let mut values: Vec<Value> = vec![];
	if let Expression::File(ref f) = semantics {
		for exp in f {
			let exp_val = exp.resolve(&mut file_handler.scope);
			values.push(exp_val)
			// match &exp_val {
			// 	Value::Error(error) => values.push(error.to_string()),
			// 	_ => ()
			// };
			// match exp {
			// 	Expression::Logger(_) => values.push(exp_val.to_string()),
			// 	_ => ()
			// };
		}
	}
	let val = Cupid {
		values,
		semantics,
		errors: file_handler.errors
	};
	
	// JsValue::from_serde(&val).unwrap()
	serde_wasm_bindgen::to_value(&val).unwrap()
}