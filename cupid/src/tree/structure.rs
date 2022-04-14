use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use crate::{Expression, Tree, Value, LexicalScope, Type, Token};

#[derive(Debug, Clone)]
pub struct Map {
	pub entries: HashMap<Expression, (usize, Expression)>,
	pub r#type: Type,
	pub token: Token,
}

impl Map {
	pub fn error(&self, message: String) -> Value {
		Value::error(&self.token, message)
	}
	pub fn unexpected_type_error(&self) -> Value {
		self.error(format!("expected a dictionary, list, or tuple, not {}", self.r#type))
	}
	pub fn unexpected_error(value: &Value, token: &Token) -> Value {
		Value::error(
			token, 
			format!("expected a dictionary, list, or tuple, not {}", Type::from(value))
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
}

impl Tree for Map {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
		// TODO error handling
		scope.add();
		let mut entries: HashMap<Value, (usize, Value)> = HashMap::new();
		for (key, (index, value)) in self.entries.iter() {
			let value = value.resolve(scope);
			
			// create symbols for identifier-like keys, e.g. person.name
			if let Expression::Symbol(symbol) = key {
				let mutable = if let Some((_, mutable)) = scope.is_mutable(symbol) {
					mutable
				} else { false };
				let entry_value = Value::MapEntry(*index, Box::new(value.clone()));
				scope.create_symbol(symbol, entry_value, mutable, mutable);
				entries.insert(
					symbol.identifier.clone(),
					(*index, value)
				);
			} else {
				entries.insert(
					key.resolve(scope),
					(*index, value)
				);
			}
		}
		if self.r#type.is_map() {
			Value::wrap_map(&self.r#type, entries)
		} else {
			self.unexpected_type_error()
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
		if let Some(value) = map.inner_map() {
			let mut token: &Token = &self.operator;
			let property_term = if let Expression::Symbol(property_symbol) = &*self.term {
				token = &property_symbol.token;
				property_symbol.identifier.clone()
			} else {
				self.term.resolve(scope)
			};
			if let Some((_, map_value)) = value.get(&property_term) {
				scope.pop();
				map_value.clone()
			} else {
				scope.pop();
				Map::no_property_error(&property_term, token)
			}
		} else {
			let term = self.term.resolve(scope);
			scope.pop();
			Map::unexpected_error(&term, &self.operator)
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
		scope.add();
		if let Expression::Symbol(map_symbol) = &*self.access.map {
			let new_value = self.value.resolve(scope);
			let map = &self.access.map.resolve(scope);
			let property = &self.access.term;
			let property_term = if let Expression::Symbol(property_symbol) = &**property {
				property_symbol.identifier.clone()
			} else {
				property.resolve(scope)
			};
			
			if let Some(original_hashmap) = map.inner_map() {
				let mut new_hashmap = original_hashmap.clone();
				new_hashmap.entry(property_term.clone()).and_modify(|e| e.1 = new_value);
				
				match scope.set_symbol(map_symbol, Value::wrap_map(&Type::from(map), new_hashmap)) {
					Ok(ok_val) => if let Some(val) = ok_val {
						scope.pop();
						return val
					},
					Err(err_val) => {
						scope.pop();
						return err_val
					}
				}
			} else {
				scope.pop();
				return Map::unexpected_error(map, &self.operator)
			}
		}
		scope.pop();
		Map::unexpected_error(&self.access.map.resolve(scope), &self.operator)
	}
}