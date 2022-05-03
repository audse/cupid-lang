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
pub struct ScopeEntry {
	pub context: ScopeContext,
	pub storage: Vec<StorageEntry>,
}

#[derive(Serialize, Deserialize)]
pub struct StorageEntry {
	pub symbol: Value,
	pub value: SymbolValue
}

#[derive(Serialize, Deserialize)]
pub struct Cupid {
	pub values: Vec<(String, Value)>,
	pub semantics: Vec<Expression>,
	pub parse: ParseNode,
	pub errors: Vec<Error>,
	pub scope: Vec<ScopeEntry>
}

#[wasm_bindgen]
pub fn run_and_collect_logs(string: &str) -> JsValue {
	let mut file_handler = FileHandler::from(string);
	let stdlib = read_file();
	file_handler.preload_contents(stdlib);
	
	let parse_tree = file_handler.parser._file(None);
	let parse = parse_tree.unwrap().0;
	let file = to_tree(&parse);
	
	let mut semantics: Vec<Expression> = vec![];
	let mut values: Vec<(String, Value)> = vec![];
	let mut errors: Vec<Error> = vec![];
	
	file_handler.scope.add(ScopeContext::Block);
	if let Expression::File(ref f) = file {
		for exp in f {
			let exp_val = exp.resolve(&mut file_handler.scope);
			match exp_val {
				Value::Error(e) => errors.push(e),
				_ => values.push((exp_val.to_string(), exp_val))
			};
			semantics.push(exp.clone())
		}
	}
	
	let scope: Vec<ScopeEntry> = file_handler.scope.scopes.iter().map(|ls| {
		let storage: Vec<StorageEntry> = ls.storage.iter().map(|(v, sv)| 
			StorageEntry { 
				symbol: v.to_owned(),
				value: sv.to_owned()
			}
		).collect();
		ScopeEntry {
			context: ls.context.to_owned(),
			storage
		}
	}).collect();
	
	let val = Cupid {
		values,
		semantics,
		parse,
		errors,
		scope,
	};
	
	// JsValue::from_serde(&val).unwrap()
	serde_wasm_bindgen::to_value(&val).unwrap()
}