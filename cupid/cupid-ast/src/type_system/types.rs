use crate::*;

pub type TypedIdent = (Str, Ident);

build_struct! { 
	#[derive(Debug, Clone, Default)]
	pub TypeBuilder => pub Type {
		pub name: Ident,
		pub fields: FieldSet,
		pub traits: Vec<Ident>,
		pub methods: Vec<Method>,
		pub base_type: BaseType,
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Display)]
pub enum BaseType {
	Primitive(Str),
	Array,
	Function,
	Sum,
	Struct,
	Alias,
	None
}

impl PartialEq for Type {
	fn eq(&self, other: &Self) -> bool {
		match (&self.base_type, &other.base_type) {
			(BaseType::Sum, _) => self.fields.find(&other.name).is_some(),
			(_, BaseType::Sum) => other.fields.find(&self.name).is_some(),
			_ => self.name == other.name
		}
	}
}

impl Eq for Type {}

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
	pub fn into_ident(self) -> Ident {
		self.name
	}
	pub fn is_array(&self) -> bool { 
		self.base_type == BaseType::Array 
	}
	pub fn is_int(&self) -> bool {
		matches!(self.base_type, BaseType::Primitive(Cow::Borrowed("int"))) 
	}
	pub fn is_function(&self) -> bool { 
		self.base_type == BaseType::Function 
	}
}

impl ToIdent for Type {
	fn to_ident(&self) -> Ident {
    	self.name.to_owned()
	}
}

impl UseAttributes for Type {
	fn attributes(&self) -> &Attributes { 
		self.name.attributes() 
	}
	fn attributes_mut(&mut self) -> &mut Attributes { 
		self.name.attributes_mut() 
	}
}

impl From<Type> for std::borrow::Cow<'_, Type> {
	fn from(t: Type) -> Self {
		std::borrow::Cow::Owned(t)
	}
}

impl<'t> From<&'t Type> for std::borrow::Cow<'t, Type> {
	fn from(t: &'t Type) -> Self {
		std::borrow::Cow::Borrowed(t)
	}
}

impl<'t> From<&'t mut Type> for std::borrow::Cow<'t, Type> {
	fn from(t: &'t mut Type) -> Self {
		std::borrow::Cow::Borrowed(t)
	}
}

impl Type {
	pub fn type_ty() -> Self {
		Self::build().primitive("type!").build()
	}
	pub fn trait_ty() -> Self {
		Self::build().primitive("trait!").build()
	}
	pub fn none() -> Self {
		Self::build().primitive("none").build()
	}
}
