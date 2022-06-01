use crate::*;

impl Analyze for Type {
	fn analyze_scope(&mut self, scope: &mut Env) -> Result<(), ASTErr> {
		let closure = scope.add_isolated_closure(Some(self.name.to_owned()), Context::Type);
		self.attributes_mut().closure = closure;
		scope.use_closure(closure);

		self.name.analyze_scope(scope)?;

		for trait_symbol in self.traits.iter_mut() {
			trait_symbol.attributes_mut().closure = closure;
			trait_symbol.analyze_scope(scope)?;
		}

		for method in self.methods.iter_mut() {
			method.attributes_mut().closure = closure;
			method.analyze_scope(scope)?;
		}

		scope.reset_closure();
		Ok(())
	}
	fn analyze_names(&mut self, scope: &mut Env) -> Result<(), ASTErr> {
		scope.use_closure(self.attributes().closure);

		self.name.analyze_names(scope)?;

		for trait_symbol in self.traits.iter_mut() {
			trait_symbol.analyze_names(scope)?;
		}

		for method in self.methods.iter_mut() {
			method.analyze_names(scope)?;
		}

		scope.reset_closure();
    	Ok(())
	}
	fn analyze_types(&mut self, scope: &mut Env) -> Result<(), ASTErr> {
		scope.use_closure(self.attributes().closure);

		let self_ident = self.to_ident();
		for trait_symbol in self.traits.iter_mut() {
			scope.modify_symbol(trait_symbol, |val| {
				val.as_trait_mut()?.unify_with(&self_ident.attributes().generics)?;
				Ok(())
			})?;
			trait_symbol.analyze_types(scope)?;
		}

		for method in self.methods.iter_mut() {
			scope.modify_symbol(&method.name, |val| {
				let val_type = val.as_function_mut()?.get_type_mut();
				val_type.unify_with(&self_ident.attributes().generics)?;
				Ok(())
			})?;
			method.analyze_types(scope)?;
		}

		scope.reset_closure();
    	Ok(())
	}
	fn check_types(&mut self, scope: &mut Env) -> Result<(), ASTErr> {
		scope.use_closure(self.attributes().closure);
		for method in self.methods.iter_mut() {
			method.check_types(scope)?;
		}
		scope.reset_closure();
		Ok(())
	}
}

impl UseAttributes for Type {
	fn attributes(&self) -> &Attributes { 
		self.name.attributes() 
	}
	fn attributes_mut(&mut self) -> &mut Attributes { 
		self.name.attributes_mut() 
	}
}
