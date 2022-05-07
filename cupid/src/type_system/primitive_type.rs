use std::hash::{Hash, Hasher};
use std::fmt::{Display, Formatter, Result as DisplayResult};
use std::borrow::Cow;
use serde::{Serialize, Deserialize};
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimitiveType {
	pub identifier: Cow<'static, str>,
	pub implementation: Implementation,
}

impl PrimitiveType {
	pub fn new(identifier: &str) -> Self {
		Self { 
			identifier: Cow::Owned(identifier.to_string()),
			implementation: Implementation::default()
		}
	}
}

impl Type for PrimitiveType {}

impl PartialEq for PrimitiveType {
	fn eq(&self, other: &Self) -> bool {
    	self.identifier == other.identifier
	}
}

impl Eq for PrimitiveType {}

impl Hash for PrimitiveType {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.identifier.hash(state);
	}
}

impl Display for PrimitiveType {
	fn fmt(&self, f: &mut Formatter) -> DisplayResult {
		write!(f, "{}", self.identifier)
	}
}