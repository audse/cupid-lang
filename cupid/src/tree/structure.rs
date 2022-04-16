use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use crate::{Expression, Tree, Value, LexicalScope, Type, Token, ErrorHandler, MapErrorHandler, Symbol, MAP_ENTRY};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map {
	pub entries: HashMap<Expression, (usize, Expression)>,
	pub r#type: Type,
	pub token: Token,
}

impl Tree for Map {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
		let mut entries: HashMap<Value, (usize, Value)> = HashMap::new();
		
		for (key, (index, value)) in self.entries.iter() {
			let value = value.resolve(scope);
			crate::abort_on_error!(value);
			
			let new_key = if let Expression::Symbol(symbol) = key {
				symbol.identifier.clone()
			} else {
				let key = key.resolve(scope);
				crate::abort_on_error!(key);
				key
			};
			entries.insert(new_key, (*index, value));
		}
		if self.r#type.is_map() {
			Value::wrap_map(&self.r#type, entries)
		} else {
			self.not_map_type_error(&self.r#type)
		}
	}
}

impl Hash for Map {
	fn hash<H: Hasher>(&self, state: &mut H) {
		for entry in self.entries.iter() {
			entry.hash(state)
		}
		self.r#type.hash(state);
		self.token.hash(state);
	}
}

impl ErrorHandler for Map {
	fn get_token(&self) -> &Token {
    	&self.token
	}
	fn get_context(&self) -> String {
		format!("creating a map of type {}", self.r#type)
	}
}
impl MapErrorHandler for Map {}


#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct PropertyAccess {
	pub operator: Token,
	pub term: Box<Expression>,
	pub map: Box<Expression>,
}

impl Tree for PropertyAccess {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
		let map = crate::resolve_or_abort!(self.map, scope);
		
		let map_type = Type::from(&map);
		let is_entry = map_type == MAP_ENTRY;
		
		if !map_type.is_map() && !is_entry {
			return self.not_map_error(&map);
		}
		
		map.make_scope(scope, self.operator.clone(), true);
		
		let deref_term = &*self.term;
		let mut function_call: Option<Value> = None;
		
		let property_term = if let Expression::Symbol(property_symbol) = deref_term {
			property_symbol.identifier.clone()
			
		} else if let Expression::FunctionCall(function) = deref_term {
			function_call = Some(function.resolve(scope));
			function.fun.identifier.clone()
			
		} else {
			crate::resolve_or_abort!(self.term, scope, { scope.pop(); })
		};
		
		let entry = if !is_entry {
			let inner_map = map.inner_map().unwrap();
			// try to get the term from the inner scope first, e.g. `object.property`
			if let Some((_, entry)) = inner_map.get(&property_term) {
				entry
			// if that fails, try to get the property from the outer scope, e.g.
			// my_var = 1; object.my_var
			} else {
				let term_value = crate::resolve_or_abort!(deref_term, scope);
				if let Some((_, entry)) = inner_map.get(&term_value) {
					entry
				} else {
					scope.pop();
					return self.no_property_error(&map, &property_term);
				}
			}
		} else {
			&map
		};
		
		if let Some(function) = function_call {
			scope.pop();
			function
		} else {
			scope.pop();
			entry.clone()
		}
	}
}

impl ErrorHandler for PropertyAccess {
	fn get_token(&self) -> &Token {
		&self.operator
	}
	fn get_context(&self) -> String {
		format!("accessing a property of map `{}`", self.map)
	}
}
impl MapErrorHandler for PropertyAccess {}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct InternalPropertyAccess {
	pub operator: Token,
	pub term: Box<Expression>,
}

impl Tree for InternalPropertyAccess {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
		if let Expression::Symbol(symbol) = &*self.term {
			let map_entry = crate::resolve_or_abort!(self.term, scope);
			if let Value::MapEntry(_, map_value) = map_entry {
				*map_value
			} else {
				self.undefined_error(symbol.get_identifier())
			}
		} else {
			let map_entry = crate::resolve_or_abort!(self.term, scope);
			if let Value::MapEntry(_, map_value) = map_entry {
				*map_value
			} else {
				self.error(format!("unable to find property {} in map", self.term))
			}
		}
	}
}

