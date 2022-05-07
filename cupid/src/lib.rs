// #![feature(test)]

// External
#[allow(unused_imports)]
use colored::*;
use wasm_bindgen::prelude::*;

mod errors;
pub use errors::*;

// mod file_handler;
// pub use file_handler::*;

mod file_handler_refactor;
pub use file_handler_refactor::FileHandler as RFileHandler;

mod utils;
pub use utils::*;

mod tree;
pub use tree::*;

mod parser;
pub use parser::*;

// mod semantics;
// pub use semantics::*;

mod refactor;
pub use refactor::{
	LexicalScope as RLexicalScope,
	Scope as RScope,
	SymbolValue as RSymbolValue,
	AssignmentNode,
	DeclarationNode,
	GenericsNode,
	parse,
	SymbolNode,
	TypeHintNode,
	ValueNode,
	AST,
	CloneAST,
	Context,
	Meta,
	Flag,
	BuiltinTypeNode,
	UseBlockNode,
	BlockNode,
	FunctionNode,
	ParametersNode,
	OptionAST,
	BoxAST,
	FunctionCallNode,
	ArgumentsNode,
	LogNode,
	ArrayNode,
	Implementation,
	FileNode,
	OperationNode,
	FunctionFlag,
	UseTraitBlockNode,
	TraitNode,
	ImplementationNode,
	PropertyNode,
};

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
	pub context: Context,
	pub storage: Vec<StorageEntry>,
}

#[derive(Serialize, Deserialize)]
pub struct StorageEntry {
	pub symbol: ValueNode,
	pub value: RSymbolValue
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
	let mut file_handler = RFileHandler::from(string);
	let stdlib = read_file();
	file_handler.preload_contents(stdlib);
	
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