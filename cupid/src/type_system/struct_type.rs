use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructType {
	pub members: Vec<(ValueNode, TypeHintNode)>,
	pub implementation: Implementation
}

impl StructType {
	pub fn is_map_equal(&self, other: &ValueNode) -> bool {
		// todo
		match &other.value {
			Value::Map(x) => {
				x.iter().all(|(key, (_, value))| {
					if let Some((_, member_type)) = self.members.iter().find(|(symbol, _)| &symbol.value == &key.value)  {
						member_type == &TypeKind::infer_id(&value).unwrap()
					} else {
						false
					}
				})
			},
			_ => false
		}
	}
}

impl Type for StructType {}

impl PartialEq for StructType {
	fn eq(&self, other: &Self) -> bool {
		self.members == other.members
	}
}

impl Eq for StructType {}

impl Hash for StructType {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.members.hash(state);
	}
}

impl Display for StructType {
	fn fmt(&self, f: &mut Formatter) -> DisplayResult {
		let members: Vec<String> = self.members
			.iter()
			.map(|(symbol, member)| format!("{symbol}: {member}"))
			.collect();
		write!(f, "[{}]", members.join(", "))
	}
}