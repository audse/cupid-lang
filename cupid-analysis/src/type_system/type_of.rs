use crate::*;

#[allow(unused_variables)]
pub trait TypeOf {
	fn type_of(&self, scope: &mut Env) -> ASTResult<Cow<Type>> { 
		Ok(nothing_type().into())
	}
}