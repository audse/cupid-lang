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
	pub base_type: BaseType, // right now, just used to know which types can be indexed like array
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BaseType {
	Primitive(Str),
	Array,
	Function,
	Sum,
	None
}

impl PartialEq for Type {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name
	}
}

impl Eq for Type {}

impl Default for Type {
	fn default() -> Self { NOTHING.to_owned() }
}

impl Default for BaseType {
	fn default() -> Self { Self::None }
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
			base_type: BaseType::Primitive(Cow::Borrowed(name)),
		}
	}
	const fn new(name: &'static str, generics: Vec<GenericParam>, fields: FieldSet, base_type: BaseType) -> Self {
		Self {
			name: Ident {
				name: Cow::Borrowed(name),
				attributes: Attributes {
					generics: GenericParams(generics),
					source: None,
					closure: 0,
				}
			},
			fields,
			traits: vec![],
			methods: vec![],
			base_type,
		}
	}
	pub fn to_ident(&self) -> Ident {
		self.name.to_owned()
	}
	pub fn into_ident(self) -> Ident {
		self.name
	}
	pub fn is_array(&self) -> bool { self.base_type == BaseType::Array }
	pub fn is_int(&self) -> bool {
		matches!(self.base_type, BaseType::Primitive(Cow::Borrowed("int"))) 
	}
	pub fn is_function(&self) -> bool { self.base_type == BaseType::Function }
	pub fn is_string(&self) -> bool {
		matches!(self.base_type, BaseType::Primitive(Cow::Borrowed("string"))) 
	}
}

impl UseAttributes for Type {
	fn attributes(&mut self) -> &mut Attributes { &mut self.name.attributes }
}

impl Analyze for Type {} // TODO

impl ToIdent for Type {
	fn to_ident(&self) -> Ident {
    	self.name.to_owned()
	}
}

impl From<Type> for Val {
	fn from(t: Type) -> Val {
		Val::Type(t)
	}
}

impl std::fmt::Display for Type {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let traits = fmt_list!(self.traits, ", ");
		let methods = fmt_list!(self.methods, ", ");
		write!(f, "(type {} = [{}], [{traits}], [{methods}])", self.name, self.fields)
	}
}
