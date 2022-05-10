use std::fmt::{Display, Formatter, Result as DisplayResult};
use std::hash::{Hash, Hasher};
use serde::{Serialize, Deserialize};
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructType<'src> {
	pub members: Vec<(SymbolNode<'src>, TypeKind<'src>)>,
	pub implementation: Implementation<'src>
}

impl<'src> StructType<'src> {
	pub fn is_map_equal(&self, other: &Value) -> bool {
		// todo
		match other {
			Value::Map(x) => {
				x.iter().all(|(key, (_, value))| {
					if let Some((_, member_type)) = self.members.iter().find(|(symbol, _)| &symbol.0.value == &key.value)  {
						member_type.is_equal(&value.value)
					} else {
						false
					}
				})
			},
			_ => false
		}
	}
}

impl<'src> Type for StructType<'src> {}

impl<'src> PartialEq for StructType<'src> {
	fn eq(&self, other: &Self) -> bool {
		self.members == other.members
	}
}

impl<'src> Eq for StructType<'src> {}

impl<'src> Hash for StructType<'src> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.members.hash(state);
	}
}

impl<'src> Display for StructType<'src> {
	fn fmt(&self, f: &mut Formatter) -> DisplayResult {
		let members: Vec<String> = self.members
			.iter()
			.map(|(symbol, member)| format!("{symbol}: {member}"))
			.collect();
		write!(f, "[{}]", members.join(", "))
	}
}