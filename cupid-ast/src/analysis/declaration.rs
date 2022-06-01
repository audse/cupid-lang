use crate::*;

impl Analyze for Declaration {
	fn analyze_scope(&mut self, scope: &mut Env) -> Result<(), ASTErr> {
		self.type_hint.analyze_scope(scope)?;
		self.value.analyze_scope(scope)?;
		Ok(())
	}
	fn analyze_names(&mut self, scope: &mut Env) -> Result<(), ASTErr> {
		scope.no_symbol(&self.name)?;
		
		let value = SymbolValue {
			value: None,
			type_hint: (*self.type_hint).to_owned(),
			mutable: self.mutable
		};
		
		scope.set_symbol(&self.name, value);
		self.value.analyze_names(scope)
	}
	fn analyze_types(&mut self, scope: &mut Env) -> Result<(), ASTErr> {
		self.value.analyze_types(scope)?;

		self.type_hint.to_typed(self.type_hint.type_of(scope)?);
		self.value.to_typed(self.value.type_of(scope)?);
		
		Ok(())
	}
	fn check_types(&mut self, scope: &mut Env) -> Result<(), ASTErr> {
		self.value.check_types(scope)?;

		let (expected, found) = (
			self.type_hint.get_node_type()?, 
			self.value.get_node_type()?
		);
		if expected != found {
			scope.traceback.push(format!("Expected type\n{expected}, found type\n{found}"));
			return Err((self.attributes.source.unwrap(), ERR_TYPE_MISMATCH));
		}
		Ok(())
	}
}

impl UseAttributes for Declaration {
    fn attributes(&self) -> &Attributes {
        &self.attributes
    }
    fn attributes_mut(&mut self) -> &mut Attributes {
        &mut self.attributes
    }
}

impl TypeOf for Declaration {}