impl ErrorHandler for InternalPropertyAccess {
	fn get_token(&self) -> &Token {
		&self.operator
	}
	fn get_context(&self) -> String {
		format!("accessing property `{}` within self", self.term)
	}
}
impl MapErrorHandler for InternalPropertyAccess {}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct PropertyAssign {
	pub access: PropertyAccess,
	pub value: Box<Expression>,
	pub operator: Token,
}

impl Tree for PropertyAssign {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
		if let Expression::Symbol(map_symbol) = &*self.access.map {
			
			let new_value = crate::resolve_or_abort!(self.value, scope);
			let map = crate::resolve_or_abort!(&self.access.map, scope);
			
			map.make_scope(scope, self.operator.clone(), true);
			let property = &self.access.term;
			let property_term = if let Expression::Symbol(property_symbol) = &**property {
				property_symbol.identifier.clone()
			} else {
				crate::resolve_or_abort!(property, scope)
			};
			
			if let Some(original_hashmap) = map.inner_map() {
				let mut new_hashmap = original_hashmap.clone();
				
				if new_hashmap.contains_key(&property_term) {
					new_hashmap.entry(property_term).and_modify(|e| e.1 = new_value);
				} else {
					let property_value = crate::resolve_or_abort!(property, scope);
					new_hashmap.entry(property_value).and_modify(|e| e.1 = new_value);
				}
				
				match scope.set_symbol(map_symbol, Value::wrap_map(&Type::from(&map), new_hashmap)) {
					Ok(ok_val) => if let Some(val) = ok_val {
						scope.pop();
						return val;
					},
					Err(err_val) => {
						scope.pop();
						return err_val;
					}
				}
			} else {
				scope.pop();
				return self.not_map_error(&map);
			}
		}
		scope.pop();
		self.not_map_error(&self.access.map.resolve(scope))
	}
}

impl ErrorHandler for PropertyAssign {
	fn get_token(&self) -> &Token {
		&self.operator
	}
	fn get_context(&self) -> String {
		format!("assigning {} to property {} of map `{}`", self.value, self.access.term, self.access.map)
	}
}
impl MapErrorHandler for PropertyAssign {}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct InternalPropertyAssign {
	pub access: InternalPropertyAccess,
	pub value: Box<Expression>,
	pub operator: Token,
}

impl Tree for InternalPropertyAssign {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
		let new_value = crate::resolve_or_abort!(self.value, scope);

		let property = &self.access.term;
		let property_term = if let Expression::Symbol(property_symbol) = &**property {
			property_symbol.identifier.clone()
		} else {
			crate::resolve_or_abort!(property, scope)
		};
		
		let self_symbol = Symbol::new_string("self".to_string(), self.operator.clone());
		if let Some(original_hashmap) = scope.get_symbol(&self_symbol) {
			
			if let Some(mut new_hashmap) = original_hashmap.inner_map_clone() {
				new_hashmap.entry(property_term.clone()).and_modify(|e| e.1 = new_value);
				
				match scope.set_symbol(&self_symbol, Value::wrap_map(&Type::from(&original_hashmap), new_hashmap)) {
					Ok(ok_val) => if let Some(val) = ok_val {
						scope.pop();
						return val;
					},
					Err(err_val) => {
						scope.pop();
						return err_val;
					}
				}
				self.no_property_error(&self_symbol.identifier, &property_term)
				
			} else {
				self.not_map_error(&original_hashmap)
			}
		} else {
			self.undefined_error("self".to_string())
		}
	}
}

impl ErrorHandler for InternalPropertyAssign {
	fn get_token(&self) -> &Token {
		&self.operator
	}
	fn get_context(&self) -> String {
		format!("assigning {} to property {}` of self", self.value, self.access.term)
	}
}
impl MapErrorHandler for InternalPropertyAssign {}