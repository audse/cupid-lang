use crate::*;

impl PreAnalyze for Implement {
    #[trace]
	fn pre_analyze_names(&mut self, scope: &mut Env) -> ASTResult<()> {
		let for_type = scope.get_type(&self.for_type)?;
		self.set_closure_to(for_type.closure());

		if let Some(for_trait) = &self.for_trait {
			scope.has_symbol(for_trait)?;
		}
		self.use_closure(scope);

		for method in self.methods.iter_mut() {
			scope.no_symbol(&method.name)?;
			
			method.pre_analyze_scope(scope)?;
			method.pre_analyze_names(scope)?;
		}

		scope.reset_closure();
		Ok(())
	}
}

impl Analyze for Implement {
    #[trace]
	fn analyze_scope(&mut self, scope: &mut Env) -> ASTResult<()> {
		self.use_closure(scope);
		
		for method in self.methods.iter_mut() {
			method.analyze_scope(scope)?;
		}

		scope.reset_closure();
		Ok(())
	}
    #[trace]
	fn analyze_names(&mut self, scope: &mut Env) -> ASTResult<()> {
		self.use_closure(scope);

		self.for_type.analyze_names(scope)?;
		self.for_trait.map_mut(|t| t.analyze_names(scope)).invert()?;

		for method in self.methods.iter_mut() {
			method.analyze_names(scope)?;
		}

		scope.reset_closure();
		Ok(())
	}
    #[trace]
	fn analyze_types(&mut self, scope: &mut Env) -> ASTResult<()> {
		self.use_closure(scope);

		self.for_type.analyze_types(scope)?;
		self.for_trait.map_mut(|t| t.analyze_types(scope)).invert()?;

		for method in self.methods.iter_mut() {
			method.analyze_types(scope)?;
		}

		scope.reset_closure();
		Ok(())
	}
    #[trace]
	fn check_types(&mut self, scope: &mut Env) -> ASTResult<()> {
		self.use_closure(scope);

		for method in self.methods.iter_mut() {
			if !method.value.body.body.is_empty() {
				method.check_types(scope)?;
			}
		}
		
		scope.reset_closure();
		Ok(())
	}
}

#[allow(unused_variables)]
impl TypeOf for Implement {
	fn type_of(&self, scope: &mut Env) -> ASTResult<Cow<Type>> { 
		Ok(Type::none().into())
	}
}