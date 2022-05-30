use crate::*;

impl Analyze for FunctionCall {
	fn analyze_scope(&mut self, scope: &mut Env) -> Result<(), ASTErr> {
		self.function.0.analyze_scope(scope)?;
		for arg in self.args.iter_mut() {
			arg.analyze_scope(scope)?;
		}
		Ok(())
	}
	fn analyze_names(&mut self, scope: &mut Env) -> Result<(), ASTErr> {
		self.function.1 = Some((*scope.get_symbol(&self.function.0)?.as_function()?).to_owned());
		self.function.1.map_mut(|f| f.analyze_names(scope)).invert()?;
		
    	for arg in self.args.iter_mut() {
			arg.analyze_names(scope)?;
		}
		Ok(())
	}
	fn analyze_types(&mut self, scope: &mut Env) -> Result<(), ASTErr> {		
		for arg in self.args.iter_mut() {
			arg.analyze_types(scope)?;
		}
    	Ok(())
	}
	fn check_types(&mut self, scope: &mut Env) -> Result<(), ASTErr> {
		let function = self.function.1.as_mut().unwrap();
		function.analyze_types(scope)?;
		
    	for (i, exp) in self.args.iter_mut().enumerate() {
			let mut param = &mut function.params[i];
			param.value = exp.to_owned().into_map(&Box::new);
			param.check_types(scope)?;
		}
		Ok(())
	}
}

impl UseAttributes for FunctionCall {
    fn attributes(&self) -> &Attributes {
        &self.attributes
    }
    fn attributes_mut(&mut self) -> &mut Attributes {
        &mut self.attributes
    }
}

impl TypeOf for FunctionCall {
	fn type_of(&self, scope: &mut Env) -> Result<Type, ASTErr> {
		self.function.1.as_ref().unwrap().body.type_of(scope)
	}
}