use crate::*;

build_struct! {
	#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Tabled)]
	pub MethodBuilder => pub Method {
		pub signature: Type,
		#[tabled(display_with = "fmt_option")]
		pub value: Option<Function>,
	}
}

impl Analyze for Method {
	fn analyze_names(&mut self, scope: &mut Env) -> Result<(), (Source, ErrCode)> {
		self.signature.analyze_names(scope)?;
		
		if let Some(val) = &mut self.value {
			val.analyze_names(scope)?;
		}
		let value = self.value
			.as_mut()
			.map(|func| ValueBuilder::new()
				.untyped_val(Val::Function(func.to_owned()))
				.attributes(func.attributes().to_owned())
				.build());
		let symbol_value = SymbolValueBuilder::new()
			.value(value)
			.type_hint(self.signature.name.to_owned())
			.build();
		scope.set_symbol(&self.signature.name, symbol_value);
		
		Ok(())
	}
	fn analyze_types(&mut self, scope: &mut Env) -> Result<(), (Source, ErrCode)> {
		self.signature.analyze_types(scope)?;

		if let Some(val) = &mut self.value {
			val.analyze_types(scope)?;
		}

    	Ok(())
	}
	fn check_types(&mut self, scope: &mut Env) -> Result<(), (Source, ErrCode)> {
		if let Some(val) = &mut self.value {
			val.check_types(scope)?;

			if self.signature != val.type_of(scope)? {
				return Err((self.source(), ERR_TYPE_MISMATCH));
			}
		}
		Ok(())
	}
}

impl UseAttributes for Method {
	fn attributes(&mut self) -> &mut Attributes { self.signature.attributes() }
}