use crate::*;

impl PreAnalyze for TraitDef<'_> {
	fn pre_analyze_scope(&mut self, scope: &mut Env) -> ASTResult<()> {
		let closure = scope.add_isolated_closure(Some(self.name.to_owned()), Context::Trait);
		self.attributes_mut().closure = closure;
		Ok(())
	}
	fn pre_analyze_names(&mut self, scope: &mut Env) -> ASTResult<()> {
		scope.no_symbol(&self.name)?;

		let trait_value = Value {
			value: IsTyped(self.to_owned().into(), TRAIT.to_owned()),
			attributes: self.attributes().to_owned()
		};
		
		let symbol_value = SymbolValue {
			value: Some(trait_value),
			type_hint: TRAIT.to_ident(),
			mutable: false
		};

		scope.trace(quick_fmt!("Defining trait ", self.name));
		scope.set_symbol(&self.name, symbol_value);
		Ok(())
	}
}

impl Analyze for TraitDef<'_> {
	fn analyze_scope(&mut self, scope: &mut Env) -> ASTResult<()> {
		self.name.analyze_scope(scope)?;
		Ok(())
	}
	fn analyze_names(&mut self, scope: &mut Env) -> ASTResult<()> {
		scope.trace(quick_fmt!("Analyzing generics of trait ", self.name));
		self.name.analyze_names(scope)?;
		Ok(())
	}
	fn analyze_types(&mut self, scope: &mut Env) -> ASTResult<()> {
		scope.trace(quick_fmt!("Analyzing types of generics of trait ", self.name));
		self.name.analyze_types(scope)?;
		Ok(())
	}
}

impl UseAttributes for TraitDef<'_> {
	fn attributes(&self) -> &Attributes {
		&self.attributes
	}
	fn attributes_mut(&mut self) -> &mut Attributes {
		&mut self.attributes
	}
}

impl TypeOf for TraitDef<'_> {}