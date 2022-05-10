use std::hash::{Hash, Hasher};
use serde::{Serialize, Deserialize};
use std::fmt::{Display, Formatter, Result as DisplayResult};
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AliasType<'src> {
	pub true_type: Box<TypeKind<'src>>,
	pub implementation: Implementation<'src>
}

impl<'src> Type for AliasType<'src> {}

impl<'src> PartialEq for AliasType<'src> {
	fn eq(&self, other: &Self) -> bool {
		self.true_type == other.true_type
	}
}

impl<'src> Eq for AliasType<'src> {}

impl<'src> Hash for AliasType<'src> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.true_type.hash(state);
	}
}

impl<'src> Display for AliasType<'src> {
	fn fmt(&self, f: &mut Formatter) -> DisplayResult {
		write!(f, "alias of {}", self.true_type)
	}
}