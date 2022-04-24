use std::hash::{Hash, Hasher};
use crate::{TypeKind, Type, GenericType, ErrorHandler, Token, Expression, Tree, Value};

#[derive(Debug, Clone)]
pub struct MapType {
	pub key_type: Box<TypeKind>,
	pub value_type: Box<TypeKind>,
}

impl Type for MapType {
	fn apply_arguments(&mut self, arguments: &[GenericType]) -> Result<(), String> {
		match &*self.key_type {
			TypeKind::Generic(key_generic) => {
				let arg = arguments.iter().find(|arg| arg.identifier == key_generic.identifier);
				if let Some(arg) = arg {
					if let Some(arg) = &arg.type_value {
						self.key_type = arg.clone();
					} else {
						return Err(format!("generic unresolved: no argument was provided for map key ({key_generic})"));
					}
				} else {
					return Err(format!("generic unresolved: no argument was provided for map key ({key_generic})"));
				}
			},
			_ => {}
		};
		match &*self.value_type {
			TypeKind::Generic(value_generic) => {
				let arg = arguments.iter().find(|arg| arg.identifier == value_generic.identifier);
				if let Some(arg) = arg {
					if let Some(arg) = &arg.type_value {
						self.value_type = arg.clone();
					} else {
						return Err(format!("generic unresolved: no argument was provided for map value ({value_generic})"));
					}
				} else {
					return Err(format!("generic unresolved: no argument was provided for map value ({value_generic})"));
				}
			},
			_ => {}
		};
		Ok(())
	}
	fn convert_primitives_to_generics(&mut self, generics: &[GenericType]) {
		let generic_identifiers: Vec<String> = generics.iter().map(|g| g.identifier.to_string()).collect();
		match &*self.key_type {
			TypeKind::Primitive(primitive) => {
				if generic_identifiers.contains(&primitive.identifier.to_string()) {
					self.key_type = Box::new(TypeKind::Generic(GenericType::new(&primitive.identifier, None)));
				}
			},
			_ => ()
		};
		match &*self.value_type {
			TypeKind::Primitive(primitive) => {
				if generic_identifiers.contains(&primitive.identifier.to_string()) {
					self.value_type = Box::new(TypeKind::Generic(GenericType::new(&primitive.identifier, None)));
				}
			},
			_ => ()
		};
	}
}

impl PartialEq for MapType {
	fn eq(&self, other: &Self) -> bool {
		let key_match = match (&*self.key_type, &*other.key_type) {
			(TypeKind::Generic(GenericType { identifier: _, type_value: _ }), _)
			| (_, TypeKind::Generic(GenericType { identifier: _, type_value: _ })) => true,
			(k, v) => k == v
		};
		let value_match = match (&*self.value_type, &*other.value_type) {
			(TypeKind::Generic(GenericType { identifier: _, type_value: _ }), _)
			| (_, TypeKind::Generic(GenericType { identifier: _, type_value: _ })) => true,
			(k, v) => k == v
		};
		key_match && value_match
	}
}

impl Eq for MapType {}

impl Hash for MapType {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.key_type.hash(state);
		self.value_type.hash(state);
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MapTypeHint {
	pub token: Token,
	pub key_type: Box<Expression>,
	pub value_type: Box<Expression>,
}

impl Tree for MapTypeHint {
	fn resolve(&self, scope: &mut crate::LexicalScope) -> Value {
		let key_type = crate::resolve_or_abort!(self.key_type, scope);
		let value_type = crate::resolve_or_abort!(self.value_type, scope);
		if let (Value::Type(key_type), Value::Type(value_type)) = (&key_type, &value_type) {
			let map_type = TypeKind::Map(MapType { 
				key_type: Box::new(key_type.clone()),
				value_type: Box::new(value_type.clone()) 
			});
			Value::Type(map_type)
		} else {
			self.error(format!("expected a type hint for map keys and values, not {key_type}: {value_type}"))
		}
	}
}

impl ErrorHandler for MapTypeHint {
	fn get_token(&self) -> &Token {
		&self.token
	}
	fn get_context(&self) -> String {
		format!("map type with keys of type {} and values of type {}", self.key_type, self.value_type)
	}
}