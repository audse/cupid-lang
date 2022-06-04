use crate::*;

#[allow(unused_variables)]
pub trait TypeOf {
	fn type_of(&self, scope: &mut Env) -> ASTResult<Cow<'_, Type>> { 
		Ok((&*NOTHING).into())
	}
}

impl From<Type> for Cow<'_, Type> {
	fn from(t: Type) -> Self {
		Cow::Owned(t)
	}
}

impl<'t> From<&'t Type> for Cow<'t, Type> {
	fn from(t: &'t Type) -> Self {
		Cow::Borrowed(t)
	}
}
