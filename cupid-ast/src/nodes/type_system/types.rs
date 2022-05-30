use crate::*;

pub type Str = Cow<'static, str>;
pub type TypedIdent = (Str, Ident);

build_struct! { 
	#[derive(Debug, Clone, Default, Tabled)]
	pub TypeBuilder => pub Type {
		pub name: Ident,
		pub fields: FieldSet,

		#[tabled(display_with = "fmt_vec")]
		pub traits: Vec<Ident>,

		#[tabled(display_with = "fmt_vec")]
		pub methods: Vec<Method>,
		pub base_type: BaseType,
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Display, Tabled)]
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
	pub fn is_string(&self) -> bool {
		matches!(self.base_type, BaseType::Primitive(Cow::Borrowed("string"))) 
	}
}

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

impl From<Type> for Value {
	fn from(t: Type) -> Self {
		Value::build()
			.attributes(t.attributes().to_owned())
			.val(IsTyped(t.into(), TYPE.to_owned()))
			.build()
	}
}