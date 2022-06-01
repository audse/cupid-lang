use crate::*;

impl Analyze for Ident {
	fn analyze_names(&mut self, scope: &mut Env) -> Result<(), ASTErr> {
		let value = scope.get_symbol(self)?;
		self.attributes_mut().closure = value.attributes().closure;
		self.attributes.generics.set_symbols(scope);
		Ok(())
	}
	fn analyze_types(&mut self, scope: &mut Env) -> Result<(), ASTErr> {
		scope.use_closure(self.attributes.closure);

		for generic in self.attributes.generics.iter_mut() {
			if let Ok(type_val) = scope.get_type(generic) {
				generic.to_typed(type_val);
			}
		}

		scope.reset_closure();
		Ok(())
	}
}

impl TypeOf for Ident {
	fn type_of(&self, scope: &mut Env) -> Result<Type, ASTErr> {
		// if an ident for a type, e.g. `int`
		let symbol_value = scope.get_symbol(self)?;
		if let Some(value) = symbol_value.value {
			if let Val::Type(mut type_hint) = value.val.into_inner() {
				type_hint.unify_with(&self.attributes().generics)?;
				return Ok(type_hint);
			}
		}
		// get the type associated with the ident's value
		scope.get_type(&symbol_value.type_hint)
	}
}

impl UseAttributes for Ident {
    fn attributes(&self) -> &Attributes {
        &self.attributes
    }
    fn attributes_mut(&mut self) -> &mut Attributes {
        &mut self.attributes
    }
}