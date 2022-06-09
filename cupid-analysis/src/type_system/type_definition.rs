use crate::*;

impl PreAnalyze for TypeDef {
    #[trace]
	fn pre_analyze_scope(&mut self, scope: &mut Env) -> ASTResult<()> {
		let closure = scope.add_isolated_closure(Some(self.name.to_owned()), Context::Type);
		self.attributes_mut().closure = closure;
		Ok(())
	}
    #[trace]
	fn pre_analyze_names(&mut self, scope: &mut Env) -> ASTResult<()> {
		scope.no_symbol(&self.name)?;
				
		let symbol_value = SymbolValue {
			value: Some(VType(self.to_owned().into())),
			type_hint: type_type().to_ident(),
			mutable: false
		};

		scope.trace(quick_fmt!("Defining type ", self.name));
		scope.set_symbol(&self.name, symbol_value);

		self.use_closure(scope);

		for field in &mut *self.fields {
			let mut dec: Declaration = field.to_owned().into();
			dec.analyze_names(scope)?;
		}

		scope.reset_closure();
		Ok(())
	}
}

impl Analyze for TypeDef {
    #[trace]
	fn analyze_scope(&mut self, scope: &mut Env) -> ASTResult<()> {
		self.name.analyze_scope(scope)?;
		Ok(())
	}
    #[trace]
	fn analyze_names(&mut self, scope: &mut Env) -> ASTResult<()> {
		scope.trace(quick_fmt!("Analyzing generics of type ", self.name));
		self.name.analyze_names(scope)?;
		Ok(())
	}
    #[trace]
	fn analyze_types(&mut self, scope: &mut Env) -> ASTResult<()> {
		scope.trace(quick_fmt!("Analyzing types of generics of type ", self.name));
		self.name.analyze_types(scope)?;
		Ok(())
	}
}

impl TypeOf for TypeDef {}