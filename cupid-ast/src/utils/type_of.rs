use crate::*;

pub trait TypeOf {
	fn type_of(&self, _scope: &mut Env) -> Result<Type, ASTErr> { 
		Ok(NOTHING.to_owned())
	}
}