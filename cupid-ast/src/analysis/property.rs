use crate::*;

impl PreAnalyze for Property<'_> {}

impl Analyze for Property<'_> {
	fn analyze_scope(&mut self, scope: &mut Env) -> ASTResult<()> {
		self.object.analyze_scope(scope)?;
		self.property.analyze_scope(scope)?;
		Ok(())
	}
	fn analyze_names(&mut self, scope: &mut Env) -> ASTResult<()> {		
    	self.object.analyze_names(scope)?;

		scope.use_closure(self.object.attributes().closure);

		// _ = self.property.analyze_names(scope);
		
		scope.reset_closure();
		Ok(())
	}
	fn analyze_types(&mut self, scope: &mut Env) -> ASTResult<()> {
    	self.object.analyze_types(scope)?;
		self.object.to_typed(self.object.type_of(scope)?.into_owned());

		scope.use_closure(self.object.attributes().closure);

		// Property names get analyzed after object's type is analyzed
		// so that associated type methods can be resolved
		scope.trace(format!("Finding property of {}", self.object));
		if self.property.analyze_names(scope).is_err() {
			let object_type = self.object.get_node_type()?;
			scope.trace(format!("Finding method in type {}", object_type));
			scope.use_closure(object_type.attributes().closure);
			self.property.analyze_names(scope)?;
		}

		self.property.analyze_types(scope)?;
		self.property.to_typed(self.property.type_of(scope)?.into_owned());
		
		scope.reset_closure();
		Ok(())
	}
	fn check_types(&mut self, scope: &mut Env) -> ASTResult<()> {	
		self.object.check_types(scope)?;
		scope.use_closure(self.object.attributes().closure);

    	let object_type = self.object.get_node_type()?;
		
		self.property.check_types(scope)?;
		if !is_allowed_access(object_type, &self.property) {
			return self.to_err(ERR_BAD_ACCESS)
		}
		
		scope.reset_closure();
		Ok(())
	}
}

impl UseAttributes for Property<'_> {
    fn attributes(&self) -> &Attributes {
        &self.attributes
    }
    fn attributes_mut(&mut self) -> &mut Attributes {
        &mut self.attributes
    }
}

#[allow(unused_variables)]
impl TypeOf for Property<'_> {
	fn type_of(&self, scope: &mut Env) -> ASTResult<Cow<'_, Type>> { 
		let property_type = self.property
			.get_type()
			.map_err(|e| self.property.as_err(e));
    	Ok(property_type?.into())
	}
}

fn is_allowed_access(object_type: &Type, property: &Typed<PropertyTerm>) -> bool {
	match (&object_type.base_type, &**property) {
		(_, PropertyTerm::FunctionCall(_)) => true,
		(BaseType::Array, PropertyTerm::Index(..)) => true,
		(BaseType::None, PropertyTerm::Term(_)) if property
			.get_type()
			.map_err(|e| (property, e))
			.unwrap()
			.is_string() => true,
		_ => false
	}
}

impl PreAnalyze for PropertyTerm<'_> {}

impl Analyze for PropertyTerm<'_> {
	fn analyze_scope(&mut self, scope: &mut Env) -> ASTResult<()> {
		match self {
			Self::Term(term) => term.analyze_scope(scope),
			Self::FunctionCall(function_call) => function_call.analyze_scope(scope),
			_ => Ok(())
		}
	}
	fn analyze_names(&mut self, scope: &mut Env) -> ASTResult<()> {
    	match self {
			Self::Term(term) => term.analyze_names(scope),
			Self::FunctionCall(function_call) => function_call.analyze_names(scope),
			_ => Ok(())
		}
	}
	fn analyze_types(&mut self, scope: &mut Env) -> ASTResult<()> {
		match self {
			Self::Term(term) => term.analyze_types(scope),
			Self::FunctionCall(function_call) => function_call.analyze_types(scope),
			_ => Ok(())
		}
	}
	fn check_types(&mut self, scope: &mut Env) -> ASTResult<()> {
    	match self {
			Self::Term(term) => term.check_types(scope),
			Self::FunctionCall(function_call) => function_call.check_types(scope),
			_ => Ok(())
		}
	}
}

impl UseAttributes for PropertyTerm<'_> {
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

impl TypeOf for PropertyTerm<'_> {
	fn type_of(&self, scope: &mut Env) -> ASTResult<Cow<'_, Type>> { 
    	match self {
			Self::Term(term) => term.type_of(scope),
			Self::FunctionCall(function_call) => function_call.type_of(scope),
			Self::Index(i, _) => Ok(
				i.infer(scope)
				.map_err(|(_, e)| self.as_err(e))?
				.into()
			)
		}
	}
}
