use std::hash::{Hash, Hasher};
use serde::{Serialize, Deserialize};
use std::fmt::{Display, Formatter, Result as DisplayResult};
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AliasType {
	pub true_type: Box<TypeKind>,
	pub implementation: Implementation
}

impl Type for AliasType {}

impl PartialEq for AliasType {
	fn eq(&self, other: &Self) -> bool {
		self.true_type == other.true_type
	}
}

impl Eq for AliasType {}

impl Hash for AliasType {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.true_type.hash(state);
	}
}

impl Display for AliasType {
	fn fmt(&self, f: &mut Formatter) -> DisplayResult {
		write!(f, "alias of {}", self.true_type)
	}
}