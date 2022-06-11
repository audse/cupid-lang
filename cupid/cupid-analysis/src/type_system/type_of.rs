use crate::*;

#[allow(unused_variables)]
pub trait TypeOf {
	fn type_of(&self, scope: &mut Env) -> ASTResult<Cow<Type>> { 
		Ok(Type::none().into())
	}
	fn type_of_hint(&self, scope: &mut Env) -> ASTResult<Cow<Type>> {
		Ok(Type::type_ty().into())
	}
}

pub trait GetNodeType {
	fn get_node_type(&self) -> ASTResult<&Type>;
}

impl<T: ErrorContext + Default> GetNodeType for Typed<T> {
	fn get_node_type(&self) -> ASTResult<&Type> {
		match self {
			IsTyped(_, t) => Ok(t),
			Untyped(_) => self.to_err(ERR_EXPECTED_TYPE)
		}
	}
}