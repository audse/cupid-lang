use std::collections::HashMap;
use std::fmt::{
	Display,
	Formatter,
	Result,
};
use crate::{
	CupidSymbol,
	CupidValue,
};

#[derive(Debug)]
pub struct CupidScope {
	pub storage: HashMap<CupidSymbol, CupidValue>,
	pub parent: Option<Box<Self>>,
}

impl Display for CupidScope {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "CupidScope {{ storage: {:?} }}", self.storage)
	}
}

impl Clone for CupidScope {
	fn clone(&self) -> Self {
    	let mut storage = HashMap::new();
		for (key, val) in self.storage.iter() {
			storage.insert(key.clone(), val.clone());
		}
		return Self {
			storage,
			parent: self.parent.clone()
		}
	}
}


impl CupidScope {

	pub fn new(parent: Option<Self>) -> Self {
		Self {
			storage: HashMap::new(),
			parent: match parent {
				Some(p) => Some(Box::new(p)),
				None => None,
			}
		}
	}

	pub fn get_symbol(&self, symbol: &CupidSymbol) -> Option<&CupidValue> {
		if let Some(stored_symbol) = self.storage.get(symbol) {
			return Some(stored_symbol);
		}
		if let Some(stored_symbol) = self.get_parent_symbol(symbol) {
			return Some(stored_symbol);
		}
		return None;
	}

	pub fn set_symbol(&mut self, symbol: &CupidSymbol, value: CupidValue) -> Option<&CupidValue> {
		if let Some((stored_symbol, _stored_value)) = self.storage.get_key_value(&symbol) {
			if !stored_symbol.mutable { return None; }
		}
		self.storage.insert(symbol.clone(), value);
		return self.get_symbol(symbol);
	}

	pub fn get_parent_symbol(&self, symbol: &CupidSymbol) -> Option<&CupidValue> {
		if let Some(parent) = &self.parent {
			return parent.get_symbol(symbol);
		}
		return None;
	}

	pub fn make_sub_scope(&self) -> Self {
		Self::new(Some(self.clone()))
	}
}