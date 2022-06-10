use crate::*;

#[allow(unused_variables)]
pub trait TypeOf {
	fn type_of(&self, scope: &mut Env) -> ASTResult<Cow<Type>> { 
		Ok(nothing_type().into())
	}
	fn type_of_hint(&self, scope: &mut Env) -> ASTResult<Cow<Type>> {
		Ok(type_type().into())
	}
}