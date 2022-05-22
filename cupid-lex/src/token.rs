use crate::*;

#[derive(PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct Token {
	pub source: Cow<'static, str>,
	pub line: usize,
	pub index: usize,
	pub end_line: usize,
	pub end_index: usize,
	pub file: usize,
}

impl Token {
	pub fn source(&self) -> &str {
		&self.source
	}
}

impl Debug for Token {
	fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
		write!(f, "(\"{}\", {}:{})", self.source, self.line, self.index)
	}
}

impl Display for Token {
	fn fmt(&self, f: &mut Formatter) -> DisplayResult {
		writeln!(f, "`{}`", self.source)
	}
}

pub struct TokenList(Vec<Token>);

impl Display for TokenList {
	fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
		_ = writeln!(f, "Tokens:");
		self.0.iter().for_each(|t| { _ = write!(f, "{}", t); });
		_ = writeln!(f);
		Ok(())
	}
}