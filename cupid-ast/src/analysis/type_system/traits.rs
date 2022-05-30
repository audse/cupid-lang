use crate::*;

impl Analyze for Trait {
	fn analyze_scope(&mut self, scope: &mut Env) -> Result<(), ASTErr> {
		let closure = scope.add_isolated_closure(Some(self.name.to_owned()), Context::Trait);
		scope.update_closure(&self.name, closure)?;
		scope.use_closure(closure);
		self.attributes_mut().closure = closure;

		self.name.analyze_scope(scope)?;
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
		for method in self.methods.iter_mut() {
			method.analyze_names(scope)?;
		}

		scope.reset_closure();
		Ok(())
	}
	fn analyze_types(&mut self, scope: &mut Env) -> Result<(), ASTErr> {
		scope.use_closure(self.attributes().closure);

		for method in self.methods.iter_mut() {
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

impl UseAttributes for Trait {
	fn attributes(&self) -> &Attributes { 
		self.name.attributes() 
	}
	fn attributes_mut(&mut self) -> &mut Attributes { 
		self.name.attributes_mut() 
	}
}

