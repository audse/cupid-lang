use std::hash::{Hash, Hasher};
use std::fmt::{Display, Formatter, Result as DisplayResult};
use serde::{Serialize, Deserialize};
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapType {
	pub key_type: Box<TypeKind>,
	pub value_type: Box<TypeKind>,
	pub implementation: Implementation,
}

impl Type for MapType {
	fn apply_arguments(&mut self, arguments: &[GenericType]) -> Result<(), String> {
		if let TypeKind::Generic(key_generic) = &*self.key_type {
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
		}
		if let TypeKind::Generic(value_generic) = &*self.value_type {
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
		}
		Ok(())
	}
	fn convert_primitives_to_generics(&mut self, generics: &[GenericType]) {
		let generic_identifiers: Vec<String> = generics.iter().map(|g| g.identifier.to_string()).collect();
		if let TypeKind::Primitive(primitive) = &*self.key_type {
			if generic_identifiers.contains(&primitive.identifier.to_string()) {
				self.key_type = Box::new(TypeKind::Generic(GenericType::new(&primitive.identifier, None)));
			}
		}
		if let TypeKind::Primitive(primitive) = &*self.value_type {
			if generic_identifiers.contains(&primitive.identifier.to_string()) {
				self.value_type = Box::new(TypeKind::Generic(GenericType::new(&primitive.identifier, None)));
			}
		}
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

impl Display for MapType {
	fn fmt(&self, f: &mut Formatter) -> DisplayResult {
		write!(f, "map [{}, {}]", self.key_type, self.value_type)
	}
}
