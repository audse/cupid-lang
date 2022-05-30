use crate::{
	Display,
	Tabled,
};

#[derive(Debug, Clone, PartialEq, Eq, Display, Tabled)]
pub enum Context {
	Global,
	Closure,
	Block,
	Loop,
	Type,
	Trait,
	Method,
	Function,
}

impl Default for Context {
	fn default() -> Self {
		Self::Block
	}
}