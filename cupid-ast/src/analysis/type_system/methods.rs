use crate::*;

impl PreAnalyze for Method {
	fn pre_analyze_scope(&mut self, scope: &mut Env) -> ASTResult<()> {
		let closure = scope.add_closure(
			Some(self.name.to_owned()), 
			Context::Method
		);
		self.attributes_mut().closure = closure;
		Ok(())
	}
	fn pre_analyze_names(&mut self, scope: &mut Env) -> ASTResult<()> {
		scope.set_symbol(&self.name, SymbolValue {
			value: Some(Value { 
				val: Untyped(Val::Function(Box::new(self.value.to_owned()))),
				attributes: self.name.attributes.to_owned() 
			}),
			type_hint: TYPE.to_ident(), 
			mutable: false,
		});
		Ok(())
	}
}

impl Analyze for Method {
	fn analyze_scope(&mut self, scope: &mut Env) -> ASTResult<()> {
		self.name.analyze_scope(scope)?;

		scope.use_closure(self.attributes().closure);
		self.value.analyze_scope(scope)?;
		scope.reset_closure();
		Ok(())
	}
	fn analyze_names(&mut self, scope: &mut Env) -> ASTResult<()> {
		scope.trace(format!("Defining method {}", self.name));
		scope.use_closure(self.attributes().closure);

		self.name.attributes.generics.set_symbols(scope);
		self.value.analyze_names(scope)?;

		scope.reset_closure();
		Ok(())
	}
	fn analyze_types(&mut self, scope: &mut Env) -> ASTResult<()> {
		scope.use_closure(self.attributes().closure);

		self.name.analyze_types(scope)?;
		self.value.analyze_types(scope)?;

		scope.reset_closure();
    	Ok(())
	}
	fn check_types(&mut self, scope: &mut Env) -> ASTResult<()> {
		scope.use_closure(self.attributes().closure);

		self.value.check_types(scope)?;

		scope.reset_closure();
		Ok(())
	}
}

impl UseAttributes for Method {
	fn attributes(&self) -> &Attributes { 
		self.name.attributes() 
	}
	fn attributes_mut(&mut self) -> &mut Attributes { 
		self.name.attributes_mut() 
	}
}

impl TypeOf for Method {
	fn type_of(&self, scope: &mut Env) -> ASTResult<Cow<'_, Type>> {
		self.value.type_of(scope)
	}
}