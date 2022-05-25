use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Exp {
	Declaration(Declaration),
	FunctionCall(FunctionCall),
	Block(Block),
	Function(Function),
	Ident(Ident),
	Value(Value),
	Empty
}

impl Default for Exp {
	fn default() -> Self {
    	Self::Empty
	}
}

macro_rules! for_each_exp {
	($s:ident, $method:tt, $scope:ident) => {
		match $s {
			Self::Declaration(declaration) => declaration.$method($scope),
			Self::FunctionCall(function_call) => function_call.$method($scope),
			Self::Block(block) => block.$method($scope),
			Self::Function(function) => function.$method($scope),
			Self::Ident(ident) => ident.$method($scope),
			Self::Value(value) => value.$method($scope),
			_ => panic!("unexpected expression: {:?}", $s)
		}
	};
	($s:ident, $method:tt) => {
		match $s {
			Self::Declaration(declaration) => declaration.$method(),
			Self::FunctionCall(function_call) => function_call.$method(),
			Self::Block(block) => block.$method(),
			Self::Function(function) => function.$method(),
			Self::Ident(ident) => ident.$method(),
			Self::Value(value) => value.$method(),
			_ => panic!("unexpected expression: {:?}", $s)
		}
	}
}

impl TypeOf for Exp {
	fn type_of(&self, scope: &mut Env) -> Result<Type, (Source, ErrCode)> {
		if let Self::Empty = self {
			return Ok(NOTHING.to_owned())
		}
		for_each_exp!(self, type_of, scope)
	}
}

impl Analyze for Exp {
	fn analyze_names(&mut self, scope: &mut Env) -> Result<(), (Source, ErrCode)> {
		if let Self::Empty = self { return Ok(()) }
		for_each_exp!(self, analyze_names, scope)
	}
	fn analyze_types(&mut self, scope: &mut Env) -> Result<(), (Source, ErrCode)> {
		if let Self::Empty = self { return Ok(()) }
		for_each_exp!(self, analyze_types, scope)
	}
	fn check_types(&mut self, scope: &mut Env) -> Result<(), (Source, ErrCode)> {
		if let Self::Empty = self { return Ok(()) }
		for_each_exp!(self, check_types, scope)
	}
}

impl UseAttributes for Exp {
	fn attributes(&mut self) -> &mut Attributes {
		for_each_exp!(self, attributes)
	}
}