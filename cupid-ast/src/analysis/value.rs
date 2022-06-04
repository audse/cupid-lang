use crate::*;

impl PreAnalyze for Value {}

impl Analyze for Value {
	fn analyze_scope(&mut self, scope: &mut Env) -> ASTResult<()> {
		match self.val.inner_mut() {
			Val::Function(function) => function.analyze_scope(scope),
			Val::Type(type_val) => type_val.analyze_scope(scope),
			Val::Trait(trait_val) => trait_val.analyze_scope(scope),
			_ => Ok(())
		}
	}
	fn analyze_names(&mut self, scope: &mut Env) -> ASTResult<()> {
    	match self.val.inner_mut() {
			Val::Function(function) => function.analyze_names(scope),
			Val::Type(type_val) => type_val.analyze_names(scope),
			Val::Trait(trait_val) => trait_val.analyze_names(scope),
			_ => Ok(())
		}
	}
	fn analyze_types(&mut self, scope: &mut Env) -> ASTResult<()> {
		match self.val.inner_mut() {
			Val::Function(function) => function.analyze_types(scope)?,
			Val::Type(type_val) => type_val.analyze_types(scope)?,
			Val::Trait(trait_val) => trait_val.analyze_types(scope)?,
			_ => ()
		};
		self.val.to_typed(self.val.type_of(scope)?.into_owned());
		Ok(())
	}
}

impl UseAttributes for Value {
	fn attributes(&self) -> &Attributes { 
		&self.attributes
	}
	fn attributes_mut(&mut self) -> &mut Attributes { 
		&mut self.attributes
	}
}

impl TypeOf for Value {
	fn type_of(&self, scope: &mut Env) -> ASTResult<Cow<'_, Type>> {
		match &self.val {
			IsTyped(_, t) => Ok(t.into()),
			Untyped(val) => Ok(infer_type(val, scope)?.into())
		}
	}
}