use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::fmt::{Display, Formatter, Result as DisplayResult};
use std::borrow::Cow;
use crate::{Type, Symbol, Value};

#[derive(Debug, Clone)]
pub struct PrimitiveType {
	pub identifier: Cow<'static, str>,
	pub implement: HashMap<Value, Value>,
}

impl PrimitiveType {
	pub fn new(identifier: &str) -> Self {
		Self { 
			identifier: Cow::Owned(identifier.to_string()),
			implement: HashMap::new()
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