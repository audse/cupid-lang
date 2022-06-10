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
		let object_type = object_type(&self.object, scope)?;

		object_type.use_closure(scope);

		// Property names get analyzed after object's type is analyzed
		// so that associated type methods can be resolved
		self.trace_find_property(&object_type, scope);
		self.property.analyze_names(scope)?;
		self.property.analyze_types(scope)?;
		self.property.to_typed(self.property.type_of(scope)?.into_owned());
		
		scope.reset_closure();
		Ok(())
	}
    #[trace]
	fn check_types(&mut self, scope: &mut Env) -> ASTResult<()> {	
		self.object.check_types(scope)?;
		let object_type = object_type(&self.object, scope)?;
		self.property.check_types(scope)?;
		if !is_allowed_access(&object_type, &self.property)? {
			return self.to_err(ERR_BAD_ACCESS)
		}
		Ok(())
	}
}

// TODO this makes multiple type_of calls, can we use 1?
fn object_type<'obj>(object: &'obj Typed<Exp>, scope: &mut Env) -> ASTResult<Cow<'obj, Type>> {
	if object.is_type_type() {
		Ok(object.inner().type_of_hint(scope)?)
	} else {
		Ok(object.get_node_type()?.into())
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

fn is_allowed_access(object_type: &Type, property: &Typed<PropertyTerm>) -> ASTResult<bool> {
	match (&object_type.base_type, &**property) {
		(_, PropertyTerm::FunctionCall(_))
		| (BaseType::Array, PropertyTerm::Index(..)) => Ok(true),
		(BaseType::Array, PropertyTerm::Term(_)) => Ok(property.get_node_type()?.is_int()),
		(BaseType::Struct, PropertyTerm::Term(_))
		| (BaseType::Sum, PropertyTerm::Term(_))
		| (BaseType::None, PropertyTerm::Term(_)) => Ok(true),
		_ => Ok(false)
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
