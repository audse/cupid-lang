use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use crate::{Expression, Tree, Value, LexicalScope, Type, Token, ErrorHandler, MapErrorHandler};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map {
	pub entries: HashMap<Expression, (usize, Expression)>,
	pub r#type: Type,
	pub token: Token,
}

impl Map {
	
	pub fn make_scope(&self, scope: &mut LexicalScope) {
		scope.add();
		for (key, (index, value)) in self.entries.iter() {
			let value = value.resolve(scope);
			
			// create symbols for identifier-like keys, e.g. person.name
			if let Expression::Symbol(symbol) = key {
				let mutable = if let Some((_, mutable)) = scope.is_mutable(symbol) {
					mutable
				} else { 
					false 
				};
				let entry_value = Value::MapEntry(*index, Box::new(value.clone()));
				scope.create_symbol(symbol, entry_value, mutable, mutable);
			}
		}
	}
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
// impl PartialEq for Map {
// 	fn eq(&self, other: &Self) -> bool {
//     	self == other
// 	}
// }
// impl Eq for Map {}

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
		if !map_type.is_map() {
			return self.not_map_error(&map);
		}
		
		map.make_scope(scope, self.operator.clone(), false);
		
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
		
		if let Some((_, map_value)) = map.inner_map().unwrap().get(&property_term) {
			if let Some(function) = function_call {
				scope.pop();
				function
			} else {
				scope.pop();
				map_value.clone()
			}
		} else {
			scope.pop();
			self.no_property_error(&map, &property_term)
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
pub struct PropertyAssign {
	pub access: PropertyAccess,
	pub value: Box<Expression>,
	pub operator: Token,
}

impl Tree for PropertyAssign {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
		if let Expression::Symbol(map_symbol) = &*self.access.map {
			let new_value = crate::resolve_or_abort!(self.value, scope);
			// let new_value = self.value.resolve(scope);
			// let map = &self.access.map.resolve(scope);
			let map = crate::resolve_or_abort!(&self.access.map, scope);
			
			map.make_scope(scope, self.operator.clone(), false);
			let property = &self.access.term;
			let property_term = if let Expression::Symbol(property_symbol) = &**property {
				property_symbol.identifier.clone()
			} else {
				crate::resolve_or_abort!(property, scope)
			};
			
			if let Some(original_hashmap) = map.inner_map() {
				let mut new_hashmap = original_hashmap.clone();
				new_hashmap.entry(property_term).and_modify(|e| e.1 = new_value);
				
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