use std::collections::hash_map::Entry::Vacant;
use crate::{Expression, Value, Tree, LexicalScope, SymbolFinder, ErrorHandler, Token, TypeKind, Symbol};

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Property {
	pub map: Box<Expression>,
	pub term: Box<Expression>,
	pub token: Token,
}

impl Tree for Property {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
		let map = crate::resolve_or_abort!(self.map, scope);
		
		let implementation = if let Expression::Symbol(map_symbol) = &*self.map {
			if let Some(type_kind) = scope.get_symbol_type(&map_symbol) {
				Some(type_kind)
			} else {
				None
			}
		} else {
			let type_value = Value::String(TypeKind::infer_name(&map));
			let type_kind = scope.get_symbol_from_value(&type_value);
			if let Some(Value::Type(type_kind)) = type_kind {
				Some(type_kind)
			} else {
				None
			}
		};
		
		if let Expression::FunctionCall(function_call) = &*self.term {
			if let Some(implement) = implementation {
				if let Some(fun) = implement.get_implemented_symbol(&function_call.fun.identifier) {
					scope.add(crate::ScopeContext::Block);
					
					let self_symbol = Symbol::new_string("self", self.token.clone());
					scope.create_symbol(&self_symbol, map, implement, true, true);
					scope.create_symbol(&function_call.fun, fun.clone(), TypeKind::new_function(), false, false);
					
					let final_val = function_call.resolve(scope);
					
					// mutate the original value
					if let Expression::Symbol(map_symbol) = &*self.map {
						let map_val = scope.get_symbol(&self_symbol).unwrap();
						match scope.set_symbol(map_symbol, map_val) {
							Ok(_) => (),
							Err(err) => {
								scope.pop();
								return err;
							}
						}
					}
					scope.pop();
					return final_val;
				}
			}
		}
		
		match &map {
			Value::Map(m) => {
				if let Expression::FunctionCall(function_call) = &*self.term {
					let fun_name = &function_call.fun;
					let function = if let Some(fun) = m.get(&fun_name.identifier) {
						fun.1.clone()
					} else {
						let term = crate::resolve_or_abort!(self.term, scope);
						return self.no_property_error(&map, &term);
					};
					scope.add(crate::ScopeContext::Block);
					scope.create_symbol(&function_call.fun, function.clone(), TypeKind::new_function(), false, false);
					let final_val = function_call.resolve(scope);
					scope.pop();
					final_val
					
				} else if let Expression::Symbol(symbol) = &*self.term {
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
					self.bad_array_access_error(&term)
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
	fn bad_array_access_error(&self, accessor: &Value) -> Value {
		self.error(format!(
			"type mismatch: array items can only be accessed with integers, not with {} ({})", 
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
		
		let map_symbol = if let Expression::Symbol(symbol) = &*self.property.map {
			symbol
		} else {
			return self.error_context(
				"cannot set property of a map before initialization",
				"try: variable_name.property_name = ..."
			);
		};
			
		match map {
			Value::Map(ref mut m) => {
				let term = if let Expression::Symbol(symbol) = &*self.property.term {
					symbol.identifier.clone()
				} else {
					crate::resolve_or_abort!(self.property.term, scope)
				};
				let entry = m
					.entry(term.clone())
					.and_modify(|entry| { entry.1 = value.clone() });
				
				if let Vacant(_) = entry {
					return self.no_property_error(&map, &term);
				}
				return match scope.set_symbol(&map_symbol, Value::Map(m.to_owned())) {
					Ok(val) => val.unwrap_or(self.unable_to_assign_error(map_symbol.get_identifier(), value)),
					Err(err) => err
				};
			},
			Value::Array(ref mut m) => {
				let term_value = crate::resolve_or_abort!(self.property.term, scope);
				let term = if let Value::Integer(i) = term_value {
					i as usize
				} else {
					return self.bad_array_access_error(&term_value);
				};
				if m.len() >= term {
					m[term] = Box::new(value.clone());
					return match scope.set_symbol(&map_symbol, Value::Array(m.to_owned())) {
						Ok(val) => val.unwrap_or(self.unable_to_assign_error(map_symbol.get_identifier(), value)),
						Err(err) => err
					};
				} else {
					return self.bad_array_index_error(m.len(), &term_value);
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
	fn bad_array_access_error(&self, accessor: &Value) -> Value {
		self.error(format!(
			"type mismatch: array items can only be accessed with integers, not with {} ({})", 
			TypeKind::infer(accessor), 
			accessor
		))
	}
	fn bad_array_index_error(&self, length: usize, accessor: &Value) -> Value {
		self.error(format!(
			"type mismatch: the index provided is out of bounds (array is size {length}, got {accessor}"
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