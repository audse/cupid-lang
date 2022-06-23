
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct Position {
	pub line: usize,
	pub character: usize,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct Span {
	pub start: Position,
	pub end: Position,
}