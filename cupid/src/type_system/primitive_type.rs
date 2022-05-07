use std::collections::HashMap;
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
			implementation: Implementation::new()
		}
	}
}

impl Type for PrimitiveType {
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