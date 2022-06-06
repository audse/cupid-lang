use crate::*;

impl PreAnalyze for FunctionCall<'_> {}

impl Analyze for FunctionCall<'_> {
	fn analyze_scope(&mut self, scope: &mut Env) -> ASTResult<()> {
		self.function.0.analyze_scope(scope)?;
		for arg in self.args.iter_mut() {
			arg.analyze_scope(scope)?;
		}
		Ok(())
	}
	fn analyze_names(&mut self, scope: &mut Env) -> ASTResult<()> {

		// Add temporary type variables for argument types
		let mut generics = self.args
			.iter()
			.enumerate()
			.map(|(i, _)| Untyped(Ident::new_name(format!("{i}!"))) )
			.collect::<Vec<Typed<Ident>>>();
		generics.push(Untyped(Ident::new_name("t")));
		self.function.0.attributes.generics = GenericList(generics);

		self.function.1 = Some(scope.get_symbol(&self.function.0)?.as_function()?);
		// self.function.1.map_mut(|f| f.analyze_names(scope)).invert()?;
		
    	for arg in self.args.iter_mut() {
			arg.analyze_names(scope)?;
		}
		Ok(())
	}
	fn analyze_types(&mut self, scope: &mut Env) -> ASTResult<()> {		
		for arg in self.args.iter_mut() {
			arg.analyze_types(scope)?;
		}
    	Ok(())
	}
	fn check_types(&mut self, scope: &mut Env) -> ASTResult<()> {
		let function = self.function.1.as_mut().unwrap();
		function.analyze_types(scope)?;
		
    	for (i, exp) in self.args.iter_mut().enumerate() {
			let mut param = &mut function.params[i];
			param.value = Box::new(exp.to_owned());
			param.check_types(scope)?;
		}
		Ok(())
	}
}

impl UseAttributes for FunctionCall<'_> {
    fn attributes(&self) -> &Attributes {
        &self.attributes
    }
    fn attributes_mut(&mut self) -> &mut Attributes {
        &mut self.attributes
    }
}

impl TypeOf for FunctionCall<'_> {
	fn type_of(&self, scope: &mut Env) -> ASTResult<Cow<'_, Type>> { 
		self.function.1.as_ref().unwrap().return_type.type_of(scope)
	}
}