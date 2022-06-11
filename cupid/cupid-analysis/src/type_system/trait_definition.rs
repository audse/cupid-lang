use crate::*;

impl PreAnalyze for TraitDef {
    #[trace]
	fn pre_analyze_scope(&mut self, scope: &mut Env) -> ASTResult<()> {
		let closure = scope.add_isolated_closure(Some(self.name.to_owned()), Context::Trait);
		self.attributes_mut().closure = closure;
		Ok(())
	}
    #[trace]
	fn pre_analyze_names(&mut self, scope: &mut Env) -> ASTResult<()> {
		scope.no_symbol(&self.name)?;
		
		let symbol_value = SymbolValue {
			value: Some(VTrait(self.to_owned().into())),
			type_hint: Type::trait_ty().to_ident(),
			mutable: false
		};

		self.trace_define_trait(scope);
		scope.set_symbol(&self.name, symbol_value);
		Ok(())
	}
}

impl Analyze for TraitDef {
    #[trace]
	fn analyze_scope(&mut self, scope: &mut Env) -> ASTResult<()> {
		self.name.analyze_scope(scope)?;
		Ok(())
	}
    #[trace]
	fn analyze_names(&mut self, scope: &mut Env) -> ASTResult<()> {
		self.trace_analyze_generic_names(scope);
		self.name.analyze_names(scope)?;
		Ok(())
	}
    #[trace]
	fn analyze_types(&mut self, scope: &mut Env) -> ASTResult<()> {
		self.trace_analyze_generic_types(scope);
		self.name.analyze_types(scope)?;
		Ok(())
	}
}

impl TypeOf for TraitDef {}