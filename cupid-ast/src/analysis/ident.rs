use std::hash::{Hash, Hasher};
use crate::*;

#[derive(Debug, Clone, Default)]
pub struct Ident {
	pub name: Str,
	pub attributes: Attributes
}

impl Ident {
	pub fn new(name: &'static str, generics: Vec<GenericParam>) -> Self {
		Self {
			name: Cow::Borrowed(name),
			attributes: Attributes {
				generics,
				source: None,
				closure: 0,
			}
		}
	}
}

impl PartialEq for Ident {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name 
			&& self.attributes.generics == other.attributes.generics
	}
}

impl Eq for Ident {}

impl Hash for Ident {
	fn hash<H: Hasher>(&self, state: &mut H) {
    	self.name.hash(state);
		self.attributes.hash(state);
	}
}

impl UseAttributes for Ident {
	fn attributes(&mut self) -> &mut Attributes { &mut self.attributes }
}

impl Analyze for Ident {
	fn analyze_names(&mut self, scope: &mut Env) -> Result<(), ErrCode> {
    	scope.get_symbol(self)?;
		Ok(())
	}
}