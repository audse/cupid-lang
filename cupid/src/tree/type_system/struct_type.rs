use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as DisplayResult};
use std::hash::{Hash, Hasher};
use serde::{Serialize, Deserialize};
use crate::{TypeKind, Type, Symbol, GenericType, Expression, Tree, Value, SymbolFinder, ErrorHandler, Token, ScopeContext};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructType {
	pub members: Vec<(Symbol, TypeKind)>,
	pub implement: HashMap<Value, Value>
}

impl StructType {
	pub fn is_map_equal(&self, other: &Value) -> bool {
		// todo
		match other {
			Value::Map(x) => {
				x.iter().all(|(key, (_, value))| {
					if let Some((_, member_type)) = self.members.iter().find(|(symbol, _)| &symbol.identifier == key)  {
						member_type.is_equal(&value)
					} else {
						false
					}
				})
			},
			_ => false
		}
	}
}

impl Type for StructType {
	fn apply_arguments(&mut self, arguments: &[GenericType]) -> Result<(), String> {
		for (_, member) in self.members.iter_mut() {
			match member.apply_arguments(arguments) {
				Ok(_) => continue,
				Err(msg) => return Err(msg)
			}
		}
		Ok(())
	}
	fn convert_primitives_to_generics(&mut self, generics: &[GenericType]) {
    	for (_, member) in self.members.iter_mut() {
			member.convert_primitives_to_generics(generics)
		}
	}
}

impl PartialEq for StructType {
	fn eq(&self, other: &Self) -> bool {
		self.members == other.members
	}
}

impl Eq for StructType {}

impl Hash for StructType {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.members.hash(state);
	}
}

impl Display for StructType {
	fn fmt(&self, f: &mut Formatter) -> DisplayResult {
		let members: Vec<String> = self.members
			.iter()
			.map(|(symbol, member)| format!("{symbol}: {member}"))
			.collect();
		write!(f, "[{}]", members.join(", "))
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DefineStruct {
	pub token: Token,
	pub symbol: Symbol,
	pub members: Vec<(Symbol, Expression)>,
	pub generics: Vec<Symbol>
}

impl Tree for DefineStruct {
	fn resolve(&self, scope: &mut crate::LexicalScope) -> Value {
		scope.add(ScopeContext::Map);
		self.define_generics(scope);
		
		let members: Vec<(Symbol, TypeKind)> = self.members
			.iter()
			.filter_map(|(symbol, exp)| {
				let val_type = exp.resolve(scope);
				if let Value::Type(mut member_type) = val_type {
					member_type.convert_primitives_to_generics(&self.resolve_generics());
					Some((symbol.clone(), member_type))
				} else {
					None
				}
			})
			.collect();
		let new_struct = TypeKind::Struct(StructType { 
			members,
			implement: HashMap::new()
		});
		scope.pop();
		if let Some(new_struct) = scope.define_type(&self.symbol, new_struct) {
			new_struct
		} else {
			self.error(String::from("unable to define type"))
		}
	}
}

impl ErrorHandler for DefineStruct {
	fn get_token(&self) -> &Token {
		&self.token
	}
	fn get_context(&self) -> String {
    	format!("defining type {} with members {:?}", self.symbol, self.members)
	}
}

impl DefineStruct {
	fn resolve_generics(&self) -> Vec<GenericType> {
		self.generics
			.iter()
			.map(|g| GenericType::new(&g.get_identifier(), None))
			.collect()
	}
	fn define_generics(&self, scope: &mut crate::LexicalScope) {
		let generics: Vec<(&Symbol, GenericType)> = self.generics
			.iter()
			.map(|g| (g, GenericType::new(&g.get_identifier(), None)))
			.collect();
		for (symbol, generic) in generics {
			scope.define_type(symbol, TypeKind::Generic(generic));
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StructTypeHint {
	pub token: Token,
	pub struct_type: Box<Expression>,
	pub member_args: Vec<(Symbol, Expression)>,
}

impl Tree for StructTypeHint {
	fn resolve(&self, scope: &mut crate::LexicalScope) -> Value {
		let struct_type = crate::resolve_or_abort!(self.struct_type, scope);
		if let Value::Type(mut struct_type) = struct_type {
			let member_args: Vec<GenericType> = self.member_args
				.iter()
				.filter_map(|(symbol, member_type)| {
					let member_type = member_type.resolve(scope);
					if let Value::Type(member_type) = member_type {
						let generic = GenericType::new(
							&symbol.get_identifier(),
							Some(Box::new(member_type)),
						);
						Some(generic)
					} else {
						None
					}
				})
				.collect();
			match struct_type.apply_arguments(&member_args) {
				Ok(_) => Value::Type(struct_type),
				Err(msg) => self.error(msg)
			}
		} else {
			self.error("not a struct")
		}
	}
}

impl ErrorHandler for StructTypeHint {
	fn get_token(&self) -> &Token {
		&self.token
	}
	fn get_context(&self) -> String {
		format!("struct type {} with args {:?}", self.struct_type, self.member_args)
	}
}