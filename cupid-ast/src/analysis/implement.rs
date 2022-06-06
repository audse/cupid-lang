use crate::*;

impl PreAnalyze for Implement<'_> {
	fn pre_analyze_names(&mut self, scope: &mut Env) -> ASTResult<()> {
		let for_type = scope.get_type(&self.for_type)?;
		let closure = for_type.attributes().closure;
		self.attributes.closure = closure;

		if let Some(for_trait) = &self.for_trait {
			scope.get_symbol(for_trait)?;
		}
		
		scope.use_closure(closure);

		for method in self.methods.iter_mut() {
			scope.no_symbol(&method.name)?;
			
			method.pre_analyze_scope(scope)?;
			method.pre_analyze_names(scope)?;
		}

		scope.reset_closure();
		Ok(())
	}
}

impl Analyze for Implement<'_> {
	fn analyze_scope(&mut self, scope: &mut Env) -> ASTResult<()> {
		scope.use_closure(self.attributes.closure);
		
		for method in self.methods.iter_mut() {
			method.analyze_scope(scope)?;
		}

		scope.reset_closure();
		Ok(())
	}
	fn analyze_names(&mut self, scope: &mut Env) -> ASTResult<()> {
		scope.use_closure(self.attributes.closure);

		self.for_type.analyze_names(scope)?;
		self.for_trait.map_mut(|t| t.analyze_names(scope)).invert()?;

		for method in self.methods.iter_mut() {
			method.analyze_names(scope)?;
		}

		scope.reset_closure();
		Ok(())
	}
	fn analyze_types(&mut self, scope: &mut Env) -> ASTResult<()> {
		scope.use_closure(self.attributes.closure);

		self.for_type.analyze_types(scope)?;
		self.for_trait.map_mut(|t| t.analyze_types(scope)).invert()?;

		for method in self.methods.iter_mut() {
			method.analyze_types(scope)?;
		}

		scope.reset_closure();
		Ok(())
	}
	fn check_types(&mut self, scope: &mut Env) -> ASTResult<()> {
		scope.use_closure(self.attributes.closure);

		for method in self.methods.iter_mut() {
			if !method.value.body.body.is_empty() {
				method.check_types(scope)?;
			}
		}
		
		scope.reset_closure();
		Ok(())
	}
}

impl UseAttributes for Implement<'_> {
	fn attributes(&self) -> &Attributes {
		&self.attributes
	}
	fn attributes_mut(&mut self) -> &mut Attributes {
		&mut self.attributes
	}
}

#[allow(unused_variables)]
impl TypeOf for Implement<'_> {
	fn type_of(&self, scope: &mut Env) -> ASTResult<Cow<'_, Type>> { 
		Ok((&*NOTHING).into())
	}
}