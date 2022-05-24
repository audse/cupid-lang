use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Exp {
	Declaration(Declaration),
	FunctionCall(FunctionCall),
	Block(Block),
	Function(Function),
	Value(Value),
	Empty
}

impl Default for Exp {
	fn default() -> Self {
    	Self::Empty
	}
}

impl TypeOf for Exp {
	fn type_of(&self, scope: &mut Env) -> Result<Type, ErrCode> {		
    	match self {
			Self::Declaration(declaration) => declaration.type_of(scope),
			Self::FunctionCall(function_call) => function_call.type_of(scope),
			Self::Block(block) => block.type_of(scope),
			Self::Function(function) => function.type_of(scope),
			Self::Value(value) => value.type_of(scope),
			_ => panic!()
		}
	}
}

impl Analyze for Exp {
	fn analyze_names(&mut self, scope: &mut Env) -> Result<(), ErrCode> {
    	match self {
			Self::Declaration(declaration) => declaration.analyze_names(scope),
			Self::FunctionCall(function_call) => function_call.analyze_names(scope),
			Self::Block(block) => block.analyze_names(scope),
			Self::Function(function) => function.analyze_names(scope),
			Self::Value(value) => value.analyze_names(scope),
			_ => panic!()
		}
	}
	fn analyze_types(&mut self, scope: &mut Env) -> Result<(), ErrCode> {
    	match self {
			Self::Declaration(declaration) => declaration.analyze_types(scope),
			Self::FunctionCall(function_call) => function_call.analyze_types(scope),
			Self::Block(block) => block.analyze_types(scope),
			Self::Function(function) => function.analyze_types(scope),
			Self::Value(value) => value.analyze_types(scope),
			_ => panic!()
		}
	}
}

impl UseAttributes for Exp {
	fn attributes(&mut self) -> &mut Attributes {
    	match self {
			Self::Declaration(declaration) => declaration.attributes(),
			Self::FunctionCall(function_call) => function_call.attributes(),
			Self::Block(block) => block.attributes(),
			Self::Function(function) => function.attributes(),
			Self::Value(value) => value.attributes(),
			_ => panic!()
		}
	}
}