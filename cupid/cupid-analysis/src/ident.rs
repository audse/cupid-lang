use crate::*;

impl PreAnalyze for Ident {}

impl Analyze for Ident {
    #[trace]
	fn analyze_names(&mut self, scope: &mut Env) -> ASTResult<()> {
		let value = scope.get_symbol(self)?;
		self.set_closure_to(value.attributes().closure);
		self.use_closure(scope);
		self.set_generic_symbols(scope)?;
		scope.reset_closure();
		Ok(())
	}
    #[trace]
	fn analyze_types(&mut self, scope: &mut Env) -> ASTResult<()> {
		self.use_closure(scope);

		// find concrete types for each generic
		for generic in self.attributes.generics.iter_mut() {
			generic.trace_find_generic_type(scope);
			// if there is a concrete type, unify the generic with the resolved type
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
		let symbol_value = scope.get_symbol(self)?;
		// get the type associated with the ident's value
		Ok(scope.get_type(&symbol_value.type_hint)?.into())
	}
	fn type_of_hint(&self, scope: &mut Env) -> ASTResult<Cow<Type>> {
		let mut type_value = scope.get_type(self)?;
		type_value.unify_with(&self.attributes.generics).ast_result(self)?;
		Ok(type_value.into())
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

trait SetGenericSymbols {
	 fn set_generic_symbols(&self, scope: &mut Env) -> ASTResult<()>;
}

impl<T: UseAttributes> SetGenericSymbols for T {
	fn set_generic_symbols(&self, scope: &mut Env) -> ASTResult<()> {
		self.attributes().generics.iter().try_for_each(|generic| {
			if scope.get_type(generic).is_err() {
				scope.set_symbol(generic, generic_symbol_value(generic))?;
			}
			Ok(())
		})?;
		Ok(())
	}
}

fn generic_symbol_value(generic: &Typed<Ident>) -> SymbolValue {
	SymbolValue { 
		value: Some(VType(
			Type::build()
				.name(generic.to_owned().into_inner())
				.build()
		)),
		type_hint: Type::type_ty().into_ident(),
		mutable: false,
	}
}