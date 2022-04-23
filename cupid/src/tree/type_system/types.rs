use std::fmt::{Display, Formatter, Result};
use crate::{TypeSymbol, SumType, Token, Value, ProductType, Tree, Symbol, LexicalScope, SymbolFinder, ErrorHandler};
use super::builtin::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
	Sum(SumType),
	Product(ProductType),
}

impl Type {
	pub const fn new_const_product(name: &'static str, fields: Vec<(TypeSymbol, Option<Symbol>)>) -> Self {
		Self::Product(ProductType {
			symbol: TypeSymbol::new_const(name),
			fields
		})
	}
	pub fn new_product(name: &str, fields: Vec<(TypeSymbol, Option<Symbol>)>) -> Self {
		Self::Product(ProductType {
			symbol: TypeSymbol::new_simple(name, vec![]),
			fields
		})
	}
	pub fn from(value: &Value) -> Type {
		match value {
			Value::Boolean(_) => BOOLEAN,
			Value::Integer(_) => INTEGER,
			Value::Decimal(_, _) => DECIMAL,
			Value::Char(_) => CHAR,
			Value::Array(a) => {
				let inner_type = if a.len() > 0 {
					Type::from(&a[0]).get_symbol().clone()
				} else {
					GENERIC
				};
				Type::new_product("array", vec![(inner_type, None)])
			},
			Value::Map(m) => {
				let (key_type, value_type) = {
					if let Some((key, (_, value))) = m.iter().next() {
						(
							Type::from(&key).get_symbol().clone(), 
							Type::from(&value).get_symbol().clone()
						)
					} else {
						(GENERIC, GENERIC)
					}
				};
				Type::new_product("map", vec![(key_type, None), (value_type, None)])
			},
			Value::ProductMap(symbol, m) => {
				let fields: Vec<(TypeSymbol, Option<Symbol>)> = {
					m.iter().map(|(key, value)| {
						let symbol = match Type::from(&value) {
							Type::Product(product_type) => product_type,
							_ => unreachable!()
						};
						(symbol.to_symbol(), Some(key.clone()))
					})
					.collect()
				};
				Type::new_product(&symbol.name, fields)
			},
			
			Value::String(_) => STRING,
			Value::FunctionBody(_, _) => FUNCTION,
			// Value::Dictionary(_) => DICTIONARY,
			// Value::List(_) => LIST,
			// Value::Tuple(_) => TUPLE,
			Value::MapEntry(_, _) => MAP_ENTRY,
			Value::Error(_) => ERROR,
			_ => NONE
		}
	}
	pub fn is_builtin(&self) -> bool {
		// todo: account for generics
		vec![&BOOLEAN, &INTEGER, &DECIMAL, &STRING, &FUNCTION, &DICTIONARY, &LIST, &TUPLE, &NONE, &ARRAY, &CHAR, &ERROR].contains(&self)
	}
	// is either a builtin map type or a struct type
	pub fn is_map(&self) -> bool {
		match self {
			Type::Product(map) => !self.is_builtin() || ["map", "array"].contains(&map.get_name().as_str()),
			_ => false
		}
	}
	pub fn get_symbol(&self) -> &TypeSymbol {
		match self {
			Self::Product(product_type) => &product_type.symbol,
			_ => unreachable!()
		}
	}
	pub fn eq_approx(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::Product(left), Self::Product(right)) => {
				if left.get_name() != right.get_name() && self.is_builtin() {
					return false;
				}
				for (i, field) in left.fields.iter().enumerate() {
					if field.0.generic {
						continue;
					}
					if &right.fields[i] != field {
						return false;
					}
				}
				true
			},
			_ => false
		}
	}
	pub fn apply_arguments(&mut self, arguments: &[TypeSymbol]) {
		match self {
			Self::Product(product_type) => {
				for (i, field) in product_type.fields.iter_mut().enumerate() {
					if field.0.generic && arguments.len() > i {
						field.0 = arguments[i].clone()
					}
				}
			},
			_ => ()
		}
	}
}

impl Display for Type {
	fn fmt(&self, f: &mut Formatter) -> Result {
		_ = match self {
			Type::Product(product_type) => write!(f, "product {}", product_type),
			Type::Sum(sum_type) => write!(f, "sum {}", sum_type)
		};
		Ok(())
	}
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DefineType {
	pub token: Token,
	pub type_symbol: TypeSymbol,
	pub type_value: Type,
}

impl Tree for DefineType {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
    	match scope.define_type(&self.type_symbol, self.type_value.clone()) {
			Ok(new_type) => Value::Type(new_type),
			Err(error) => error
		}
	}
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DefineTypeAlias {
	pub token: Token,
	pub type_symbol: TypeSymbol,
	pub arguments: Vec<TypeSymbol>,
}

impl Tree for DefineTypeAlias {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
		if let Some(mut stored_type) = scope.get_definition(&self.arguments[0]) {
			stored_type.apply_arguments(&self.arguments[0].arguments);
			match scope.define_type(&self.type_symbol, stored_type) {
				Ok(new_type) => Value::Type(new_type),
				Err(error) => error
			}
		} else {
			self.error(format!("cannot create a type alias for a non-existent type ({})", self.arguments[0]))
		}
	}
}

impl ErrorHandler for DefineTypeAlias {
	fn get_token(&self) -> &Token {
		&self.token
	}
	fn get_context(&self) -> String {
		let args: Vec<String> = self.arguments.iter().map(|a| a.to_string()).collect();
		format!(
			"defining a type alias for {} with arguments {:?}", 
			self.type_symbol, 
			args.join(", ")
		)
	}
}