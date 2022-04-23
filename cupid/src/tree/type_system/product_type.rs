use std::fmt::{Display, Formatter, Result};
use std::hash::{Hash, Hasher};
use crate::{TypeSymbol, Symbol};

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
	pub fn to_symbol(&self) -> TypeSymbol {
		let arguments: Vec<TypeSymbol> = self.fields
			.iter()
			.map(|(field, _)| field.clone())
			.collect();
		TypeSymbol {
			name: self.symbol.name.clone(),
			token: self.symbol.token.clone(),
			generic: self.symbol.generic,
			arguments
		}
	}
}

impl Display for ProductType {
	fn fmt(&self, f: &mut Formatter) -> Result {
		
		let fields: Vec<String> = self.fields
			.iter()
			.map(|(symbol, identifier)| {
				let args: Vec<String> = symbol.arguments.iter().map(|a| a.name.to_string()).collect();
				format!(
					"{}: {} ({})", 
					if identifier.is_some() { 
						identifier.clone().unwrap().get_identifier() 
					} else { 
						String::new() 
					},
					symbol.name,
					args.join(", ")
				)
			})
			.collect();
		write!(f, "{}: [{}]", self.symbol, fields.join(", "))
	}
}