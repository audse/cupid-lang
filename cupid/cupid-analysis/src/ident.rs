use crate::*;

impl PreAnalyze for Ident {}

impl Analyze for Ident {
    #[trace]
	fn analyze_names(&mut self, scope: &mut Env) -> ASTResult<()> {
		let value = scope.get_symbol(self)?;
		scope.trace(&value.attributes().closure);
		self.set_closure_to(value.attributes().closure);
		self.use_closure(scope);
		
		self.attributes.generics.set_symbols(scope);

		scope.reset_closure();
		Ok(())
	}
    #[trace]
	fn analyze_types(&mut self, scope: &mut Env) -> ASTResult<()> {
		// self.use_closure(scope);

		for generic in self.attributes.generics.iter_mut() {
			generic.trace_find_generic_type(scope);
			if let Ok(type_val) = scope.get_type(generic) {
				generic.to_typed(type_val);
			}
		}
		// scope.reset_closure();
		Ok(())
	}
}

impl TypeOf for Ident {
	fn type_of(&self, scope: &mut Env) -> ASTResult<Cow<Type>> { 
		let symbol_value = scope.get_symbol(self)?;
		// get the type associated with the ident's value
		Ok(scope.get_type(&symbol_value.type_hint)?.into())
	}
	fn type_of_hint(&self, scope: &mut Env) -> ASTResult<Cow<Type>> {
		let symbol_value = scope.get_symbol(self)?;
		match symbol_value.value {
			Some(value) => match value {
				VType(mut type_val) => {
					type_val.unify_with(&self.attributes.generics)?;
					Ok(type_val.into())
				},
				x => x.to_err(ERR_EXPECTED_TYPE)
			},
			None => symbol_value.to_err(ERR_EXPECTED_TYPE)
		}
	}
}

impl TypeOf for Typed<Ident> {
	fn type_of(&self, scope: &mut Env) -> ASTResult<Cow<Type>> { 
		match self {
			Self::Typed(_, t) => Ok(t.into()),
			Self::Untyped(v) => v.type_of(scope)
		}
	}
	fn type_of_hint(&self, scope: &mut Env) -> ASTResult<Cow<Type>> {
		match self {
			Self::Typed(_, t) => Ok(t.into()),
			Self::Untyped(v) => v.type_of_hint(scope)
		}
	}
}