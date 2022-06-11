use crate::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

impl std::fmt::Display for Context {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", fmt_type!(Self))
	}
}