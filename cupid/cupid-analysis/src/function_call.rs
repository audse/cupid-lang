use crate::*;

impl PreAnalyze for FunctionCall {}

impl Analyze for FunctionCall {
    #[trace]
	fn analyze_scope(&mut self, scope: &mut Env) -> ASTResult<()> {
		self.set_closure(scope);
		self.function.0.analyze_scope(scope)?;
		for arg in self.args.iter_mut() {
			arg.analyze_scope(scope)?;
		}
		Ok(())
	}
    #[trace]
	fn analyze_names(&mut self, scope: &mut Env) -> ASTResult<()> {
		self.function.0.analyze_names(scope)?;
		let function = scope.get_symbol(&self.function.0)?.as_function()?;
		self.function.1 = Some(function);
		
		self.use_closure(scope);
		self.trace_analyze_arg_names(scope);
    	for arg in self.args.iter_mut() {
			arg.analyze_names(scope)?;
		}
		scope.reset_closure();
		Ok(())
	}
    #[trace]
	fn analyze_types(&mut self, scope: &mut Env) -> ASTResult<()> {	
		let function = self.function.1.as_mut().unwrap();
		function.analyze_types(scope)?;

		self.use_closure(scope);
		self.trace_analyze_arg_types(scope);
		for arg in self.args.iter_mut() {
			arg.analyze_types(scope)?;
			arg.to_typed(arg.type_of(scope)?.into_owned());
		}
		scope.reset_closure();
    	Ok(())
	}
    #[trace]
	fn check_types(&mut self, scope: &mut Env) -> ASTResult<()> {
    	for (i, exp) in self.args.iter_mut().enumerate() {
			let mut param = &mut self.function.1.as_mut().unwrap().params[i];
			param.value = Box::new(exp.to_owned());
			param.check_types(scope)?;
		}
		Ok(())
	}
}

impl TypeOf for FunctionCall {
	fn type_of(&self, scope: &mut Env) -> ASTResult<Cow<Type>> { 
		self.function.1.as_ref().unwrap().return_type.type_of(scope)
	}
}