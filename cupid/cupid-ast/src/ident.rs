use std::hash::{Hash, Hasher};
use crate::*;

build_struct! {
	#[derive(Debug, Clone, Default)]
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
				typ: None
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
		self.type_eq(other)
	}
}

impl Eq for Ident {}

impl Hash for Ident {
	fn hash<H: Hasher>(&self, state: &mut H) {
    	self.name.hash(state);
	}
}

impl From<&'static str> for Ident {
	fn from(name: &'static str) -> Self {
		Self::build().name(name.into()).build()
	}
}

impl From<Str> for Ident {
	fn from(name: Str) -> Self {
		Self::build().name(name).build()
	}
}

pub trait ToIdent {
	fn to_ident(&self) -> Ident;
}

impl UseAttributes for Ident {
    fn attributes(&self) -> &Attributes {
        &self.attributes
    }
    fn attributes_mut(&mut self) -> &mut Attributes {
        &mut self.attributes
    }
}