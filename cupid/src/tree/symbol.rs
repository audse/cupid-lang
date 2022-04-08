use std::fmt::{
	Display,
	Formatter,
	Result,
};
use crate::{
	CupidValue,
	Tree,
	CupidScope,
};

#[derive(Debug, Hash, Eq, Clone)]
pub struct CupidSymbol {
	pub identifier: CupidValue,
	pub mutable: bool,
	pub deep_mutable: bool,
}

impl PartialEq for CupidSymbol {
	fn eq(&self, other: &Self) -> bool {
		self.identifier == other.identifier
	}
}

impl Display for CupidSymbol {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{} (mutable: {}, deep_mutable: {})", self.identifier, self.mutable, self.deep_mutable)
	}
}

impl Tree for CupidSymbol {
	fn resolve(&self, scope: &mut CupidScope) -> CupidValue {
		if let Some(value) = scope.get_symbol(self) {
			return value.clone();
		}
		return CupidValue::None;
	}
}

impl CupidSymbol {
	pub fn new(identifier: String, mutable: bool, deep_mutable: bool) -> Self {
		Self {
			identifier: CupidValue::String(identifier),
			mutable,
			deep_mutable
		}
	}
	
	pub fn clone(&self) -> Self {
		Self {
			identifier: self.identifier.clone(),
			mutable: self.mutable,
			deep_mutable: self.deep_mutable
		}
	}
}
