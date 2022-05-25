use crate::*;

pub struct Property {
	pub object: Typed<Exp>,
	pub property: Typed<Exp>,
	pub attributes: Attributes,
}

impl Analyze for Property {
	fn analyze_names(&mut self, scope: &mut Env) -> Result<(), (Source, ErrCode)> {
    	self.object.analyze_names(scope)?;
		self.property.analyze_names(scope)?;
		Ok(())
	}
	fn analyze_types(&mut self, scope: &mut Env) -> Result<(), (Source, ErrCode)> {
    	self.object.analyze_types(scope)?;
		self.property.analyze_types(scope)?;
		
		self.object.to_typed(self.object.type_of(scope)?);
		self.property.to_typed(self.property.type_of(scope)?);
		
		Ok(())
	}
	fn check_types(&mut self, scope: &mut Env) -> Result<(), (Source, ErrCode)> {
		self.object.check_types(scope)?;
		self.property.check_types(scope)?;
		
    	let object_type = self.object.get_type();
		let property_type = self.property.get_type();
		
		if !is_allowed_access(object_type, property_type) {
			return Err((self.source(), ERR_BAD_ACCESS));
		}
		Ok(())
	}
}

impl UseAttributes for Property {
	fn attributes(&mut self) -> &mut Attributes { &mut self.attributes }
}

impl TypeOf for Property {
	fn type_of(&self, scope: &mut Env) -> Result<Type, (Source, ErrCode)> {
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