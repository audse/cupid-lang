use std::fmt::{Display, Formatter, Result};
use crate::{TypeSymbol};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SumType {
	pub symbols: Vec<TypeSymbol>
}

impl Display for SumType {
	fn fmt(&self, f: &mut Formatter) -> Result {
		let type_strings: Vec<String> = self.symbols.iter().map(|s| s.to_string()).collect();
		write!(f, "[{}]", type_strings.join(", "))
	}
}