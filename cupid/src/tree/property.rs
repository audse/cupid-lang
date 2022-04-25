use std::collections::hash_map::Entry::Vacant;
use crate::{Expression, Value, Tree, LexicalScope, SymbolFinder, ErrorHandler, Token, TypeKind};

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Property {
	pub map: Box<Expression>,
	pub term: Box<Expression>,
	pub token: Token,
}

impl Tree for Property {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
		let map = crate::resolve_or_abort!(self.map, scope);
		match &map {
			Value::Map(m) => {
				if let Expression::Symbol(symbol) = &*self.term {
					if let Some(val) = m.get(&symbol.identifier) {
						val.1.clone()
					} else {
						let term = crate::resolve_or_abort!(self.term, scope);
						self.no_property_error(&map, &term)
					}
				} else {
					let term = crate::resolve_or_abort!(self.term, scope);
					if let Some(val) = m.get(&term) {
						val.1.clone()
					} else {
						let term = crate::resolve_or_abort!(self.term, scope);
						self.no_property_error(&map, &term)
					}
				}
			},
			Value::Array(m) => {
				let term = crate::resolve_or_abort!(self.term, scope);
				if let Value::Integer(i) = term {
					*(m[i as usize]).clone()
				} else {
					self.bad_access_error(&term)
				}
			},
			_ => {
				let term = crate::resolve_or_abort!(self.term, scope);
				self.not_accessible_error(&map, &term)
			}
		}
	}
}

impl ErrorHandler for Property {
	fn get_token(&self) -> &crate::Token {
		&self.token
	}
	fn get_context(&self) -> String {
		format!("Accessing property {} of {}", self.term, self.map)
	}
}

impl Property {
	fn bad_access_error(&self, accessor: &Value) -> Value {
		self.error(format!(
			"type mismatch: array items can only be accessed with integers, not with {} ({})", 
			TypeKind::infer(accessor), 
			accessor
		))
	}
	fn bad_map_access_error(&self, accessor: &Value) -> Value {
		self.error(format!(
			"type mismatch: type members can only be accessed with identifier names, not with {} ({})", 
			TypeKind::infer(accessor), 
			accessor
		))
	}
	fn not_accessible_error(&self, array: &Value, accessor: &Value) -> Value {
		self.error(format!(
			"type mismatch: {} ({}) is not an array or map, and cannot be accessed by {} ({})",
			array,
			TypeKind::infer(array),
			accessor,
			TypeKind::infer(accessor)
		))
	}
	fn no_property_error(&self, map: &Value, accessor: &Value) -> Value {
		self.error(format!(
			"undefined: {} doesn't have property {}",
			map,
			accessor
		))
	}
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct PropertyAssign {
	pub property: Property,
	pub value: Box<Expression>,
	pub operator: Token,
}

impl Tree for PropertyAssign {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
		let value = crate::resolve_or_abort!(self.value, scope);
		let mut map = crate::resolve_or_abort!(self.property.map, scope);
		match map {
			Value::Map(ref mut m) => {
				if let Expression::Symbol(symbol) = &*self.property.term {
					let entry = m
						.entry(symbol.identifier.clone())
						.and_modify(|entry| { entry.1 = value.clone() });
					
					if let Vacant(_) = entry {
						let term = crate::resolve_or_abort!(self.property.term, scope);
						return self.no_property_error(&map, &term);
					} else {
						if let Expression::Symbol(symbol) = &*self.property.map {
							return match scope.set_symbol(&symbol, Value::Map(m.to_owned())) {
								Ok(val) => val.unwrap_or(self.unable_to_assign_error(symbol.get_identifier(), value)),
								Err(err) => err
							};
						}
					}
				} else {
					let term = crate::resolve_or_abort!(self.property.term, scope);
					let entry = m
						.entry(term.clone())
						.and_modify(|entry| { entry.1 = value.clone() });
					
					if let Vacant(_) = entry {
						let term = crate::resolve_or_abort!(self.property.term, scope);
						return self.no_property_error(&map, &term);
					} else {
						if let Expression::Symbol(symbol) = &*self.property.map {
							return match scope.set_symbol(&symbol, Value::Map(m.to_owned())) {
								Ok(val) => val.unwrap_or(self.unable_to_assign_error(symbol.get_identifier(), value)),
								Err(err) => err
							};
						}
					}
				}
			},
			Value::Array(ref mut m) => {
				let term = crate::resolve_or_abort!(self.property.term, scope);
				if let Value::Integer(i) = term {
					let i = i as usize;
					if m.len() >= i {
						m[i] = Box::new(value.clone());
						if let Expression::Symbol(symbol) = &*self.property.map {
							return match scope.set_symbol(&symbol, Value::Array(m.to_owned())) {
								Ok(val) => val.unwrap_or(self.unable_to_assign_error(symbol.get_identifier(), value)),
								Err(err) => err
							};
						} else {
							return self.bad_access_error(&term);
						}
					} else {
						return self.bad_access_error(&term);
					}
				} else {
					return self.bad_access_error(&term);
				}
			},
			_ => {}
		};
		Value::None
	}
}


impl ErrorHandler for PropertyAssign {
	fn get_token(&self) -> &crate::Token {
		&self.operator
	}
	fn get_context(&self) -> String {
		format!("Assigning {} to property {} of {}", self.value, self.property.term, self.property.map)
	}
}

impl PropertyAssign {
	fn bad_access_error(&self, accessor: &Value) -> Value {
		self.error(format!(
			"type mismatch: array items can only be accessed with integers, not with {} ({})", 
			TypeKind::infer(accessor), 
			accessor
		))
	}
	fn bad_map_access_error(&self, accessor: &Value) -> Value {
		self.error(format!(
			"type mismatch: type members can only be accessed with identifier names, not with {} ({})", 
			TypeKind::infer(accessor), 
			accessor
		))
	}
	fn not_accessible_error(&self, array: &Value, accessor: &Value) -> Value {
		self.error(format!(
			"type mismatch: {} ({}) is not an array or map, and cannot be accessed by {} ({})",
			array,
			TypeKind::infer(array),
			accessor,
			TypeKind::infer(accessor)
		))
	}
	fn no_property_error(&self, map: &Value, accessor: &Value) -> Value {
		self.error(format!(
			"undefined: {} doesn't have property {}",
			map,
			accessor
		))
	}
}