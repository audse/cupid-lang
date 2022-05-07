use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as DisplayResult};
use std::hash::{Hash, Hasher};
use serde::{Serialize, Deserialize};
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructType {
	pub members: Vec<(SymbolNode, TypeKind)>,
	pub implementation: Implementation
}

impl StructType {
	pub fn is_map_equal(&self, other: &Value) -> bool {
		// todo
		match other {
			Value::Map(x) => {
				x.iter().all(|(key, (_, value))| {
					if let Some((_, member_type)) = self.members.iter().find(|(symbol, _)| &symbol.0.value == key)  {
						member_type.is_equal(value)
					} else {
						false
					}
				})
			},
			_ => false
		}
	}
}

impl Type for StructType {
	fn apply_arguments(&mut self, arguments: &[GenericType]) -> Result<(), String> {
		for (_, member) in self.members.iter_mut() {
			match member.apply_arguments(arguments) {
				Ok(_) => continue,
				Err(msg) => return Err(msg)
			}
		}
		Ok(())
	}
	fn convert_primitives_to_generics(&mut self, generics: &[GenericType]) {
    	for (_, member) in self.members.iter_mut() {
			member.convert_primitives_to_generics(generics)
		}
	}
	fn implement(&mut self, functions: HashMap<ValueNode, ValueNode>) -> Result<(), ()> {
    	self.implementation.implement(functions);
		Ok(())
	}
	fn find_function(&self, symbol: &SymbolNode, scope: &mut LexicalScope) -> Option<ValueNode> {
		self.implementation.find_function(symbol, scope)
	}
	fn implement_trait(&mut self, trait_symbol: SymbolNode, functions: HashMap<ValueNode, ValueNode>) -> Result<(), ()> { 
		let implementation = Implementation { functions, traits: HashMap::new(), };
		self.implementation.implement_trait(trait_symbol, implementation);
		Ok(())
	}
}

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