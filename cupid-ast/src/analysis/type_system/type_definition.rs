use crate::*;

impl PreAnalyze for TypeDef<'_> {
	fn pre_analyze_scope(&mut self, scope: &mut Env) -> ASTResult<()> {
		let closure = scope.add_isolated_closure(Some(self.name.to_owned()), Context::Type);
		self.attributes_mut().closure = closure;
		Ok(())
	}
	fn pre_analyze_names(&mut self, scope: &mut Env) -> ASTResult<()> {
		scope.no_symbol(&self.name)?;

		let type_value = Value {
			value: IsTyped(self.to_owned().into(), TYPE.to_owned()),
			attributes: self.attributes().to_owned()
		};
		
		let symbol_value = SymbolValue {
			value: Some(type_value),
			type_hint: TYPE.to_ident(),
			mutable: false
		};

		scope.trace(quick_fmt!("Defining type ", self.name));
		scope.set_symbol(&self.name, symbol_value);
		Ok(())
	}
}

impl Analyze for TypeDef<'_> {
	fn analyze_scope(&mut self, scope: &mut Env) -> ASTResult<()> {
		self.name.analyze_scope(scope)?;
		Ok(())
	}
	fn analyze_names(&mut self, scope: &mut Env) -> ASTResult<()> {
		scope.trace(quick_fmt!("Analyzing generics of type ", self.name));
		self.name.analyze_names(scope)?;
		Ok(())
	}
	fn analyze_types(&mut self, scope: &mut Env) -> ASTResult<()> {
		scope.trace(quick_fmt!("Analyzing types of generics of type ", self.name));
		self.name.analyze_types(scope)?;
		Ok(())
	}
}

impl UseAttributes for TypeDef<'_> {
	fn attributes(&self) -> &Attributes {
		&self.name.attributes
	}
	fn attributes_mut(&mut self) -> &mut Attributes {
		&mut self.name.attributes
	}
}

impl TypeOf for TypeDef<'_> {}