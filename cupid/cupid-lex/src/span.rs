use std::cmp::Ordering;


#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Position {
	pub line: usize,
	pub character: usize,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Span {
	pub start: Position,
	pub end: Position,
}

impl PartialOrd for Position {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		let line_cmp = self.line.cmp(&other.line);
		Some(if line_cmp == Ordering::Equal {
			self.character.cmp(&other.character)
		} else {
			line_cmp
		})
	}
}

impl Ord for Position {
	fn cmp(&self, other: &Self) -> Ordering {
		self.partial_cmp(other).unwrap()
	}
}

impl PartialOrd for Span {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		let start_cmp = self.start.cmp(&other.start);
		Some(if start_cmp == Ordering::Equal {
			self.end.cmp(&other.end)
		} else {
			start_cmp
		})
	}
}

impl Ord for Span {
	fn cmp(&self, other: &Self) -> Ordering {
		self.partial_cmp(other).unwrap()
	}
}