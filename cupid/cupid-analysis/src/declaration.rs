use crate::*;

impl PreAnalyze for Declaration {}

impl Analyze for Declaration {
    #[trace]
	fn analyze_scope(&mut self, scope: &mut Env) -> ASTResult<()> {
		self.attributes_mut().closure = scope.current_closure;
		self.type_hint.analyze_scope(scope)?;
		self.value.analyze_scope(scope)?;
		Ok(())
	}
    #[trace]
	fn analyze_names(&mut self, scope: &mut Env) -> ASTResult<()> {
		self.trace_declare(scope);
		scope.no_symbol(&self.name)?;
		
		let value = SymbolValue {
			value: None,
			type_hint: (*self.type_hint).to_owned(),
			mutable: self.mutable
		};
		scope.set_symbol(&self.name, value);
		self.value.analyze_names(scope)?;

		Ok(())
	}
    #[trace]
	fn analyze_types(&mut self, scope: &mut Env) -> ASTResult<()> {
		self.type_hint.analyze_types(scope)?;
		self.value.analyze_types(scope)?;

		self.type_hint.to_typed(self.type_hint.type_of_hint(scope)?.into_owned());
		self.value.to_typed(self.value.type_of(scope)?.into_owned());
		
		Ok(())
	}
    #[trace]
	fn check_types(&mut self, scope: &mut Env) -> ASTResult<()> {
		self.value.check_types(scope)?;

		let (expected, found) = (
			self.type_hint.get_node_type()?, 
			self.value.get_node_type()?
		);
		if expected != found {
			self.trace_type_mismatch(scope);
			return self.to_err(ERR_TYPE_MISMATCH);
		}
		Ok(())
	}
}

impl TypeOf for Declaration {}