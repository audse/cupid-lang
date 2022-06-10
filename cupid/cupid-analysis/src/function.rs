use crate::*;

impl PreAnalyze for Function {}

impl Analyze for Function {
    #[trace]
	fn analyze_scope(&mut self, scope: &mut Env) -> ASTResult<()> {
		self.set_closure_to(scope.add_closure(None, Context::Function));
		self.use_closure(scope);
		
		for param in self.params.iter_mut() {
			param.analyze_scope(scope)?;
		}
		self.return_type.analyze_scope(scope)?;
		self.body.analyze_scope(scope)?;

		scope.reset_closure();
		Ok(())
	}
    #[trace]
	fn analyze_names(&mut self, scope: &mut Env) -> ASTResult<()> {
		self.use_closure(scope);

		for param in self.params.iter_mut() {
			param.trace_declare_param(scope);
			param.analyze_names(scope)?;
		}
		self.return_type.analyze_names(scope)?;
		self.body.analyze_names(scope)?;
		
		scope.reset_closure();
		Ok(())
	}
    #[trace]
	fn analyze_types(&mut self, scope: &mut Env) -> ASTResult<()> {
		self.use_closure(scope);
		
		for param in self.params.iter_mut() {
			param.analyze_types(scope)?;
		}
		
		self.body.analyze_types(scope)?;
		self.return_type.analyze_types(scope)?;

		self.body.to_typed(self.body.type_of(scope)?.into_owned());
		self.return_type.to_typed(self.return_type.type_of_hint(scope)?.into_owned());
		
		scope.reset_closure();
		Ok(())
	}
    #[trace]
	fn check_types(&mut self, scope: &mut Env) -> ASTResult<()> {
		self.use_closure(scope);

		let (body_type, return_type) = (
			self.body.get_node_type()?,
			self.return_type.get_node_type()?
		);
		if body_type != return_type {
			self.trace_type_mismatch(scope);
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