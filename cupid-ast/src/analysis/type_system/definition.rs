use crate::*;

impl PreAnalyze for TypeDefinition {
	fn pre_analyze_scope(&mut self, scope: &mut Env) -> Result<(), ASTErr> {
		let closure = scope.add_isolated_closure(Some(self.name.to_owned()), Context::Type);
		self.attributes_mut().closure = closure;
		Ok(())
	}
	fn pre_analyze_names(&mut self, scope: &mut Env) -> ASTResult<()> {
		scope.no_symbol(&self.name)?;

		let type_value = Value {
			val: IsTyped(Val::Type(self.to_owned().into()), TYPE.to_owned()),
			attributes: self.attributes().to_owned()
		};
		
		let symbol_value = SymbolValue {
			value: Some(type_value),
			type_hint: TYPE.to_ident(),
			mutable: false
		};
		
		scope.set_symbol(&self.name, symbol_value);
		Ok(())
	}
}

impl Analyze for TypeDefinition {
	fn analyze_scope(&mut self, scope: &mut Env) -> Result<(), ASTErr> {
		self.name.analyze_scope(scope)?;
		Ok(())
	}
	fn analyze_names(&mut self, scope: &mut Env) -> Result<(), ASTErr> {
		self.name.analyze_names(scope)?;
		Ok(())
	}
	fn analyze_types(&mut self, scope: &mut Env) -> Result<(), ASTErr> {
		scope.use_closure(self.attributes().closure);

		self.name.analyze_types(scope)?;

		scope.reset_closure();
		Ok(())
	}
}

impl UseAttributes for TypeDefinition {
	fn attributes(&self) -> &Attributes {
		&self.name.attributes
	}
	fn attributes_mut(&mut self) -> &mut Attributes {
		&mut self.name.attributes
	}
}

impl TypeOf for TypeDefinition {}