use crate::*;

impl PreAnalyze for Declaration<'_> {}

impl Analyze for Declaration<'_> {
	fn analyze_scope(&mut self, scope: &mut Env) -> ASTResult<()> {
		self.type_hint.analyze_scope(scope)?;
		self.value.analyze_scope(scope)?;
		Ok(())
	}
	fn analyze_names(&mut self, scope: &mut Env) -> ASTResult<()> {
		scope.trace(quick_fmt!("Declaring variable `", self.name, "` [", self.type_hint, "]"));
		scope.no_symbol(&self.name)?;
		
		let value = SymbolValue {
			value: None,
			type_hint: (*self.type_hint).to_owned(),
			mutable: self.mutable
		};
		
		scope.set_symbol(&self.name, value);
		self.value.analyze_names(scope)
	}
	fn analyze_types(&mut self, scope: &mut Env) -> ASTResult<()> {
		self.type_hint.analyze_types(scope)?;
		self.value.analyze_types(scope)?;

		self.type_hint.to_typed(self.type_hint.type_of(scope)?.into_owned());
		self.value.to_typed(self.value.type_of(scope)?.into_owned());
		
		Ok(())
	}
	fn check_types(&mut self, scope: &mut Env) -> ASTResult<()> {
		self.value.check_types(scope)?;

		let (expected, found) = (
			self.type_hint.get_node_type()?, 
			self.value.get_node_type()?
		);
		if expected != found {
			scope.trace(format!("Expected type\n{expected}, found type\n{found}"));
			return self.to_err(ERR_TYPE_MISMATCH);
		}
		Ok(())
	}
}

impl UseAttributes for Declaration<'_> {
    fn attributes(&self) -> &Attributes {
        &self.attributes
    }
    fn attributes_mut(&mut self) -> &mut Attributes {
        &mut self.attributes
    }
}

impl TypeOf for Declaration<'_> {}