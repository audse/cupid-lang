use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Function {
	pub body: Typed<Block>,
	pub params: Vec<Declaration>,
	pub attributes: Attributes,
}

impl Analyze for Function {
	fn analyze_scope(&mut self, scope: &mut Env) -> Result<(), ErrCode> {
		self.attributes.closure = scope.add_closure();
		Ok(())
	}
	fn analyze_names(&mut self, scope: &mut Env) -> Result<(), ErrCode> {
		scope.use_closure(self.attributes.closure);
		
		for param in self.params.iter_mut() {
			param.analyze_names(scope)?;
		}
		self.body.analyze_names(scope)?;
		
		scope.reset_closure();
		Ok(())
	}
	fn analyze_types(&mut self, scope: &mut Env) -> Result<(), ErrCode> {
		scope.use_closure(self.attributes.closure);
		
		for param in self.params.iter_mut() {
			param.analyze_types(scope)?;
		}
		self.body.analyze_types(scope)?;
		self.body.to_typed(self.body.type_of(scope)?);
		
		scope.reset_closure();
		Ok(())
	}
}

impl UseAttributes for Function {
	fn attributes(&mut self) -> &mut Attributes { &mut self.attributes }
}

impl TypeOf for Function {
	fn type_of(&self, scope: &mut Env) -> Result<Type, ErrCode> {
		let return_type = self.body.get_type().to_ident();
    	let params: Vec<Ident> = self.params
			.iter()
			.map(|p| (*p.type_hint).to_owned())
			.collect();
		Ok(function_signature(
			self.attributes.generics.to_owned(), 
			params, 
			return_type, 
			scope
		))
	}
}