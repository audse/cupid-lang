use crate::*;

impl PreAnalyze for Function {}

impl Analyze for Function {
	fn analyze_scope(&mut self, scope: &mut Env) -> Result<(), ASTErr> {
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
	fn analyze_names(&mut self, scope: &mut Env) -> Result<(), ASTErr> {
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
	fn analyze_types(&mut self, scope: &mut Env) -> Result<(), ASTErr> {
		scope.use_closure(self.attributes.closure);
		
		for param in self.params.iter_mut() {
			param.analyze_types(scope)?;
		}
		
		self.body.analyze_types(scope)?;
		self.return_type.analyze_types(scope)?;

		self.body.to_typed(self.body.type_of(scope)?);
		self.return_type.to_typed(self.return_type.type_of(scope)?);
		
		scope.reset_closure();
		Ok(())
	}
	fn check_types(&mut self, scope: &mut Env) -> Result<(), ASTErr> {
		scope.use_closure(self.attributes.closure);

		let (body_type, return_type) = (
			self.body.get_node_type()?,
			self.return_type.get_node_type()?
		);
		if self.body.get_node_type()? != self.return_type.get_node_type()? {
			scope.trace(format!("\nExpected to return: \n{return_type}Actually returned: \n{body_type}"));
			return Err((self.return_type.source(), ERR_TYPE_MISMATCH));
		}
		scope.reset_closure();
		Ok(())
	}
}

impl UseAttributes for Function {
    fn attributes(&self) -> &Attributes {
        &self.attributes
    }
    fn attributes_mut(&mut self) -> &mut Attributes {
        &mut self.attributes
    }
}

impl TypeOf for Function {
	fn type_of(&self, scope: &mut Env) -> Result<Type, ASTErr> {
		infer_function(self, scope)
	}
}