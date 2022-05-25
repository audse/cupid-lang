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
				generics: GenericParams(generics),
				source: None,
				closure: 0,
			}
		}
	}
	pub fn src(&self) -> usize {
		self.attributes.source.unwrap_or(0)
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

impl UseAttributes for Ident {
	fn attributes(&mut self) -> &mut Attributes { &mut self.attributes }
}

impl Analyze for Ident {
	fn analyze_names(&mut self, scope: &mut Env) -> Result<(), (Source, ErrCode)> {
    	scope.get_symbol(self)?;
		Ok(())
	}
}

impl TypeOf for Ident {
	fn type_of(&self, scope: &mut Env) -> Result<Type, (Source, ErrCode)> {
		// if an ident for a type, e.g. `int`
		let mut symbol_value = scope.get_symbol(&*self)?;
		if let Some(value) = &mut symbol_value.value {
			if let Val::Type(type_hint) = &mut *value.0 {
				type_hint.name.attributes.generics.apply(self.attributes.generics.to_owned());
				return Ok(type_hint.to_owned());
			}
		}
		// get the type associated with the ident's value
		scope.get_type(&symbol_value.type_hint)
	}
}

pub trait ToIdent {
	fn to_ident(&self) -> Ident;
}

impl std::fmt::Display for Ident {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let g = fmt_list!(self.attributes.generics.0);
		let g = fmt_if_nonempty!(g, format!(" [{}]", g.join(", ")));
		write!(f, "{}{g}", self.name)
	}
}