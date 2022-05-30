use crate::*;

impl Analyze for Method {
	fn analyze_scope(&mut self, scope: &mut Env) -> Result<(), ASTErr> {
		scope.use_closure(self.attributes().closure);
		
		self.name.analyze_scope(scope)?;
		self.value.map_mut(|val| val.analyze_scope(scope)).invert()?;

		scope.reset_closure();
		Ok(())
	}
	fn analyze_names(&mut self, scope: &mut Env) -> Result<(), ASTErr> {
		scope.use_closure(self.attributes().closure);

		scope.set_symbol(&self.name, SymbolValue { 
			value: self.value.to_owned().map(|f| 
				Value::build()
				.typed_val(Val::Function(Box::new(Untyped(f))), self.signature.to_owned())
				.build()
			), 
			type_hint: self.signature.to_ident(), 
			mutable: false 
		});

		self.name.analyze_names(scope)?;
		self.signature.analyze_names(scope)?;

		scope.reset_closure();
		Ok(())
	}
	fn analyze_types(&mut self, scope: &mut Env) -> Result<(), ASTErr> {
		scope.use_closure(self.attributes().closure);

		self.signature.analyze_types(scope)?;

		if let Some(val) = &mut self.value {
			val.analyze_types(scope)?;
		}

		scope.reset_closure();
    	Ok(())
	}
	fn check_types(&mut self, scope: &mut Env) -> Result<(), ASTErr> {
		scope.use_closure(self.attributes().closure);

		if let Some(val) = &mut self.value {
			val.check_types(scope)?;

			let val_type = val.type_of(scope)?;
			if self.signature != val_type {
				log!("Expected type\n", self.signature, "Found type\n", val_type);
				// scope.traceback.push(quick_fmt!("Expected type\n", self.signature, "Found type\n", val_type));
				return Err((self.source(), ERR_TYPE_MISMATCH));
			}
		}

		scope.reset_closure();
		Ok(())
	}
}

impl UseAttributes for Method {
	fn attributes(&self) -> &Attributes { 
		self.name.attributes() 
	}
	fn attributes_mut(&mut self) -> &mut Attributes { 
		self.name.attributes_mut() 
	}
}