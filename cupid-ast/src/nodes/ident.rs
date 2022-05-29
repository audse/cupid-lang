use std::hash::{Hash, Hasher};
use crate::*;

build_struct! {
	#[derive(Debug, Clone, Default, Tabled)]
	pub IdentBuilder => pub Ident {
		pub name: Str,
		pub attributes: Attributes
	}
}

impl Ident {
	pub fn new(name: &'static str, generics: GenericList) -> Self {
		Self {
			name: Cow::Borrowed(name),
			attributes: Attributes {
				generics,
				source: None,
				closure: 0,
			}
		}
	}
	pub fn new_name(name: &'static str) -> Self {
		Self::new(name, GenericList(vec![]))
	}
	pub fn src(&self) -> usize {
		self.attributes.source.unwrap_or(0)
	}
	pub fn set_generic_symbols(&self, scope: &mut Env) -> Result<(), (Source, ErrCode)> {
		self.attributes.generics.set_symbols(scope)
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
	}
}

pub trait ToIdent {
	fn to_ident(&self) -> Ident;
}