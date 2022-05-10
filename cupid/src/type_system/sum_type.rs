use std::fmt::{Display, Formatter, Result as DisplayResult};
use std::hash::{Hash, Hasher};
use serde::{Serialize, Deserialize};
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SumType<'src> {
	pub types: Vec<TypeKind<'src>>,
	pub implementation: Implementation<'src>
}

impl<'src> SumType<'src> {
	pub fn contains(&self, other: &Value) -> bool {
		self.types
			.iter()
			.any(|t| t.is_equal(other))
	}
}

impl<'src> Type for SumType<'src> {}

impl<'src> PartialEq for SumType<'src> {
	fn eq(&self, other: &Self) -> bool {
		self.types == other.types
	}
}

impl<'src> Eq for SumType<'src> {}

impl<'src> Hash for SumType<'src> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.types.hash(state);
	}
}

impl<'src> Display for SumType<'src> {
	fn fmt(&self, f: &mut Formatter) -> DisplayResult {
		let types: Vec<String> = self.types
			.iter()
			.map(|member| member.to_string())
			.collect();
		write!(f, "one of [{}]", types.join(", "))
	}
}