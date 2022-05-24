#![allow(dead_code)]
use crate::*;

pub type Str = Cow<'static, str>;
pub type TypedIdent = (Str, Ident);

#[derive(Debug, Clone)]
pub struct Type {
	pub name: Ident,
	pub fields: FieldSet,
	pub traits: Vec<Ident>,
	pub methods: Vec<Type>,
}

impl PartialEq for Type {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name
	}
}

impl Eq for Type {}

impl Default for Type {
	fn default() -> Self {
    	NOTHING.to_owned()
	}
}

impl Hash for Type {
	fn hash<H: Hasher>(&self, state: &mut H) {
    	self.name.hash(state);
		self.fields.hash(state);
		self.traits.hash(state);
		self.methods.hash(state);
	}
}

impl Type {
	pub fn primitive(name: &'static str) -> Self {
		Type {
			name: Ident {
				name: Cow::Borrowed(name),
				attributes: Attributes::default()
			},
			fields: FieldSet::Empty,
			traits: vec![],
			methods: vec![],
		}
	}
	const fn new(name: &'static str, generics: Vec<GenericParam>, fields: FieldSet) -> Self {
		Self {
			name: Ident {
				name: Cow::Borrowed(name),
				attributes: Attributes {
					generics,
					source: None,
					closure: 0,
				}
			},
			fields,
			traits: vec![],
			methods: vec![],
		}
	}
	pub fn to_ident(&self) -> Ident {
		self.name.to_owned()
	}
	pub fn into_ident(self) -> Ident {
		self.name
	}
}

impl UseAttributes for Type {
	fn attributes(&mut self) -> &mut Attributes { &mut self.name.attributes }
}

impl Analyze for Type {} // TODO