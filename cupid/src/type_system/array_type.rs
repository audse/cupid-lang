use std::hash::{Hash, Hasher};
use serde::{Serialize, Deserialize};
use std::fmt::{Display, Formatter, Result as DisplayResult};
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArrayType<'src> {
	pub element_type: Box<TypeKind<'src>>,
	pub implementation: Implementation<'src>,
}

impl<'src> Type for ArrayType<'src> {
	fn apply_args(&mut self, args: Vec<TypeKind>) -> Result<(), &str> {
		if let TypeKind::Generic(_) = &*self.element_type {
			if !args.is_empty() {
				self.element_type = Box::new(args[0].to_owned());
			}
			Ok(())
		} else if args.is_empty() {
			Ok(())
		} else {
			Err("array element type is not generic")
		}
	}
}

impl<'src> PartialEq for ArrayType<'src> {
	fn eq(&self, other: &Self) -> bool {
		match &*self.element_type {
			TypeKind::Generic(GenericType	{ identifier: _, type_value: _ }) => true,
			_ => self.element_type == other.element_type
		}
	}
}

impl<'src> Eq for ArrayType<'src> {}

impl<'src> Hash for ArrayType<'src> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.element_type.hash(state);
	}
}

impl<'src> Display for ArrayType<'src> {
	fn fmt(&self, f: &mut Formatter) -> DisplayResult {
		write!(f, "array [{}] {}", self.element_type, self.implementation)
	}
}