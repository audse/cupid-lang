use std::fmt::{Display, Formatter, Result as DisplayResult};
use std::borrow::Cow;
use serde::{Serialize, Deserialize};
use std::hash::{Hash, Hasher};
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericType {
	pub identifier: Cow<'static, str>,
	pub type_value: Option<Box<TypeKind>>
}

impl GenericType {
	pub const fn new_const(identifier: &'static str) -> Self {
		Self { identifier: Cow::Borrowed(identifier), type_value: None }
	}
	pub fn new(identifier: &str, type_value: Option<Box<TypeKind>>) -> Self {
		Self { identifier: Cow::Owned(identifier.to_string()), type_value }
	}
}

impl Type for GenericType {}

impl PartialEq for GenericType {
	fn eq(&self, other: &Self) -> bool {
		self.identifier == other.identifier
	}
}

impl Eq for GenericType {}

impl Hash for GenericType {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.identifier.hash(state);
		self.type_value.hash(state);
	}
}

impl Display for GenericType {
	fn fmt(&self, f: &mut Formatter) -> DisplayResult {
		write!(f, "<{}>", self.identifier)
	}
}