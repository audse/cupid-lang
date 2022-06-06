use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Unwrap, Tabled)]
pub enum Exp<'ast> {
	Block(Block<'ast>),
	Declaration(Declaration<'ast>),
	Empty,
	Function(Function<'ast>),
	FunctionCall(Box<FunctionCall<'ast>>),
	Ident(Ident),
	Implement(Implement<'ast>),
	Property(Box<Property<'ast>>),
	TraitDef(TraitDef<'ast>),
	TypeDef(TypeDef<'ast>),
	Value(BoxValue<'ast>),
}

impl Default for Exp<'_> {
	fn default() -> Self {
    	Self::Empty
	}
}

#[macro_export]
macro_rules! for_each_exp {
	($s:ident, $method:tt $(,)? $($arg:expr),*) => {
		match $s {
			Self::Block(block) => block.$method($($arg),*),
			Self::Declaration(declaration) => declaration.$method($($arg),*),
			Self::Function(function) => function.$method($($arg),*),
			Self::FunctionCall(function_call) => function_call.$method($($arg),*),
			Self::Ident(ident) => ident.$method($($arg),*),
			Self::Implement(implement) => implement.$method($($arg),*),
			Self::Property(property) => property.$method($($arg),*),
			Self::TraitDef(trait_def) => trait_def.$method($($arg),*),
			Self::TypeDef(type_def) => type_def.$method($($arg),*),
			Self::Value(value) => value.$method($($arg),*),
			_ => panic!("unexpected expression: {:?}", $s)
		}
	};
}