use std::fmt::{Display, Formatter, Result};
use std::hash::{Hash, Hasher};
use crate::{TypeSymbol, Symbol, Type};
use super::builtin::*;

#[derive(Debug, Clone)]
pub struct ProductType {
	pub symbol: TypeSymbol,
	pub fields: Vec<(TypeSymbol, Option<Symbol>)>,
}

impl PartialEq for ProductType {
	fn eq(&self, other: &Self) -> bool {
		let eq = self.symbol.name == other.symbol.name;
		for (i, field) in self.fields.iter().enumerate() {
			if &other.fields[i] != field {
				return false;
			}
		}
		eq
	}
}

impl Eq for ProductType {}

impl Hash for ProductType {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.symbol.hash(state);
		self.fields.iter().for_each(|(_, s)| s.hash(state));
	}
}

impl ProductType {
	pub fn get_name(&self) -> String {
		self.symbol.name.to_string()
	}
	pub fn is(&self, name: &str) -> bool {
		self.symbol.name == name
	}
	pub fn from_symbol(symbol: &TypeSymbol) -> Self {
		Self {
			symbol: symbol.clone(),
			fields: symbol.arguments.iter().cloned().map(|f| (f, None)).collect()
		}
	}
}

impl Display for ProductType {
	fn fmt(&self, f: &mut Formatter) -> Result {
		let fields: Vec<String> = self.fields
			.iter()
			.map(|(symbol, identifier)| format!("{} {}", symbol, if identifier.is_some() { identifier.clone().unwrap().to_string() } else { String::new() }))
			.collect();
		write!(f, "{}: ({})", self.symbol, fields.join(", "))
	}
}