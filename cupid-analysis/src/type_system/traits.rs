use crate::*;

impl PreAnalyze for Trait {}

impl Analyze for Trait {
	// fn analyze_scope(&mut self, scope: &mut Env) -> ASTResult<()> {
	// 	let closure = scope.add_isolated_closure(Some(self.name.to_owned()), Context::Trait);
	// 	scope.update_closure(&self.name, closure)?;
	// 	use_closure!(scope, self.closure());
	// 	self.attributes_mut().closure = closure;

	// 	self.name.analyze_scope(scope)?;
	// 	for method in self.methods.iter_mut() {
	// 		method.attributes_mut().closure = closure;

	// 		method.analyze_scope(scope)?;
	// 	}
	// 	scope.reset_closure();
	// 	Ok(())
	// }
	// fn analyze_names(&mut self, scope: &mut Env) -> ASTResult<()> {
	// 	use_closure!(scope, self.closure());

	// 	self.name.analyze_names(scope)?;
	// 	for method in self.methods.iter_mut() {
	// 		method.analyze_names(scope)?;
	// 	}

	// 	scope.reset_closure();
	// 	Ok(())
	// }
	// fn analyze_types(&mut self, scope: &mut Env) -> ASTResult<()> {
	// 	use_closure!(scope, self.closure());

	// 	for method in self.methods.iter_mut() {
	// 		method.analyze_types(scope)?;
	// 	}

	// 	scope.reset_closure();
	// 	Ok(())
	// }
	// fn check_types(&mut self, scope: &mut Env) -> ASTResult<()> {
	// 	use_closure!(scope, self.closure());

	// 	for method in self.methods.iter_mut() {
	// 		method.check_types(scope)?;
	// 	}

	// 	scope.reset_closure();
	// 	Ok(())
	// }
}

#[allow(unused_variables)]
impl TypeOf for Trait {
	fn type_of(&self, scope: &mut Env) -> ASTResult<Cow<Type>> {
		Ok(trait_type().into())
	}
	fn type_of_hint(&self, scope: &mut Env) -> ASTResult<Cow<Type>> {
		self.name.type_of_hint(scope)
	}
}