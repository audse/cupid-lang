use crate::*;
use tabled::Tabled;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Unwrap, Tabled)]
pub enum Exp {
	Declaration(Declaration),
	FunctionCall(FunctionCall),
	Block(Block),
	Function(Function),
	Property(Box<Property>),
	Ident(Ident),
	Value(Value),
	Empty
}

impl Default for Exp {
	fn default() -> Self {
    	Self::Empty
	}
}

#[macro_export]
macro_rules! for_each_exp {
	($s:ident, $method:tt $(, $arg:expr)?) => {
		match $s {
			Self::Declaration(declaration) => declaration.$method($($arg)?),
			Self::FunctionCall(function_call) => function_call.$method($($arg)?),
			Self::Block(block) => block.$method($($arg)?),
			Self::Function(function) => function.$method($($arg)?),
			Self::Property(property) => property.$method($($arg)?),
			Self::Ident(ident) => ident.$method($($arg)?),
			Self::Value(value) => value.$method($($arg)?),
			_ => panic!("unexpected expression: {:?}", $s)
		}
	};
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