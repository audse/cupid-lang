use crate::*;

pub trait TypeOf {
	fn type_of(&self, _scope: &mut Env) -> Result<Type, (Source, ErrCode)> { 
		Ok(NOTHING.to_owned())
	}
}