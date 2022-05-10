use std::hash::{Hash, Hasher};
use std::fmt::{Display, Formatter, Result as DisplayResult};
use std::borrow::Cow;
use serde::{Serialize, Deserialize};
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimitiveType<'src> {
	pub identifier: Cow<'src, str>,
	pub implementation: Implementation<'src>,
}

impl<'src> PrimitiveType<'src> {
	pub fn new(identifier: &str) -> Self {
		Self { 
			identifier: identifier.into(),
			implementation: Implementation::default()
		}
	}
}

impl<'src> Type for PrimitiveType<'src> {}

impl<'src> PartialEq for PrimitiveType<'src> {
	fn eq(&self, other: &Self) -> bool {
    	self.identifier == other.identifier
	}
}

impl<'src> Eq for PrimitiveType<'src> {}

impl<'src> Hash for PrimitiveType<'src> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.identifier.hash(state);
	}
}

impl<'src> Display for PrimitiveType<'src> {
	fn fmt(&self, f: &mut Formatter) -> DisplayResult {
		write!(f, "{}", self.identifier)
	}
}