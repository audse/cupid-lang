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

impl Type for MapType {}

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
		write!(f, "{:8} {}, {} {}", "map", self.key_type, self.value_type, self.implementation)
	}
}
