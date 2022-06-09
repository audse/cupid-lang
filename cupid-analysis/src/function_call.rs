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
		// Add temporary type variables for argument types
		// let mut generics = self.args
		// 	.iter()
		// 	.enumerate()
		// 	.map(|(i, _)| Untyped(Ident::new_name(format!("{i}!"))) )
		// 	.collect::<Vec<Typed<Ident>>>();
		// generics.push(Untyped(Ident::new_name("t")));
		// self.function.0.attributes.generics = GenericList(generics);
		self.function.0.analyze_names(scope)?;
		let function = scope.get_symbol(&self.function.0)?.as_function()?;
		self.function.1 = Some(function);
		
		self.use_closure(scope);
		scope.trace("Analyzing names of arguments...");
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
		scope.trace("Analyzing types of arguments...");
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