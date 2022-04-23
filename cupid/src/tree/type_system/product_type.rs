// use std::fmt::{Display, Formatter, Result};
use std::borrow::Cow;
// use std::hash::{Hash, Hasher};
use crate::{Symbol};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PrimitiveType {
	pub identifier: Cow<'static, str>,
}

impl PrimitiveType {
	pub const fn new(identifier: &'static str) -> Self {
		PrimitiveType { identifier: Cow::Borrowed(identifier) }
	}
}

impl Type for PrimitiveType {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayType {
	pub item_type: dyn Type,
	pub generic: Option<GenericType>
}

impl Type for ArrayType {
	fn apply_arguments(&mut self, arguments: &[impl Type]) -> Result<(), ()> {
    	if let Some(generic) = self.generic {
			if arguments.len() >= 1 {
				self.item_type = arguments[0];
			} else {
				Err(())
			}
		}
		Ok(())
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MapType {
	pub key_type: dyn Type,
	pub value_type: dyn Type,
	pub key_generic: Option<GenericType>,
	pub value_generic: Option<GenericType>,
}

impl Type for MapType {
	fn apply_arguments(&mut self, arguments: &[impl Type]) -> Result<(), ()> {
    	if let Some(key_generic) = self.key_generic {
			if arguments.len() >= 1 {
				self.key_type = arguments[0];
			} else {
				Err(())
			}
		}
		if let Some(value_generic) = self.value_generic {
			if arguments.len() > 1 {
				self.value_type = arguments[1];
			} else {
				Err(())
			}
		}
		Ok(())
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StructType {
	pub identifier: Symbol,
	pub members: Vec<(Symbol, dyn Type)>,
	pub generics: Vec<GenericType>
}

impl Type for StructType {
	fn apply_arguments(&mut self, arguments: &[impl Type]) -> Result<(), ()> {
		if self.generics.len() != arguments.len() {
			Err(())
		} else {
			for (_, member) in self.members.iter() {
				match member.apply_arguments(arguments) {
					Ok(_) => continue,
					Err(_) => return Err(())
				}
			}
			Ok(())
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SumType {
	pub identifier: Symbol,
	pub members: Vec<dyn Type>,
	pub generics: Vec<GenericType>
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AliasType {
	pub identifier: Symbol,
	pub true_type: dyn Type,
	pub generics: Vec<GenericType>
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GenericType {
	pub identifier: Cow<'static, str>,
}

impl GenericType {
	pub const fn new_const(identifier: &'static str) -> Self {
		GenericType { identifier: Cow::Borrowed(identifier) }
	}
	pub fn new(identifier: &str) -> Self {
		GenericType { identifier: Cow::Owned(identifier) }
	}
}

pub trait Type {
	fn apply_arguments(&mut self, arguments: &[impl Type]) -> Result<(), ()> {
		Ok(())
	}
}

/*

1. Define a type
	a. Generic type identifiers as arguments
	b. Recursively resolve all members
		- If the type name is in the list of generics, use GenericType
		- Otherwise, resolve type from storage and throw error if undefined

2. Use a type
	a. Recursively apply arguments to GenericType in members
		- Map arguments to generics in order
		- For any member that is that generic, replace with the matched argument
	b. Throw an error if any generics are not resolved
		- This may get tricky because there will also be generics in functions, etc

*/




// 
// impl PartialEq for ProductType {
// 	fn eq(&self, other: &Self) -> bool {
// 		let eq = self.symbol.name == other.symbol.name;
// 		for (i, field) in self.fields.iter().enumerate() {
// 			if &other.fields[i] != field {
// 				return false;
// 			}
// 		}
// 		eq
// 	}
// }
// 
// impl Eq for ProductType {}
// 
// impl Hash for ProductType {
// 	fn hash<H: Hasher>(&self, state: &mut H) {
// 		self.symbol.hash(state);
// 		self.fields.iter().for_each(|(_, s)| s.hash(state));
// 	}
// }
// 
// impl ProductType {
// 	pub fn get_name(&self) -> String {
// 		self.symbol.name.to_string()
// 	}
// 	pub fn is(&self, name: &str) -> bool {
// 		self.symbol.name == name
// 	}
// 	pub fn from_symbol(symbol: &TypeSymbol) -> Self {
// 		Self {
// 			symbol: symbol.clone(),
// 			fields: symbol.arguments.iter().cloned().map(|f| (f, None)).collect()
// 		}
// 	}
// 	pub fn to_symbol(&self) -> TypeSymbol {
// 		let arguments: Vec<TypeSymbol> = self.fields
// 			.iter()
// 			.map(|(field, _)| field.clone())
// 			.collect();
// 		TypeSymbol {
// 			name: self.symbol.name.clone(),
// 			token: self.symbol.token.clone(),
// 			generic: self.symbol.generic,
// 			arguments
// 		}
// 	}
// }
// 
// impl Display for ProductType {
// 	fn fmt(&self, f: &mut Formatter) -> Result {
// 		
// 		let fields: Vec<String> = self.fields
// 			.iter()
// 			.map(|(symbol, identifier)| {
// 				let args: Vec<String> = symbol.arguments.iter().map(|a| a.name.to_string()).collect();
// 				format!(
// 					"{}: {} ({})", 
// 					if identifier.is_some() { 
// 						identifier.clone().unwrap().get_identifier() 
// 					} else { 
// 						String::new() 
// 					},
// 					symbol,
// 					args.join(", ")
// 				)
// 			})
// 			.collect();
// 		write!(f, "{}: [{}]", self.symbol, fields.join(", "))
// 	}
// }