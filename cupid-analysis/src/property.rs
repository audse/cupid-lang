use crate::*;

impl PreAnalyze for Property {}

impl Analyze for Property {
    #[trace]
	fn analyze_scope(&mut self, scope: &mut Env) -> ASTResult<()> {
		self.set_closure(scope);
		self.object.analyze_scope(scope)?;
		self.property.analyze_scope(scope)?;
		Ok(())
	}
    #[trace]
	fn analyze_names(&mut self, scope: &mut Env) -> ASTResult<()> {	
    	self.object.analyze_names(scope)?;
		Ok(())
	}
    #[trace]
	fn analyze_types(&mut self, scope: &mut Env) -> ASTResult<()> {
    	self.object.analyze_types(scope)?;
		self.object.to_typed(self.object.type_of(scope)?.into_owned());
		let object_type = self.object.get_node_type()?;

		object_type.use_closure(scope);

		// Property names get analyzed after object's type is analyzed
		// so that associated type methods can be resolved
		scope.trace(format!("Finding property `{}` of type \n{object_type}", *self.property));

		self.property.analyze_names(scope)?;
		self.property.analyze_types(scope)?;
		self.property.to_typed(self.property.type_of(scope)?.into_owned());
		
		scope.reset_closure();
		Ok(())
	}
    #[trace]
	fn check_types(&mut self, scope: &mut Env) -> ASTResult<()> {	
		self.object.check_types(scope)?;
    	let object_type = self.object.get_node_type()?;
		self.property.check_types(scope)?;
		if !is_allowed_access(object_type, &self.property) {
			return self.to_err(ERR_BAD_ACCESS)
		}
		Ok(())
	}
}

#[allow(unused_variables)]
impl TypeOf for Property {
	fn type_of(&self, scope: &mut Env) -> ASTResult<Cow<Type>> { 
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

impl PreAnalyze for PropertyTerm {}

impl Analyze for PropertyTerm {
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

impl TypeOf for PropertyTerm {
	fn type_of(&self, scope: &mut Env) -> ASTResult<Cow<Type>> { 
    	match self {
			Self::Term(term) => term.type_of(scope),
			Self::FunctionCall(function_call) => function_call.type_of(scope),
			Self::Index(i, attr) => Ok(VInteger(*i as i32, attr.to_owned()).infer(scope)?.into())
		}
	}
}
