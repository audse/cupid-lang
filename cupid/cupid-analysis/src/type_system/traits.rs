use crate::*;

impl PreAnalyze for Trait {}
impl Analyze for Trait {}

#[allow(unused_variables)]
impl TypeOf for Trait {
	fn type_of(&self, scope: &mut Env) -> ASTResult<Cow<Type>> {
		Ok(Type::trait_ty().into())
	}
	fn type_of_hint(&self, scope: &mut Env) -> ASTResult<Cow<Type>> {
		self.name.type_of_hint(scope)
	}
}