use std::fmt::{Display, Formatter, Result as DisplayResult};
use std::hash::{Hash, Hasher};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Implementation<'src> {
	pub functions: HashMap<ValueNode<'src>, ValueNode<'src>>,
	pub traits: HashMap<SymbolNode<'src>, Implementation<'src>>,
	pub generics: Vec<GenericType<'src>>,
}

impl<'src> From<HashMap<ValueNode<'src>, ValueNode<'src>>> for Implementation<'src> {
	fn from(functions: HashMap<ValueNode, ValueNode>) -> Self {
		Self {
			functions,
			traits: HashMap::new(),
			generics: vec![]
		}
	}
}

impl<'src> Implementation<'src> {
	// TODO make sure params match
	pub fn get_function(&self, symbol: &SymbolNode) -> Option<&FunctionNode> {
		if let Some(func) = self.functions.get(&symbol.0) {
			if let Value::Function(function) = &func.value {
				return Some(function)
			}
		}
		None
	}
	pub fn get_trait_function(&self, symbol: &SymbolNode) -> Option<(&Implementation, &FunctionNode)> {
		if let Some(implement) = self.traits.iter().find(|(k, _)| k.0 == symbol.0) {
			if let Some(function) = implement.1.get_function(symbol) {
				return Some((implement.1, &function));
			}
		}
		None
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

impl<'src> Hash for Implementation<'src> {
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

impl<'src> PartialEq for Implementation<'src> {
	fn eq(&self, other: &Self) -> bool {
		self.functions == other.functions
			&& self.traits == other.traits
	}
}

impl<'src> Eq for Implementation<'src> {}

impl<'src> Display for Implementation<'src> {
	fn fmt(&self, f: &mut Formatter) -> DisplayResult {		
		let functions: Vec<String> = self.functions
			.iter()
			.map(|(key, _)| key.to_string())
			.collect();
		let traits: Vec<String> = self.traits
			.iter()
			.map(|(key, _)| key.to_string())
			.collect();
		let generics: Vec<String> = self.generics
			.iter()
			.map(|generic| generic.to_string())
			.collect();
		let generics: String = if !generics.is_empty() { 
			format!("{} ", generics.join(", ")) 
		} else { 
			String::new() 
		};
		write!(f, "[{}functions: [{}], traits: [{}]]", generics, functions.join(", "), traits.join(", "))
	}
}