use crate::*;

impl PreAnalyze for Value {}

impl Analyze for Value {
	fn analyze_scope(&mut self, scope: &mut Env) -> ASTResult<()> {
		match self {
			VArray(x, ..) | VTuple(x, ..) => x.iter_mut().map(|el| el.analyze_scope(scope)).collect(),
			VFunction(function, ..) => function.analyze_scope(scope),
			VTrait(trait_val) => trait_val.analyze_scope(scope),
			VType(type_val) => type_val.analyze_scope(scope),
			_ => Ok(())
		}
	}
	fn analyze_names(&mut self, scope: &mut Env) -> ASTResult<()> {
		match self {
			VArray(x, ..) | VTuple(x, ..) => x.iter_mut().map(|el| el.analyze_names(scope)).collect(),
			VFunction(function, ..) => function.analyze_names(scope),
			VTrait(trait_val) => trait_val.analyze_names(scope),
			VType(type_val) => type_val.analyze_names(scope),
			_ => Ok(())
		}
	}
	fn analyze_types(&mut self, scope: &mut Env) -> ASTResult<()> {
		match self {
			VArray(x, ..) | VTuple(x, ..) => x.iter_mut().map(|el| el.analyze_types(scope)).collect(),
			VFunction(function, ..) => function.analyze_types(scope),
			VTrait(trait_val) => trait_val.analyze_types(scope),
			VType(type_val) => type_val.analyze_types(scope),
			_ => Ok(())
		}
	}
	fn check_types(&mut self, scope: &mut Env) -> ASTResult<()> {
		match self {
			VArray(x, ..) | VTuple(x, ..) => x.iter_mut().map(|el| el.check_types(scope)).collect(),
			VFunction(function, ..) => function.check_types(scope),
			VTrait(trait_val) => trait_val.check_types(scope),
			VType(type_val) => type_val.check_types(scope),
			_ => Ok(())
		}
	}
}

impl TypeOf for Value {
	fn type_of(&self, scope: &mut Env) -> ASTResult<Cow<Type>> {
		infer_type(self, scope).map(|t| t.into())
	}
}

// impl InferType for Box<dyn InferType> {
// 	fn infer(&self, scope: &mut Env) -> ASTResult<Type> {
// 		(**self).infer(scope)
// 	}
// }