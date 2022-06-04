use crate::*;

static DEFAULT_ATTR: Attributes = Attributes {
	closure: 0,
	source: None,
	generics: GenericList(vec![])
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Unwrap, Tabled)]
pub enum Val {
	Array(Vec<Val>),
	Boolean(bool),
	Char(char),
	Decimal(i32, u32),
	Function(Box<crate::Function>),
	Integer(i32),
	None,
	String(Cow<'static, str>),
	Tuple(Vec<Val>),
	Type(crate::Type),
	Trait(crate::Trait),
	BuiltinPlaceholder,
}

impl Default for Val {
	fn default() -> Self { Self::None }
}

impl TypeOf for Val {
	fn type_of(&self, scope: &mut Env) -> ASTResult<Cow<'_, Type>> {
    	Ok(infer_type(self, scope)?.into())
	}
}

impl UseAttributes for Val {
	fn attributes(&self) -> &Attributes {
		&DEFAULT_ATTR
	}
	fn attributes_mut(&mut self) -> &mut Attributes {
		panic!()
	}
}

build_struct! {
	#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Tabled)]
	pub ValueBuilder => pub Value {
		pub val: Typed<Val>, 

        #[tabled(skip)]
		pub attributes: Attributes
	}
}

impl From<Typed<Val>> for Value {
	fn from(v: Typed<Val>) -> Self {
		Value::build().val(v).build()
	}
}

impl From<Val> for Value {
	fn from(v: Val) -> Self {
		Value::build().val(Untyped(v)).build()
	}
}

impl From<&Typed<Val>> for Value {
	fn from(v: &Typed<Val>) -> Self {
		Value::build().val(v.to_owned()).build()
	}
}

impl From<&Val> for Value {
	fn from(v: &Val) -> Self {
		Value::build().val(Untyped(v.to_owned())).build()
	}
}