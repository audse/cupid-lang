use std::fmt::{Display, Formatter, Result as DisplayResult};
use std::hash::{Hash, Hasher};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Implementation {
	pub functions: HashMap<ValueNode, ValueNode>,
	pub traits: HashMap<SymbolNode, Implementation>
}

impl Implementation {
	pub fn new() -> Self {
		Self {
			functions: HashMap::new(),
			traits: HashMap::new(),
		}
	}
	pub fn find_function_value(&self, symbol: &SymbolNode, scope: &mut LexicalScope) -> Option<FunctionNode> {
		if let Some(func) = self.find_function(symbol, scope) {
			match func.value {
				Value::Function(function) => Some(function),
				_ => None
			}
		} else {
			None
		}
	}
	
	// TODO make sure params match
	pub fn find_function(&self, symbol: &SymbolNode, scope: &mut LexicalScope) -> Option<ValueNode> {
		if let Some(func) = self.get_function(symbol) {
			Some(func.to_owned())
		} else {
			for t in &self.traits {
				// search in custom implementations
				if let Some(fun) = t.1.find_function(symbol, scope) {
					return Some(fun);
				}
				// search in stored implementations
				match scope.get_symbol(&t.0) {
					Ok(value) => if let Value::Implementation(trait_definition) = value.value {
						if let Some(func) = trait_definition.find_function(symbol, scope) {
							return Some(func.to_owned());
						}
					},
					_ => ()
				}
			}
			None
		}
	}
	pub fn get_function(&self, symbol: &SymbolNode) -> Option<&ValueNode> {
		if let Some(func) = self.functions.get(&symbol.0) {
			Some(func)
		} else if let Some(implement) = self.traits.iter().find(|(k, _)| k.0 == symbol.0) {
			implement.1.get_function(symbol)
		} else {
			None
		}
	}
	pub fn implement(&mut self, functions: HashMap<ValueNode, ValueNode>) {
		functions.into_iter().for_each(|(k, v)| {
			self.functions.insert(k, v); 
		});
	}
	pub fn implement_trait(&mut self, trait_symbol: SymbolNode, implement: Implementation) {
		self.traits.insert(trait_symbol, implement);
	}
}

impl Hash for Implementation {
	fn hash<H: Hasher>(&self, state: &mut H) {
		for (symbol, func) in self.functions.iter() {
			symbol.hash(state);
			func.hash(state);
		}
		for (trait_symbol, implement) in self.traits.iter() {
			trait_symbol.hash(state);
			implement.hash(state);
		}
	}
}

impl PartialEq for Implementation {
	fn eq(&self, other: &Self) -> bool {
		self.functions == other.functions
			&& self.traits == other.traits
	}
}

impl Eq for Implementation {}

impl Display for Implementation {
	fn fmt(&self, f: &mut Formatter) -> DisplayResult {		
		let functions: Vec<String> = self.functions
			.iter()
			.map(|(key, value)| format!("{key}: {value}"))
			.collect();
		let traits: Vec<String> = self.traits
			.iter()
			.map(|(key, value)| format!("{key}: {value}"))
			.collect();
		write!(f, "functions: [{}], traits: [{}]", functions.join(", "), traits.join(", "))
	}
}