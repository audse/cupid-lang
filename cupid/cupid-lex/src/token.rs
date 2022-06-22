use std::{
	borrow::Cow,
	fmt::{
		Formatter,
		Result as DisplayResult,
		Display,
	},
};
use crate::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Default)]
pub struct Token {
	pub source: Cow<'static, str>,
	pub line: usize,
	pub index: usize,
	pub end_line: usize,
	pub end_index: usize,
	pub file: usize,
}

type LineNum = usize;
type Tabs = String;

impl Token {
	pub fn source(&self) -> &str {
		&self.source
	}
	pub fn combine(mut self, token: Token) -> Self {
		self.source += token.source;
		self.end_line = std::cmp::max(self.end_line, token.end_line);
		self.end_index = std::cmp::max(self.end_index, token.end_index);
		self
	}
	pub fn lines<'src>(&'src self, source: &'src str) -> Vec<(LineNum, Tabs, &'src str)> {
		let slice = self.line-1..self.end_line;
		let lines = &(source.lines().collect::<Vec<&str>>());
		lines[slice]
			.iter()
			.enumerate()
			.map(|(i, line)| {
				let tabs = line.rsplit_once('\t').map(|(p, _)| p.to_string() + "\t").unwrap_or_default();
				let line = line.trim_start_matches('\t');
				(i + self.line, tabs, line)
			})
			.collect::<Vec<(LineNum, Tabs, &str)>>()
	}
	pub fn underline(&self, lines: Vec<(LineNum, Tabs, &str)>) -> Vec<(String, String)> {
		lines
			.iter()
			.map(|(line_num, tabs, line)| {
				let divider = " | ".dimmed();

				let line_num_str = line_num.to_string();
				let line_num_offset = line_num_str.len();

				let current_line = format!("{}{divider}{tabs}{}", line_num_str.dimmed(), line.bold());

				let underline_len = line.len() - tabs.len();
				let underline = format!(
				"{0:line_num_offset$}{divider}{tabs}{1:2$}{3}",
					"",
					"",
					0, // TODO self.index
					format!("{:^<underline_len$}", "").red().bold()
				);
				(current_line, underline)
			})
			.collect()
	}
}

impl Display for Token {
	fn fmt(&self, f: &mut Formatter) -> DisplayResult {
		writeln!(f, "`{}`", self.source)
	}
}
