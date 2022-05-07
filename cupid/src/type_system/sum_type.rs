use std::fmt::{Display, Formatter, Result as DisplayResult};
use std::hash::{Hash, Hasher};
use serde::{Serialize, Deserialize};
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SumType {
	pub types: Vec<TypeKind>,
	pub implementation: Implementation
}

impl SumType {
	pub fn contains(&self, other: &Value) -> bool {
		self.types
			.iter()
			.any(|t| t.is_equal(other))
	}
}

impl Type for SumType {
	fn apply_arguments(&mut self, arguments: &[GenericType]) -> Result<(), String> {
		self.types.iter_mut().for_each(|t| { _ = t.apply_arguments(arguments); });
		Ok(())
	}
	fn convert_primitives_to_generics(&mut self, generics: &[GenericType]) {
		_ = self.types.iter_mut().map(|t| t.convert_primitives_to_generics(generics));
	}
}

impl PartialEq for SumType {
	fn eq(&self, other: &Self) -> bool {
		self.types == other.types
	}
}

impl Eq for SumType {}

impl Hash for SumType {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.types.hash(state);
	}
}

impl Display for SumType {
	fn fmt(&self, f: &mut Formatter) -> DisplayResult {
		let types: Vec<String> = self.types
			.iter()
			.map(|member| member.to_string())
			.collect();
		write!(f, "one of [{}]", types.join(", "))
	}
}