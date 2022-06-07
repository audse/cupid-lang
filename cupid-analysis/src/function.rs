use crate::*;

impl PreAnalyze for Function {}

impl Analyze for Function {
	fn analyze_scope(&mut self, scope: &mut Env) -> ASTResult<()> {
		self.attributes.closure = scope.add_closure(None, Context::Function);
		scope.use_closure(self.attributes.closure);
		
		for param in self.params.iter_mut() {
			param.analyze_scope(scope)?;
		}
		self.return_type.analyze_scope(scope)?;
		self.body.analyze_scope(scope)?;	

		scope.reset_closure();
		Ok(())
	}
	fn analyze_names(&mut self, scope: &mut Env) -> ASTResult<()> {
		scope.use_closure(self.attributes.closure);
		
		for param in self.params.iter_mut() {
			scope.trace("Adding parameter...");
			param.analyze_names(scope)?;
		}
		self.return_type.analyze_names(scope)?;
		self.body.analyze_names(scope)?;
		
		scope.reset_closure();
		Ok(())
	}
	fn analyze_types(&mut self, scope: &mut Env) -> ASTResult<()> {
		scope.use_closure(self.attributes.closure);
		
		for param in self.params.iter_mut() {
			param.analyze_types(scope)?;
		}
		
		self.body.analyze_types(scope)?;
		self.return_type.analyze_types(scope)?;

		self.body.to_typed(self.body.type_of(scope)?.into_owned());
		self.return_type.to_typed(self.return_type.type_of(scope)?.into_owned());
		
		scope.reset_closure();
		Ok(())
	}
	fn check_types(&mut self, scope: &mut Env) -> ASTResult<()> {
		scope.use_closure(self.attributes.closure);

		let (body_type, return_type) = (
			self.body.get_node_type()?,
			self.return_type.get_node_type()?
		);
		if body_type != return_type {
			scope.trace(format!("\nExpected to return: \n{return_type}Actually returned: \n{body_type}"));
			return self.return_type.to_err(ERR_TYPE_MISMATCH)
		}
		scope.reset_closure();
		Ok(())
	}
}

impl TypeOf for Function {
	fn type_of(&self, scope: &mut Env) -> ASTResult<Cow<Type>> { 
		Ok(self.infer(scope)?.into())
	}
}