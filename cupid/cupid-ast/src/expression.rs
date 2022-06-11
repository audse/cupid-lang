use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Unwrap)]
pub enum Exp {
	Block(Block),
	Declaration(Declaration),
	Empty,
	Function(Function),
	FunctionCall(Box<FunctionCall>),
	Ident(Ident),
	Implement(Implement),
	Property(Box<Property>),
	TraitDef(TraitDef),
	TypeDef(TypeDef),
	Value(Value),
}

impl Default for Exp {
	fn default() -> Self {
    	Self::Empty
	}
}

#[macro_export]
macro_rules! for_each_exp {
	($s:ident => $closure:expr) => {
		match $s {
			Exp::Block(block) => $closure(block),
			Exp::Declaration(declaration) => $closure(declaration),
			Exp::Function(function) => $closure(function),
			Exp::FunctionCall(function_call) => $closure(&**function_call),
			Exp::Ident(ident) => $closure(ident),
			Exp::Implement(implement) => $closure(implement),
			Exp::Property(property) => $closure(&**property),
			Exp::TraitDef(trait_def) => $closure(trait_def),
			Exp::TypeDef(type_def) => $closure(type_def),
			Exp::Value(value) => $closure(value),
			_ => panic!("unexpected expression: {:?}", $s)
		}
	};
	($s:ident, $method:tt $(,)? $($arg:expr),*) => {
		match $s {
			Exp::Block(block) => block.$method($($arg),*),
			Exp::Declaration(declaration) => declaration.$method($($arg),*),
			Exp::Function(function) => function.$method($($arg),*),
			Exp::FunctionCall(function_call) => function_call.$method($($arg),*),
			Exp::Ident(ident) => ident.$method($($arg),*),
			Exp::Implement(implement) => implement.$method($($arg),*),
			Exp::Property(property) => property.$method($($arg),*),
			Exp::TraitDef(trait_def) => trait_def.$method($($arg),*),
			Exp::TypeDef(type_def) => type_def.$method($($arg),*),
			Exp::Value(value) => value.$method($($arg),*),
			_ => panic!("unexpected expression: {:?}", $s)
		}
	};
}

impl UseAttributes for Exp {
	fn attributes(&self) -> &Attributes {
		for_each_exp!(self, attributes)
	}
	fn attributes_mut(&mut self) -> &mut Attributes {
		for_each_exp!(self, attributes_mut)
	}
}