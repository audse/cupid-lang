use crate::*;

impl Analyze for Type {
	fn analyze_scope(&mut self, scope: &mut Env) -> Result<(), (Source, ErrCode)> {
		let closure = scope.add_isolated_closure(Some(self.name.to_owned()), Context::Type);
		scope.update_closure(&self.name, closure)?;
		scope.use_closure(closure);
		self.attributes().closure = closure;

		self.name.analyze_scope(scope)?;
		for trait_val in self.traits.iter_mut() {
			scope.update_closure(trait_val, closure)?;
			trait_val.attributes().closure = closure;
		}
		for method in self.methods.iter_mut() {
			method.attributes().closure = closure;
			method.analyze_scope(scope)?;
		}
		scope.reset_closure();
		Ok(())
	}
	fn analyze_names(&mut self, scope: &mut Env) -> Result<(), (Source, ErrCode)> {
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
	fn analyze_types(&mut self, scope: &mut Env) -> Result<(), (Source, ErrCode)> {
		scope.use_closure(self.attributes().closure);

		let self_ident = self.to_ident();
		for trait_symbol in self.traits.iter_mut() {
			scope.modify_symbol(trait_symbol, |val| {
				use_type_as_generic_args(val.as_trait_mut()?, self_ident.to_owned());
				Ok(())
			})?;
			trait_symbol.analyze_types(scope)?;
		}

		for method in self.methods.iter_mut() {
			scope.modify_symbol(&method.name, |val| {
				let val_type = val.as_function_mut()?.get_type_mut();
				use_type_as_generic_args(val_type, self_ident.to_owned());
				Ok(())
			})?;
			method.analyze_types(scope)?;
		}

		scope.reset_closure();
    	Ok(())
	}
	fn check_types(&mut self, scope: &mut Env) -> Result<(), (Source, ErrCode)> {
		scope.use_closure(self.attributes().closure);
		for method in self.methods.iter_mut() {
			method.check_types(scope)?;
		}
		scope.reset_closure();
		Ok(())
	}
}

impl UseAttributes for Type {
	fn attributes(&mut self) -> &mut Attributes { &mut self.name.attributes }
}
