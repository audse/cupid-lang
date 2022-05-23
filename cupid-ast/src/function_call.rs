use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionCall {
	function_name: Ident,
	args: Vec<Typed<Exp>>,
	function: Option<Function>,
}

impl TypeOf for FunctionCall {
	fn type_of(&self, scope: &mut Env) -> Type {
    	if let Some(function) = &self.function {
			get_type_or_panic(&*return_type, scope)
		} else {
			Type::nothing()
		}
	}
}

impl Analyze for FunctionCall {
	fn resolve_names(&mut self, scope: &mut Env) -> Result<(), Error> {
		let function = scope
			.get_symbol(&self.function_name)
			.unwrap_or_else(|| panic!("function not found"))
			.as_function()
			.unwrap_or_else(|| panic!("expected function"));
		self.function = Some(function);
    	for arg in self.args.iter_mut() {
			arg.resolve_names(scope)?;
		}
		Ok(())
	}
	fn resolve_types(&mut self, scope: &mut Env) -> Result<(), Error> {
		for arg in self.args.iter_mut() {
			arg.resolve_types(scope)?;
		}
		self.function.as_mut().unwrap().resolve_types(scope)?;
		self.return_type = self.function
			.as_ref()
			.unwrap()
			.get_return_type()
			.map(|t| t);
    	Ok(())
	}
}