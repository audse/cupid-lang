use crate::*;

impl Analyze for Property {
	fn analyze_scope(&mut self, scope: &mut Env) -> Result<(), ASTErr> {
		self.object.analyze_scope(scope)?;
		self.property.analyze_scope(scope)?;
		Ok(())
	}
	fn analyze_names(&mut self, scope: &mut Env) -> Result<(), ASTErr> {		
    	self.object.analyze_names(scope)?;

		scope.use_closure(self.object.attributes().closure);

		// _ = self.property.analyze_names(scope);
		
		scope.reset_closure();
		Ok(())
	}
	fn analyze_types(&mut self, scope: &mut Env) -> Result<(), ASTErr> {
    	self.object.analyze_types(scope)?;
		self.object.to_typed(self.object.type_of(scope)?);

		scope.use_closure(self.object.attributes().closure);

		// Property names get analyzed after object's type is analyzed
		// so that associated type methods can be resolved
		if self.property.analyze_names(scope).is_err() {
			let object_type = self.object.get_node_type()?;
			log!("closure", object_type.attributes().closure);
			scope.use_closure(object_type.attributes().closure);
			self.property.analyze_names(scope)?;
		}

		self.property.analyze_types(scope)?;
		self.property.to_typed(self.property.type_of(scope)?);
		
		scope.reset_closure();
		Ok(())
	}
	fn check_types(&mut self, scope: &mut Env) -> Result<(), ASTErr> {	
		self.object.check_types(scope)?;
		scope.use_closure(self.object.attributes().closure);

    	let object_type = self.object.get_node_type()?;
		
		self.property.check_types(scope)?;
		if !is_allowed_access(object_type, &self.property) {
			return Err((self.source(), ERR_BAD_ACCESS));
		}
		
		scope.reset_closure();
		Ok(())
	}
}

impl UseAttributes for Property {
    fn attributes(&self) -> &Attributes {
        &self.attributes
    }
    fn attributes_mut(&mut self) -> &mut Attributes {
        &mut self.attributes
    }
}

impl TypeOf for Property {
	fn type_of(&self, _scope: &mut Env) -> Result<Type, ASTErr> {
    	Ok(self.property.get_type().map_err(|e| (self.property.source(), e))?.to_owned())
	}
}

fn is_allowed_access(object_type: &Type, property: &Typed<PropertyTerm>) -> bool {
	match (&object_type.base_type, &**property) {
		(_, PropertyTerm::FunctionCall(_)) => true,
		(BaseType::Array, PropertyTerm::Index(..)) => true,
		(BaseType::None, PropertyTerm::Term(_)) if property
			.get_type()
			.map_err(|e| (property.source(), e))
			.unwrap()
			.is_string() => true,
		_ => false
	}
}


impl Analyze for PropertyTerm {
	fn analyze_names(&mut self, scope: &mut Env) -> Result<(), ASTErr> {
    	match self {
			Self::Term(term) => term.analyze_names(scope),
			Self::FunctionCall(function_call) => function_call.analyze_names(scope),
			_ => Ok(())
		}
	}
	fn analyze_types(&mut self, scope: &mut Env) -> Result<(), ASTErr> {
		match self {
			Self::Term(term) => term.analyze_types(scope),
			Self::FunctionCall(function_call) => function_call.analyze_types(scope),
			_ => Ok(())
		}
	}
	fn check_types(&mut self, scope: &mut Env) -> Result<(), ASTErr> {
    	match self {
			Self::Term(term) => term.check_types(scope),
			Self::FunctionCall(function_call) => function_call.check_types(scope),
			_ => Ok(())
		}
	}
}

impl UseAttributes for PropertyTerm {
	fn attributes(&self) -> &Attributes {
		match self {
			Self::FunctionCall(function_call) => function_call.attributes(),
			Self::Index(_, attr) => attr,
			Self::Term(term) => term.attributes()
		}
	}
	fn attributes_mut(&mut self) -> &mut Attributes {
		match self {
			Self::FunctionCall(function_call) => function_call.attributes_mut(),
			Self::Index(_, attr) => attr,
			Self::Term(term) => term.attributes_mut()
		}
	}
}

impl TypeOf for PropertyTerm {
	fn type_of(&self, scope: &mut Env) -> Result<Type, ASTErr> {
    	match self {
			Self::Term(term) => term.type_of(scope),
			Self::FunctionCall(function_call) => function_call.type_of(scope),
			Self::Index(..) => Ok(INTEGER.to_owned())
		}
	}
}
