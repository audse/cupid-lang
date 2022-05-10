use std::hash::{Hash, Hasher};
use serde::{Serialize, Deserialize};
use std::fmt::{Display, Formatter, Result as DisplayResult};
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionType<'src> {
	pub return_type: Box<TypeKind<'src>>,
	pub param_types: Vec<TypeKind<'src>>, // TODO
	pub implementation: Implementation<'src>,
}

impl<'src> Type for FunctionType<'src> {}

impl<'src> PartialEq for FunctionType<'src> {
	fn eq(&self, other: &Self) -> bool {
		// TODO params as well?
		match (&*self.return_type, &*other.return_type) {
			(TypeKind::Generic(GenericType	{ identifier: _, type_value: _ }), _) => true,
			(_, TypeKind::Generic(GenericType	{ identifier: _, type_value: _ })) => true,
			_ => self.return_type == other.return_type
		}
	}
}

impl<'src> Eq for FunctionType<'src> {}

impl<'src> Hash for FunctionType<'src> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.return_type.hash(state);
	}
}

impl<'src> Display for FunctionType<'src> {
	fn fmt(&self, f: &mut Formatter) -> DisplayResult {
		let param_types: Vec<String> = self.param_types.iter().map(|p| p.to_string()).collect();
		write!(f, "{:8} [{}: {}] {}", "fun", self.return_type, param_types.join(", "), self.implementation)
	}
}