use std::collections::HashMap;
use crate::{Symbol, Value, Type};


pub struct LexicalScope {
	pub scopes: Vec<Scope>
}

impl LexicalScope {
	pub fn last(&self) -> Option<&Scope> {
		self.scopes.last()
	}
	pub fn last_mut(&mut self) -> Option<&mut Scope> {
		self.scopes.last_mut()
	}
	pub fn add(&mut self) -> &Scope {
		let scope;
		if let Some(last) = self.last_mut() {
			scope = last.make_sub_scope();
		} else {
			scope = Scope::new(None)
		}
		self.scopes.push(scope);
		self.last().unwrap()
	}
	pub fn pop(&mut self) -> Option<Scope> {
		self.scopes.pop()
	}
	pub fn get_symbol(&self, symbol: &Symbol) -> Option<Value> {
		for scope in self.scopes.iter().rev() {
			if let Some(value) = scope.get_symbol(symbol) {
				return Some(value);
			}
		}
		None
	}
	pub fn set_symbol(&mut self, symbol: &Symbol, value: Value) -> Result<Option<Value>, Value> {
		let current_scope = self.scopes.iter_mut().rev().find(|scope| 
			if let Some(_) = scope.get_symbol(symbol) {
				true
			} else {
				false
			}
		);
		if let Some(scope) = current_scope {
			return scope.set_symbol(symbol, value);
		}
		Err(not_found_error(symbol))
	}
	pub fn create_symbol(&mut self, symbol: &Symbol, value: Value, mutable: bool, deep_mutable: bool) -> Option<Value> {
		if let Some(scope) = self.last_mut() {
			return scope.create_symbol(symbol, value, mutable, deep_mutable);
		}
		None
	}
}

#[derive(Debug, Clone)]
pub struct Scope {
	pub storage: HashMap<Symbol, SymbolValue>,
	pub parent: Option<Box<Scope>>,
	pub closed: bool,
}

#[derive(Debug, Clone)]
pub struct SymbolValue {
	pub value: Value,
	pub r#type: Type,
	pub mutable: bool,
	pub deep_mutable: bool,
}

impl Scope {

	pub fn new(parent: Option<Box<Scope>>) -> Self {
		Self {
			storage: HashMap::new(),
			parent,
			closed: false,
		}
	}

	pub fn get_symbol(&self, symbol: &Symbol) -> Option<Value> {
		if let Some(stored_symbol) = self.storage.get(symbol) {
			return Some(stored_symbol.value.clone());
		}
		// if let Some(stored_symbol) = self.get_parent_symbol(symbol) {
		// 	return Some(stored_symbol);
		// }
		None
	}

	pub fn set_symbol(&mut self, symbol: &Symbol, value: Value) -> Result<Option<Value>, Value> {
		if let Some(stored_value) = self.storage.get_mut(symbol) {
			if let Some(immutable_assign_error) = assign_to_immutable_error(symbol, &stored_value) {
				return Err(immutable_assign_error)
			}
			if let Some(type_mismatch_error) = assign_type_mismatch_error(symbol, &stored_value, &value) {
				return Err(type_mismatch_error)
			}
			stored_value.value = value;
			return Ok(self.get_symbol(symbol))
		}
		// if let Some(closed_scope_error) = self.closed_scope_error(symbol) {
		// 	return Err(closed_scope_error)
		// }
		Err(not_found_error(symbol))
		// self.set_parent_symbol(symbol, value)
	}
	
	pub fn create_symbol(&mut self, symbol: &Symbol, value: Value, mutable: bool, deep_mutable: bool) -> Option<Value> {
		self.storage.insert(symbol.clone(), SymbolValue {
			r#type: Type::from_value(&value),
			value, 
			mutable, 
			deep_mutable,
		});
		self.get_symbol(symbol)
	}

	// pub fn get_parent_symbol(&self, symbol: &Symbol) -> Option<Value> {
	// 	if let Some(parent) = &self.parent {
	// 		return parent.get_symbol(symbol);
	// 	}
	// 	None
	// }
	// 
	// pub fn set_parent_symbol(&mut self, symbol: &Symbol, value: Value) -> Result<Option<Value>, Value> {
	// 	if let Some(parent) = &mut self.parent {
	// 		return parent.set_symbol(symbol, value);
	// 	}
	// 	Err(not_found_error(symbol))
	// }

	pub fn make_sub_scope(&mut self) -> Self {
		Scope::new(Some(Box::new(self.clone())))
	}
	
	fn closed_scope_error(&self, symbol: &Symbol) -> Option<Value> {
		if !self.closed {
			if let Some(_) = &self.parent {
				return None;
			}
			return Some(Value::error(
				&symbol.1[0],
				format!(
					"`{symbol}` could not be found in the current scope",
					symbol = symbol.get_identifier()
				)
			));
		} else {
			Some(Value::error(
				&symbol.1[0],
				format!(
					"`{symbol}` is outside the current scope and can't be changed",
					symbol = symbol.get_identifier()
				)
			))
		}
	}
}

fn assign_type_mismatch_error(symbol: &Symbol, stored_symbol: &SymbolValue, assign_value: &Value) -> Option<Value> {
	let assign_type = Type::from_value(assign_value);
	let stored_type = Type::from_value(&stored_symbol.value);
	if assign_type == stored_type {
		None
	} else {
		Some(Value::error(
			&symbol.1[0],
			format!(
				"type mismatch: `{symbol}` ({stored_type}) can't be assigned `{value}` ({assign_type})",
				assign_type = assign_type,
				symbol = symbol.get_identifier(),
				stored_type = stored_type,
				value = assign_value,
			)
		))
	}
}

fn assign_to_immutable_error(symbol: &Symbol, stored_value: &SymbolValue) -> Option<Value> {
	if !stored_value.mutable {
		return Some(Value::error(
			&symbol.1[0],
			format!(
				"variable `{symbol}` is immutable and cannot be reassigned",
				symbol = symbol.get_identifier(),
			)
		))
	}
	None
}

fn not_found_error(symbol: &Symbol) -> Value {
	Value::error(
		&symbol.1[0],
		format!(
			"variable `{symbol}` is not defined",
			symbol = symbol.get_identifier(),
		)
	)
}
