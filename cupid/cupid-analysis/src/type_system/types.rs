use crate::*;

impl PreAnalyze for Type {}
impl Analyze for Type {}

#[allow(unused_variables)]
impl TypeOf for Type {
	fn type_of(&self, scope: &mut Env) -> ASTResult<Cow<Type>> {
		Ok(Type::type_ty().into())
	}
	fn type_of_hint(&self, scope: &mut Env) -> ASTResult<Cow<Type>> {
		self.name.type_of_hint(scope)
	}
}