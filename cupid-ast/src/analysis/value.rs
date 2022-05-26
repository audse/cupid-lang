use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Display, Unwrap)]
pub enum Val {
	#[display(fmt = "{:?}", "fmt_list!(_0)")]
	Array(Vec<Val>),
	Boolean(bool),
	Char(char),
	#[display(fmt = "{_0}.{_1}")]
	Decimal(i32, u32),
	Function(crate::Function),
	Integer(i32),
	None,
	String(Cow<'static, str>),
	#[display(fmt = "{:?}", "fmt_list!(_0)")]
	Tuple(Vec<Val>),
	Type(crate::Type),
	Trait(crate::Trait),
	BuiltinPlaceholder,
}

impl Default for Val {
	fn default() -> Self { Self::None }
}

impl TypeOf for Val {
	fn type_of(&self, _scope: &mut Env) -> Result<Type, (Source, ErrCode)> {
    	infer_type(self).map_err(|code| (0, code))
	}
}

build_struct! {
	#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
	pub ValueBuilder => pub Value {
		pub val: Typed<Val>, 
		pub attributes: Attributes
	}
}

impl Analyze for Value {
	fn analyze_names(&mut self, scope: &mut Env) -> Result<(), (Source, ErrCode)> {
    	match self.val.inner_mut() {
			Val::Function(function) => function.analyze_names(scope),
			Val::Type(type_val) => type_val.analyze_names(scope),
			Val::Trait(trait_val) => trait_val.analyze_names(scope),
			_ => Ok(())
		}
	}
	fn analyze_types(&mut self, scope: &mut Env) -> Result<(), (Source, ErrCode)> {
		match self.val.inner_mut() {
			Val::Function(function) => function.analyze_types(scope)?,
			Val::Type(type_val) => type_val.analyze_types(scope)?,
			Val::Trait(trait_val) => trait_val.analyze_types(scope)?,
			_ => ()
		};
		self.val.to_typed(self.val.type_of(scope)?);
		Ok(())
	}
}

impl UseAttributes for Value {
	fn attributes(&mut self) -> &mut Attributes { &mut self.attributes }
}

impl TypeOf for Value {
	fn type_of(&self, _scope: &mut Env) -> Result<Type, (Source, ErrCode)> {
    	Ok(self.val.get_type().to_owned())
	}
}

impl ValueBuilder {
	pub fn typed_val(mut self, val: Val, val_type: Type) -> Self {
		self.val = Typed::Typed(val, val_type);
		self
	}
	pub fn untyped_val(mut self, val: Val) -> Self {
		self.val = Typed::Untyped(val);
		self
	}
	pub fn none(mut self) -> Self {
		self.val = IsTyped(Val::None, NOTHING.to_owned());
		self
	}
	pub fn builtin(mut self) -> Self {
		self.val = IsTyped(Val::BuiltinPlaceholder, NOTHING.to_owned());
		self
	}
}