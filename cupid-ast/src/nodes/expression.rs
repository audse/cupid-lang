use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Unwrap, Tabled)]
pub enum Exp {
	Block(Block),
	Declaration(Declaration),
	Empty,
	Function(Function),
	FunctionCall(Box<FunctionCall>),
	Ident(Ident),
	Property(Box<Property>),
	TypeDefinition(TypeDefinition),
	Value(Value),
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
			Self::Block(block) => block.$method($($arg)?),
			Self::Declaration(declaration) => declaration.$method($($arg)?),
			Self::Function(function) => function.$method($($arg)?),
			Self::FunctionCall(function_call) => function_call.$method($($arg)?),
			Self::Ident(ident) => ident.$method($($arg)?),
			Self::Property(property) => property.$method($($arg)?),
			Self::TypeDefinition(property) => property.$method($($arg)?),
			Self::Value(value) => value.$method($($arg)?),
			_ => panic!("unexpected expression: {:?}", $s)
		}
	};
}