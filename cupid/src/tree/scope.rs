use std::collections::HashMap;
use std::fmt::{
	Display,
	Formatter,
	Result,
};
use crate::{Symbol, Value};

#[derive(Debug, Clone)]
pub struct Scope {
	pub storage: HashMap<Symbol, SymbolValue>,
	pub parent: Option<Box<Self>>,
}

#[derive(Debug, Clone)]
pub struct SymbolValue {
	pub value: Value,
	pub mutable: bool,
	pub deep_mutable: bool,
}

impl Display for Scope {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "CupidScope {{ storage: {:?} }}", self.storage)
	}
}


impl Scope {

	pub fn new(parent: Option<Self>) -> Self {
		Self {
			storage: HashMap::new(),
			parent: parent.map(Box::new),
		}
	}

	pub fn get_symbol(&self, symbol: &Symbol) -> Option<&Value> {
		if let Some(stored_symbol) = self.storage.get(symbol) {
			return Some(&stored_symbol.value);
		}
		if let Some(stored_symbol) = self.get_parent_symbol(symbol) {
			return Some(stored_symbol);
		}
		None
	}

	pub fn set_symbol(&mut self, symbol: &Symbol, value: Value) -> Option<&Value> {
		if let Some(stored_value) = self.storage.get_mut(symbol) {
			if stored_value.mutable {
				stored_value.value = value;
			}
		}
		self.get_symbol(symbol)
	}
	
	pub fn create_symbol(&mut self, symbol: &Symbol, value: Value, mutable: bool, deep_mutable: bool) -> Option<&Value> {
		self.storage.insert(symbol.clone(), SymbolValue {
			value, 
			mutable, 
			deep_mutable
		});
		self.get_symbol(symbol)
	}

	pub fn get_parent_symbol(&self, symbol: &Symbol) -> Option<&Value> {
		if let Some(parent) = &self.parent {
			return parent.get_symbol(symbol);
		}
		None
	}

	pub fn make_sub_scope(&self) -> Self {
		Self::new(Some(self.clone()))
	}
}