use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Exp {
	Declaration(Declaration),
	FunctionCall(FunctionCall),
	Empty
}

impl Default for Exp {
	fn default() -> Self {
    	Self::Empty
	}
}

impl TypeOf for Exp {
	fn type_of(&self, scope: &mut Env) -> Type {		
    	match self {
			Self::Declaration(declaration) => declaration.type_of(scope),
			Self::FunctionCall(function_call) => function_call.type_of(scope),
			_ => panic!()
		}
	}
}

impl Analyze for Exp {
	fn resolve_names(&mut self, scope: &mut Env) -> Result<(), Error> {
    	match self {
			Self::Declaration(declaration) => declaration.resolve_names(scope),
			Self::FunctionCall(function_call) => function_call.resolve_names(scope),
			_ => panic!()
		}
	}
}