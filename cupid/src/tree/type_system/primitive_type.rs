use std::fmt::{Display, Formatter, Result as DisplayResult};
use std::borrow::Cow;
use crate::Type;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PrimitiveType {
	pub identifier: Cow<'static, str>,
}

impl PrimitiveType {
	pub const fn new_const(identifier: &'static str) -> Self {
		Self { identifier: Cow::Borrowed(identifier) }
	}
	pub fn new(identifier: &str) -> Self {
		Self { identifier: Cow::Owned(identifier.to_string()) }
	}
}

impl Type for PrimitiveType {}

impl Display for PrimitiveType {
	fn fmt(&self, f: &mut Formatter) -> DisplayResult {
		write!(f, "{}", self.identifier)
	}
}