use crate::{Expression, Value, Tree, LexicalScope, ErrorHandler, Token, Type};

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
					let term = &symbol.identifier;
					m.get(&term).unwrap_or(&(0, self.no_property_error(&map, &term))).1.clone()
				} else {
					let term = crate::resolve_or_abort!(self.term, scope);
					m.get(&term).unwrap_or(&(0, self.no_property_error(&map, &term))).1.clone()
				}
			},
			Value::ProductMap(_, m) => {
				if let Expression::Symbol(symbol) = &*self.term {
					if let Some(val) = m.get(&symbol) {
						val.clone()
					} else {
						let term = crate::resolve_or_abort!(self.term, scope);
						self.no_property_error(&map, &term)
					}
				} else {
					let term = crate::resolve_or_abort!(self.term, scope);
					self.bad_map_access_error(&term)
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
			Type::from(accessor), 
			accessor
		))
	}
	fn bad_map_access_error(&self, accessor: &Value) -> Value {
		self.error(format!(
			"type mismatch: product type members can only be accessed with identifier names, not with {} ({})", 
			Type::from(accessor), 
			accessor
		))
	}
	fn not_accessible_error(&self, array: &Value, accessor: &Value) -> Value {
		self.error(format!(
			"type mismatch: {} ({}) is not an array or map, and cannot be accessed by {} ({})",
			array,
			Type::from(array),
			accessor,
			Type::from(accessor)
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