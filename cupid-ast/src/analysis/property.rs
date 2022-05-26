use crate::*;

build_struct! {
	#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
	pub PropertyBuilder => pub Property {
		pub object: Typed<Box<Exp>>,
		pub property: Typed<PropertyTerm>,
		pub attributes: Attributes,
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PropertyTerm {
	FunctionCall(Box<FunctionCall>),
	Index(usize, Attributes),
	Term(Box<Exp>),
}

impl Analyze for Property {
	fn analyze_scope(&mut self, scope: &mut Env) -> Result<(), (Source, ErrCode)> {
		self.attributes.closure = scope.add_closure();
		Ok(())
	}
	fn analyze_names(&mut self, scope: &mut Env) -> Result<(), (Source, ErrCode)> {
		scope.use_closure(self.attributes.closure);
		
    	self.object.analyze_names(scope)?;
		self.property.analyze_names(scope)?;
		
		scope.reset_closure();
		Ok(())
	}
	fn analyze_types(&mut self, scope: &mut Env) -> Result<(), (Source, ErrCode)> {
		scope.use_closure(self.attributes.closure);
		
    	self.object.analyze_types(scope)?;
		self.property.analyze_types(scope)?;
		
		self.property.to_typed(self.property.type_of(scope)?);
		self.object.to_typed(self.object.type_of(scope)?);
		
		scope.reset_closure();
		Ok(())
	}
	fn check_types(&mut self, scope: &mut Env) -> Result<(), (Source, ErrCode)> {
		scope.use_closure(self.attributes.closure);
		
		self.object.check_types(scope)?;
    	let object_type = self.object.get_type();
		
		self.property.check_types(scope)?;
		let property_type = self.property.get_type();
		
		if !is_allowed_access(object_type, property_type) {
			return Err((self.source(), ERR_BAD_ACCESS));
		}
		
		scope.reset_closure();
		Ok(())
	}
}

impl UseAttributes for Property {
	fn attributes(&mut self) -> &mut Attributes { &mut self.attributes }
}

impl TypeOf for Property {
	fn type_of(&self, _scope: &mut Env) -> Result<Type, (Source, ErrCode)> {
    	Ok(self.property.get_type().to_owned())
	}
}

fn is_allowed_access(object_type: &Type, property_type: &Type) -> bool {
	match object_type.base_type {
		BaseType::Array => property_type.is_int() || property_type.is_function(),
		BaseType::Primitive(_) | BaseType::Function | BaseType::Sum => property_type.is_function(),
		BaseType::None => property_type.is_function() || property_type.is_string()
	}
}


impl Analyze for PropertyTerm {
	fn analyze_names(&mut self, scope: &mut Env) -> Result<(), (Source, ErrCode)> {
    	match self {
			Self::Term(term) => term.analyze_names(scope)?,
			Self::FunctionCall(_function_call) => todo!(),
			_ => ()
		}
		Ok(())
	}
	fn analyze_types(&mut self, scope: &mut Env) -> Result<(), (Source, ErrCode)> {
		match self {
			Self::Term(term) => term.analyze_types(scope)?,
			Self::FunctionCall(_function_call) => todo!(),
			_ => ()
		}
		Ok(())
	}
	fn check_types(&mut self, scope: &mut Env) -> Result<(), (Source, ErrCode)> {
    	match self {
			Self::Term(term) => term.check_types(scope)?,
			Self::FunctionCall(_function_call) => todo!(),
			_ => ()
		}
		Ok(())
	}
}

impl UseAttributes for PropertyTerm {
	fn attributes(&mut self) -> &mut Attributes {
		match self {
			Self::FunctionCall(function_call) => function_call.attributes(),
			Self::Index(_, attr) => attr,
			Self::Term(term) => term.attributes()
		}
	}
}

impl TypeOf for PropertyTerm {
	fn type_of(&self, scope: &mut Env) -> Result<Type, (Source, ErrCode)> {
    	match self {
			Self::Term(term) => term.type_of(scope),
			_  => panic!()
		}
	}
}

impl Default for PropertyTerm {
	fn default() -> Self { Self::Term(Box::new(Exp::Empty)) }
}