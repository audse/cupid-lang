
// External
#[allow(unused_imports)]
use colored::*;
use wasm_bindgen::prelude::*;
pub use serde::{Serialize, Deserialize};
pub use lazy_static::lazy_static;

// Stdlib
pub use std::collections::HashMap;
pub use std::borrow::Cow;
pub use std::hash::{Hash, Hasher};
pub use std::fmt::{Display, Formatter, Result as DisplayResult};
pub use std::collections::hash_map::Entry;

mod errors;
pub use errors::*;

mod file_handler;
pub use file_handler::FileHandler;

mod utils;
pub use utils::*;

mod packages;
pub use packages::*;

mod parsing;
pub use parsing::*;

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
pub struct ScopeEntry {
	pub context: Context,
	pub storage: Vec<StorageEntry>,
}

#[derive(Serialize, Deserialize)]
pub struct StorageEntry {
	pub symbol: ValueNode,
	pub value: SymbolValue
}

#[derive(Serialize, Deserialize)]
pub struct Cupid {
	pub values: Vec<(String, ValueNode)>,
	pub semantics: Vec<BoxAST>,
	pub parse: ParseNode,
	pub errors: Vec<Error>,
	pub scope: Vec<ScopeEntry>
}

#[wasm_bindgen]
pub fn run_and_collect_logs(string: &str) -> JsValue {
	let mut file_handler = FileHandler::from(string);
	let stdlib = read_file();
	_ = file_handler.preload_contents(stdlib);
	
	let parse_tree = file_handler.parser._file();
	let mut parse = parse_tree.unwrap().0;
	let file = match Result::<FileNode, Error>::from(&mut parse) {
		Ok(ok) => ok,
		Err(e) => panic!("{}", e.string(".."))
	};
	
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