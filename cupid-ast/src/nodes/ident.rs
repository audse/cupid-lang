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
	pub fn new<S: Into<Str>>(name: S, generics: GenericList) -> Self {
		Self {
			name: name.into(),
			attributes: Attributes {
				generics,
				source: None,
				closure: 0,
			}
		}
	}
	pub fn new_name<S: Into<Str>>(name: S) -> Self {
		Self::new(name, GenericList(vec![]))
	}
	pub fn src(&self) -> usize {
		self.attributes.source.unwrap_or(0)
	}
}

impl PartialEq for Ident {
	fn eq(&self, other: &Self) -> bool {
		self.can_unify(other)
		// self.name == other.name 
		// && self.attributes.generics.len() == other.attributes.generics.len()
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