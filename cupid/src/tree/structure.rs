use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::Entry::Occupied;
use crate::{Expression, Tree, Value, LexicalScope, Scope, Type, Token, Symbol};

#[derive(Debug, Clone)]
pub struct Map {
	pub entries: HashMap<Expression, (usize, Expression)>,
	pub r#type: Type,
	pub token: Token,
}

impl Map {
	pub fn unexpected_type_error(&self) -> Value {
		Value::error(
			&self.token,
			format!("expected a dictionary, list, or tuple, not {}", self.r#type)
		)
	}
	pub fn unexpected_error(value: &Value, token: &Token) -> Value {
		Value::error(
			token, 
			format!("expected a dictionary, list, or tuple, not {}", Type::from_value(value))
		)
	}
	pub fn no_property_error(key: &Value, token: &Token) -> Value {
		Value::error(
			token, 
			format!("property `{}` doesn't exist on this object", key)
		)
	}
	pub fn unable_to_assign_error(key: &Value, value: &Value, token: &Token) -> Value {
		Value::error(
			token, 
			format!("unable to assign {} to property `{}`", key, value)
		)
	}
	// fn set_or_create_property(&mut self, symbol: &Symbol, value: Value) -> Value {
	// 	match self.inner_scope.set_symbol(symbol, value) {
	// 		Ok(new_value) => new_value.unwrap(),
	// 		Err(_) => {
	// 			self.inner_scope.create_symbol(symbol, value, self.mutable, self.deep_mutable).unwrap()
	// 		}
	// 	}
	// }
	// fn set(&mut self, scope: &mut LexicalScope, term: &Expression, new_value: &Expression, token: &Token) -> Value {
	// 	if let Occupied(entry) = self.entries.entry(term.clone()) {
	// 		let old_value = entry.into_mut();
	// 		*old_value = (old_value.0, new_value.clone());
	// 		return new_value.resolve(scope)
	// 	}
	// 	let term = term.resolve(scope);
	// 	Map::no_property_error(&term, token)
	// }
}

impl Tree for Map {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
		// TODO error handling
		let mut entries: HashMap<Value, (usize, Value)> = HashMap::new();
		for (key, (index, value)) in self.entries.iter() {
			let value = value.resolve(scope);
			
			// create symbols for identifier-like keys, e.g. person.name
			if let Expression::Symbol(symbol) = key {
				// self.set_or_create_property(symbol, value);
				// scope.create_symbol(symbol, value.clone(), false, false);
			}
			entries.insert(
				key.resolve(scope),
				(*index, value)
			);
		}
		match self.r#type {
			Type::Dictionary => Value::Dictionary(entries),
			Type::List => Value::List(entries),
			Type::Tuple => Value::Tuple(entries),
			_ => self.unexpected_type_error()
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
impl PartialEq for Map {
	fn eq(&self, other: &Self) -> bool {
    	self == other
	}
}
impl Eq for Map {}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct PropertyAccess {
	pub operator: Token,
	pub term: Box<Expression>,
	pub map: Box<Expression>,
}

impl Tree for PropertyAccess {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
		let map = self.map.resolve(scope);
		match map {
			Value::Dictionary(map)
			| Value::List(map)
			| Value::Tuple(map) => {
				let term = self.term.resolve(scope);
				let value = if let Some((_, value)) = map.get(&term) {
					value.clone()
				} else {
					Map::no_property_error(&term, &self.operator)
				};
				// remove the scope added by accessing the map
				// scope.pop();
				value
			},
			_ => {
				let term = self.term.resolve(scope);
				Map::unexpected_error(&term, &self.operator)
			}
		}
	}
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct PropertyAssign {
	pub access: PropertyAccess,
	pub value: Box<Expression>,
	pub operator: Token,
}

impl Tree for PropertyAssign {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
		let map = &self.access.map.resolve(scope);
		
		
		// if let Expression::Map(mut map) = Box::into_inner(map_exp.clone()) {
		// 	return map.set(scope, &*self.access.term, &*self.value, &self.operator)
		// }
		Map::unexpected_error(&map, &self.operator)
	}
}