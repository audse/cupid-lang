use std::hash::{Hash, Hasher};
use serde::{Serialize, Deserialize};
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionType {
	pub return_type: Box<TypeKind>,
}

impl Type for FunctionType {
	fn apply_arguments(&mut self, arguments: &[GenericType]) -> Result<(), String> {
		if arguments.len() > 0 {
			match &*self.return_type {
				TypeKind::Generic(return_generic) => {
					let arg = arguments.iter().find(|arg| arg.identifier == return_generic.identifier);
					if let Some(arg) = arg {
						if let Some(arg) = &arg.type_value {
							self.return_type = arg.clone();
							return Ok(())
						} else {
							return Err(format!("generic mismatch (function): the return type is generic, and a generic was provided (expected a concrete type)"))
						}
					} else {
						return Err(format!("generic mismatch (function): the return type is generic, but no matching type argument was provided. expected [{return_generic}: ...]"))
					}
				},
				_ => {}
			}
		}
		Ok(())
	}
	fn convert_primitives_to_generics(&mut self, generics: &[GenericType]) {
		match &*self.return_type {
			TypeKind::Primitive(primitive) => {
				let generic_identifiers: Vec<String> = generics.iter().map(|g| g.identifier.to_string()).collect();
				if generic_identifiers.contains(&primitive.identifier.to_string()) {
					self.return_type = Box::new(TypeKind::Generic(GenericType::new(&primitive.identifier, None)));
				}
			},
			_ => ()
		}
	}
}

impl PartialEq for FunctionType {
	fn eq(&self, other: &Self) -> bool {
		match (&*self.return_type, &*other.return_type) {
			(TypeKind::Generic(GenericType	{ identifier: _, type_value: _ }), _) => true,
			(_, TypeKind::Generic(GenericType	{ identifier: _, type_value: _ })) => true,
			_ => self.return_type == other.return_type
		}
	}
}

impl Eq for FunctionType {}

impl Hash for FunctionType {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.return_type.hash(state);
	}
}