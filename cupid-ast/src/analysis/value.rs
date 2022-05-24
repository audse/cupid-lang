use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Val {
	Array(Vec<Val>),
	Boolean(bool),
	Char(char),
	Decimal(i32, u32),
	Function(crate::Function),
	Integer(i32),
	None,
	String(Cow<'static, str>),
	Tuple(Vec<Val>),
	Type(crate::Type),
}

impl Default for Val {
	fn default() -> Self { Self::None }
}

impl TypeOf for Val {
	fn type_of(&self, _scope: &mut Env) -> Result<Type, ErrCode> {
    	infer_type(self)
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Value(pub Typed<Val>, pub Attributes);

impl Analyze for Value {
	fn analyze_names(&mut self, scope: &mut Env) -> Result<(), ErrCode> {
    	match &mut self.0.inner_mut() {
			Val::Function(function) => function.analyze_names(scope),
			Val::Type(type_val) => type_val.analyze_names(scope),
			_ => Ok(())
		}
	}
	fn analyze_types(&mut self, scope: &mut Env) -> Result<(), ErrCode> {
		match &mut self.0.inner_mut() {
			Val::Function(function) => function.analyze_types(scope)?,
			Val::Type(type_val) => type_val.analyze_types(scope)?,
			_ => ()
		};
		self.0.to_typed(self.0.type_of(scope)?);
		Ok(())
	}
}

impl UseAttributes for Value {
	fn attributes(&mut self) -> &mut Attributes { &mut self.1 }
}

impl TypeOf for Value {
	fn type_of(&self, _scope: &mut Env) -> Result<Type, ErrCode> {
    	Ok(self.0.get_type().to_owned())
	}
}