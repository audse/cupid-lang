use crate::*;

build_struct! {
	#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
	pub FunctionCallBuilder => pub FunctionCall {
		pub function: Typed<(Ident, Option<Function>)>,
		pub args: Vec<Typed<Exp>>,
		pub attributes: Attributes,
	}
}

impl Analyze for FunctionCall {
	fn analyze_names(&mut self, scope: &mut Env) -> Result<(), (Source, ErrCode)> {
		self.function.1 = Some(scope.get_symbol(&self.function.0)?.as_function()?);
		self.function.1.as_mut().unwrap().analyze_names(scope)?;
		
    	for arg in self.args.iter_mut() {
			arg.analyze_names(scope)?;
		}
		Ok(())
	}
	fn analyze_types(&mut self, scope: &mut Env) -> Result<(), (Source, ErrCode)> {		
		for arg in self.args.iter_mut() {
			arg.analyze_types(scope)?;
		}
    	Ok(())
	}
	fn check_types(&mut self, scope: &mut Env) -> Result<(), (Source, ErrCode)> {
		let function = self.function.1.as_mut().unwrap();
		function.analyze_types(scope)?;
		
    	for (i, exp) in self.args.iter_mut().enumerate() {
			let mut param = &mut function.params[i];
			param.value = exp.to_owned().into_map(&Box::new);
			param.check_types(scope)?;
		}
		Ok(())
	}
}

impl UseAttributes for FunctionCall {
	fn attributes(&mut self) -> &mut Attributes { &mut self.attributes }
}

impl TypeOf for FunctionCall {
	fn type_of(&self, scope: &mut Env) -> Result<Type, (Source, ErrCode)> {
		self.function.1.as_ref().unwrap().body.type_of(scope)
	}
}