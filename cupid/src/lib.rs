
// External
#[allow(unused_imports)]
use colored::*;
use wasm_bindgen::prelude::*;
pub use serde::{Serialize, Deserialize};

// Stdlib
pub use std::collections::HashMap;
pub use std::borrow::Cow;

mod errors;
pub use errors::*;

mod file_handler;
pub use file_handler::FileHandler;

mod utils;
pub use utils::*;

mod parser;
pub use parser::*;

mod semantics;
pub use semantics::*;

mod tree;
pub use tree::*;

mod tests;
pub use tests::*;

mod tokenizer;
pub use tokenizer::*;

mod type_system;
pub use type_system::*;

#[wasm_bindgen(raw_module = "/../playground/src/stdlib.js")]
extern {
	pub fn read_file() -> String;
}


#[derive(Serialize, Deserialize)]
pub struct ScopeEntry<'src> {
	pub context: Context,
	pub storage: Vec<StorageEntry<'src>>,
}

#[derive(Serialize, Deserialize)]
pub struct StorageEntry<'src> {
	pub symbol: ValueNode<'src>,
	pub value: SymbolValue<'src>
}

#[derive(Serialize, Deserialize)]
pub struct Cupid<'src> {
	pub values: Vec<(String, ValueNode<'src>)>,
	pub semantics: Vec<BoxAST>,
	pub parse: ParseNode<'src>,
	pub errors: Vec<Error>,
	pub scope: Vec<ScopeEntry<'src>>
}

#[wasm_bindgen]
pub fn run_and_collect_logs(string: &str) -> JsValue {
	let mut file_handler = FileHandler::from(string);
	let stdlib = read_file();
	_ = file_handler.preload_contents(stdlib);
	
	let parse_tree = file_handler.parser._file(None);
	let mut parse = parse_tree.unwrap().0;
	let file = FileNode::from(&mut parse);
	
	let mut semantics: Vec<BoxAST> = vec![];
	let mut values: Vec<(String, ValueNode)> = vec![];
	let mut errors: Vec<Error> = vec![];
	
	for exp in file.expressions {
		let exp_val = exp.resolve(&mut file_handler.scope);
		match exp_val {
			Err(e) => errors.push(e),
			Ok(val) => values.push((val.to_string(), val))
		};
		semantics.push(exp.clone())
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