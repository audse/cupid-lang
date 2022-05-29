use crate::*;

impl Analyze for Value {
	fn analyze_scope(&mut self, scope: &mut Env) -> Result<(), (Source, ErrCode)> {
		match self.val.inner_mut() {
			Val::Function(function) => function.analyze_scope(scope),
			Val::Type(type_val) => type_val.analyze_scope(scope),
			Val::Trait(trait_val) => trait_val.analyze_scope(scope),
			_ => Ok(())
		}
	}
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