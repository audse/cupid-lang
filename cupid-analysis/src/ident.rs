use crate::*;

impl PreAnalyze for Ident {}

impl Analyze for Ident {
	fn analyze_names(&mut self, scope: &mut Env) -> ASTResult<()> {
		let value = scope.get_symbol(self)?;
		let closure = value.attributes().closure;
		self.attributes_mut().closure = closure;
		scope.use_closure(closure);
		
		self.attributes.generics.set_symbols(scope);

		scope.reset_closure();
		Ok(())
	}
	fn analyze_types(&mut self, scope: &mut Env) -> ASTResult<()> {
		scope.use_closure(self.attributes.closure);

		for generic in self.attributes.generics.iter_mut() {
			scope.trace(format!("Finding type of generic `{generic}`"));
			if let Ok(type_val) = scope.get_type(generic) {
				generic.to_typed(type_val);
			}
		}

		scope.reset_closure();

		Ok(())
	}
}

impl TypeOf for Ident {
	fn type_of(&self, scope: &mut Env) -> ASTResult<Cow<Type>> { 
		// if an ident for a type, e.g. `int`
		let symbol_value = scope.get_symbol(self)?;
		if let Some(value) = symbol_value.value {
			match value {
				VType(mut type_val) => {
					type_val.unify_with(&self.attributes.generics)?;
					return Ok(type_val.into());
				},
				_ => ()
			}
		}
		// get the type associated with the ident's value
		Ok(scope.get_type(&symbol_value.type_hint)?.into())
	}
}

impl TypeOf for Typed<Ident> {
	fn type_of(&self, scope: &mut Env) -> ASTResult<Cow<Type>> { 
		match self {
			Self::Typed(_, t) => Ok(t.into()),
			Self::Untyped(v) => v.type_of(scope)
		}
	}
}