use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SumType {
	pub types: Vec<TypeHintNode>,
	pub implementation: Implementation
}

impl SumType {
	pub fn contains(&self, other: &ValueNode) -> bool {
		self.types
			.iter()
			.any(|t| t == &TypeKind::infer_id(&other).unwrap())
	}
}

impl Type for SumType {}

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