use crate::*;

impl Analyze for Function {
	fn analyze_scope(&mut self, scope: &mut Env) -> Result<(), ASTErr> {
		self.attributes.closure = scope.add_closure(None, Context::Function);
		scope.use_closure(self.attributes.closure);

		self.body.analyze_scope(scope)?;
		
		scope.reset_closure();
		Ok(())
	}
	fn analyze_names(&mut self, scope: &mut Env) -> Result<(), ASTErr> {
		scope.use_closure(self.attributes.closure);
		
		for param in self.params.iter_mut() {
			param.analyze_names(scope)?;
		}
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
		self.body.to_typed(self.body.type_of(scope)?);
		
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
	fn type_of(&self, _scope: &mut Env) -> Result<Type, ASTErr> {
		let return_type = self.body.get_type().to_owned();
    	let mut params: Vec<Typed<Ident>> = self.params
			.iter()
			.map(|p| (p.type_hint).to_owned())
			.collect();
		params.push(IsTyped(Ident::default(), return_type));
			
		let mut signature = FUNCTION.to_owned();
		signature.unify_with(&params)?;

		Ok(signature)
		// Ok(function_signature(
		// 	self.attributes.generics.to_owned(), 
		// 	params, 
		// 	return_type, 
		// 	scope
		// ))
	}
}

// fn param_is_not_type(param: &mut Ident, scope: &mut Env) -> Result<(), ASTErr> {
// 	if scope.get_type(param).is_ok() {
// 		Err((param.source(), ERR_UNEXPECTED_TYPE))
// 	} else {
// 		Ok(())
// 	}
// }
