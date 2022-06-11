use crate::*;

impl PreAnalyze for Value {}

impl Analyze for Value {
    #[trace]
	fn analyze_scope(&mut self, scope: &mut Env) -> ASTResult<()> {
		self.attributes_mut().closure = scope.current_closure;
		match self {
			VArray(x, ..) | VTuple(x, ..) => x.iter_mut().try_for_each(|el| el.analyze_scope(scope)),
			VFunction(function, ..) => function.analyze_scope(scope),
			VTrait(trait_val) => trait_val.analyze_scope(scope),
			VType(type_val) => type_val.analyze_scope(scope),
			_ => Ok(())
		}
	}
    #[trace]
	fn analyze_names(&mut self, scope: &mut Env) -> ASTResult<()> {
		match self {
			VArray(x, ..) | VTuple(x, ..) => x.iter_mut().try_for_each(|el| el.analyze_names(scope)),
			VFunction(function, ..) => function.analyze_names(scope),
			VTrait(trait_val) => trait_val.analyze_names(scope),
			VType(type_val) => type_val.analyze_names(scope),
			_ => Ok(())
		}
	}
    #[trace]
	fn analyze_types(&mut self, scope: &mut Env) -> ASTResult<()> {
		match self {
			VArray(x, ..) | VTuple(x, ..) => x.iter_mut().try_for_each(|el| el.analyze_types(scope)),
			VFunction(function, ..) => function.analyze_types(scope),
			VTrait(trait_val) => trait_val.analyze_types(scope),
			VType(type_val) => type_val.analyze_types(scope),
			_ => Ok(())
		}
	}
    #[trace]
	fn check_types(&mut self, scope: &mut Env) -> ASTResult<()> {
		match self {
			VArray(x, ..) | VTuple(x, ..) => x.iter_mut().try_for_each(|el| el.check_types(scope)),
			VFunction(function, ..) => function.check_types(scope),
			VTrait(trait_val) => trait_val.check_types(scope),
			VType(type_val) => type_val.check_types(scope),
			_ => Ok(())
		}
	}
}

impl TypeOf for Value {
	fn type_of(&self, scope: &mut Env) -> ASTResult<Cow<Type>> {
		Ok(self.infer(scope)?.into())
	}
}