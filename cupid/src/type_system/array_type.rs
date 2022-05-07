use std::hash::{Hash, Hasher};
use serde::{Serialize, Deserialize};
use std::fmt::{Display, Formatter, Result as DisplayResult};
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArrayType {
	pub element_type: Box<TypeKind>,
	pub implementation: Implementation,
}

impl Type for ArrayType {
	fn apply_arguments(&mut self, arguments: &[GenericType]) -> Result<(), String> {
		if let Some(element_type) = TypeKind::replace_generic(&*self.element_type, &arguments[0]) {
			self.element_type = element_type;
		} else {
			let element_type = &self.element_type;
			let generic = &arguments[0];
			return Err(format!("either the element type ({element_type}) of this array is not generic, or the generic given  ({generic}) doesn't match"));
		}
		Ok(())
	}
	fn convert_primitives_to_generics(&mut self, generics: &[GenericType]) {
		if let TypeKind::Primitive(primitive) = &*self.element_type {
			if generics.iter().any(|x| x.identifier == primitive.identifier) {
				self.element_type = Box::new(TypeKind::Generic(GenericType::new(&primitive.identifier, None)));
			}
		}
	}
}

impl PartialEq for ArrayType {
	fn eq(&self, other: &Self) -> bool {
		match &*self.element_type {
			TypeKind::Generic(GenericType	{ identifier: _, type_value: _ }) => true,
			_ => self.element_type == other.element_type
		}
	}
}

impl Eq for ArrayType {}

impl Hash for ArrayType {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.element_type.hash(state);
	}
}

impl Display for ArrayType {
	fn fmt(&self, f: &mut Formatter) -> DisplayResult {
		write!(f, "array [{}]", self.element_type)
	}
}