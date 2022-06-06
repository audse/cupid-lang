use crate::*;

impl<T: Default> PreAnalyze for Value<T> {}

impl<T: Default> Analyze for Value<T> {
	fn analyze_scope(&mut self, scope: &mut Env) -> ASTResult<()> {
		if let Some(function) = self.as_function_mut() {
			function.analyze_scope(scope)?;
		}
		if let Some(type_val) = self.as_type_mut() {
			type_val.analyze_scope(scope)?;
		}
		if let Some(trait_val) = self.as_trait_mut() {
			trait_val.analyze_scope(scope)?;
		}
		Ok(())
	}
	fn analyze_names(&mut self, scope: &mut Env) -> ASTResult<()> {
		if let Some(function) = self.as_function_mut() {
			function.analyze_names(scope)?;
		}
		if let Some(type_val) = self.as_type_mut() {
			type_val.analyze_names(scope)?;
		}
		if let Some(trait_val) = self.as_trait_mut() {
			trait_val.analyze_names(scope)?;
		}
		Ok(())
	}
	fn analyze_types(&mut self, scope: &mut Env) -> ASTResult<()> {
		if let Some(function) = self.as_function_mut() {
			function.analyze_types(scope)?;
		}
		if let Some(type_val) = self.as_type_mut() {
			type_val.analyze_types(scope)?;
		}
		if let Some(trait_val) = self.as_trait_mut() {
			trait_val.analyze_types(scope)?;
		}
		self.value.to_typed(self.value.type_of(scope)?.into_owned());
		Ok(())
	}
}

impl<T: Default> UseAttributes for Value<T> {
	fn attributes(&self) -> &Attributes { 
		&self.attributes
	}
	fn attributes_mut(&mut self) -> &mut Attributes { 
		&mut self.attributes
	}
}

impl<T: Default> TypeOf for Value<T> {
	fn type_of(&self, scope: &mut Env) -> ASTResult<Cow<'_, Type>> {
		match &self.value {
			IsTyped(_, t) => Ok(t.into()),
			Untyped(val) => val.infer(scope)?.into()
		}
	}
}