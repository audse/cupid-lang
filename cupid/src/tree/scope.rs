use std::collections::HashMap;
use crate::{Symbol, Value, Type};

#[derive(Debug, Clone)]
pub struct Scope {
	pub storage: HashMap<Symbol, SymbolValue>,
	pub parent: Option<Box<Self>>,
}

#[derive(Debug, Clone)]
pub struct SymbolValue {
	pub value: Value,
	pub r#type: Type,
	pub mutable: bool,
	pub deep_mutable: bool,
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

	pub fn set_symbol(&mut self, symbol: &Symbol, value: Value) -> Result<Option<&Value>, Value> {
		if let Some(stored_value) = self.storage.get_mut(symbol) {
			if stored_value.mutable {
				let setting_type = Type::from_value(&value);
				let stored_type = Type::from_value(&stored_value.value);
				if setting_type == stored_type {
					stored_value.value = value;
				} else {
					return Err(Value::error(
						&symbol.1[0],
						format!(
							"type mismatch: `{symbol}` ({stored_type}) can't be assigned `{value}` ({setting_type})",
							setting_type = setting_type,
							symbol = symbol.get_identifier(),
							stored_type = stored_type,
							value = value,
						)
					))
				}
			} else {
				return Err(Value::error(
					&symbol.1[0],
					format!(
						"variable `{symbol}` is immutable and cannot be reassigned",
						symbol = symbol.get_identifier(),
					)
				))
			}
		}
		Ok(self.get_symbol(symbol))
	}
	
	pub fn create_symbol(&mut self, symbol: &Symbol, value: Value, mutable: bool, deep_mutable: bool) -> Option<&Value> {
		self.storage.insert(symbol.clone(), SymbolValue {
			r#type: Type::from_value(&value),
			value, 
			mutable, 
			deep_mutable,
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