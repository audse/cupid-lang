use crate::*;

impl PreAnalyze for Method {
    #[trace]
	fn pre_analyze_scope(&mut self, scope: &mut Env) -> ASTResult<()> {
		self.set_closure_to(scope.add_closure(
			Some(self.name.to_owned()), 
			Context::Method
		));
		self.use_closure(scope);

		self.value.analyze_scope(scope)?;

		scope.reset_closure();
		Ok(())
	}
    #[trace]
	fn pre_analyze_names(&mut self, scope: &mut Env) -> ASTResult<()> {
		scope.set_symbol(&self.name, SymbolValue {
			value: Some(VFunction(Box::new(self.value.to_owned()))),
			type_hint: type_type().to_ident(), 
			mutable: false,
		});
		Ok(())
	}
}

impl Analyze for Method {
    #[trace]
	fn analyze_scope(&mut self, scope: &mut Env) -> ASTResult<()> {
		self.name.analyze_scope(scope)?;
		Ok(())
	}
    #[trace]
	fn analyze_names(&mut self, scope: &mut Env) -> ASTResult<()> {
		self.trace_define_method(scope);
		self.use_closure(scope);

		self.name.analyze_names(scope)?;
		self.value.analyze_names(scope)?;

		scope.reset_closure();
		Ok(())
	}
    #[trace]
	fn analyze_types(&mut self, scope: &mut Env) -> ASTResult<()> {
		self.use_closure(scope);

		self.name.analyze_types(scope)?;
		self.value.analyze_types(scope)?;

		scope.reset_closure();
    	Ok(())
	}
    #[trace]
	fn check_types(&mut self, scope: &mut Env) -> ASTResult<()> {
		self.use_closure(scope);

		self.value.check_types(scope)?;

		scope.reset_closure();
		Ok(())
	}
}

impl TypeOf for Method {
	fn type_of(&self, scope: &mut Env) -> ASTResult<Cow<Type>> {
		self.value.type_of(scope)
	}
}