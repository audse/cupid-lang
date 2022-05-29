use crate::*;

impl Analyze for Ident {
	fn analyze_names(&mut self, scope: &mut Env) -> Result<(), (Source, ErrCode)> {
		scope.get_symbol(self)?;
		self.set_generic_symbols(scope)?;
		Ok(())
	}
}

impl TypeOf for Ident {
	fn type_of(&self, scope: &mut Env) -> Result<Type, (Source, ErrCode)> {
		// if an ident for a type, e.g. `int`
		let mut symbol_value = scope.get_symbol(&*self)?;
		if let Some(value) = &mut symbol_value.value {
			if let Val::Type(type_hint) = &mut *value.val {
				type_hint.name.attributes.generics.apply(self.attributes.generics.to_owned());
				return Ok(type_hint.to_owned());
			}
		}
		// get the type associated with the ident's value
		scope.get_type(&symbol_value.type_hint)
	}
}

impl UseAttributes for Ident {
	fn attributes(&mut self) -> &mut Attributes { &mut self.attributes }
}