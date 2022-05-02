use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as DisplayResult};
use std::hash::{Hash, Hasher};
use serde::{Serialize, Deserialize};
use crate::{TypeKind, Type, Symbol, GenericType, Expression, Tree, Value, SymbolFinder, ErrorHandler, Token, ScopeContext, UseGenerics, Implementation, LexicalScope};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SumType {
	pub types: Vec<TypeKind>,
	pub implementation: Implementation
}

impl SumType {
	pub fn contains(&self, other: &Value) -> bool {
		self.types
			.iter()
			.find(|t| t.is_equal(other))
			.is_some()
	}
}

impl Type for SumType {
	fn apply_arguments(&mut self, arguments: &[GenericType]) -> Result<(), String> {
		self.types.iter_mut().for_each(|t| { _ = t.apply_arguments(arguments); });
		Ok(())
	}
	fn convert_primitives_to_generics(&mut self, generics: &[GenericType]) {
		_ = self.types.iter_mut().map(|t| t.convert_primitives_to_generics(generics));
	}
	fn implement(&mut self, functions: HashMap<Value, Value>) -> Result<(), ()> {
    	self.implementation.implement(functions);
		Ok(())
	}
	fn find_function(&self, symbol: &Symbol, scope: &mut LexicalScope) -> Option<Value> {
		self.implementation.find_function(symbol, scope)
	}
	fn implement_trait(&mut self, trait_symbol: Symbol, functions: HashMap<Value, Value>) -> Result<(), ()> { 
		let implementation = Implementation { functions, traits: HashMap::new(), };
		self.implementation.implement_trait(trait_symbol, implementation);
		Ok(())
	}
}

impl PartialEq for SumType {
	fn eq(&self, other: &Self) -> bool {
		self.types == other.types
	}
}

impl Eq for SumType {}

impl Hash for SumType {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.types.hash(state);
	}
}

impl Display for SumType {
	fn fmt(&self, f: &mut Formatter) -> DisplayResult {
		let types: Vec<String> = self.types
			.iter()
			.map(|member| member.to_string())
			.collect();
		write!(f, "one of [{}]", types.join(", "))
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DefineSum {
	pub token: Token,
	pub symbol: Symbol,
	pub types: Vec<Expression>,
	pub generics: Vec<Symbol>
}

impl Tree for DefineSum {
	fn resolve(&self, scope: &mut crate::LexicalScope) -> Value {
		scope.add(ScopeContext::Map);
		self.define_generics(scope);
		let types: Vec<TypeKind> = self.types
			.iter()
			.filter_map(|exp| {
				if let Value::Type(mut member_type) = exp.resolve(scope) {
					member_type.convert_primitives_to_generics(&self.resolve_generics());
					Some(member_type)
				} else {
					None
				}
			})
			.collect();
		let new_sum = TypeKind::Sum(SumType { 
			types,
			implementation: Implementation::new()
		});
		scope.pop();
		if let Some(new_sum) = scope.define_type(&self.symbol, new_sum) {
			new_sum
		} else {
			self.error(String::from("unable to define type"))
		}
	}
}

impl ErrorHandler for DefineSum {
	fn get_token(&self) -> &Token {
		&self.token
	}
	fn get_context(&self) -> String {
		let types: Vec<String> = self.types.iter().map(|t| t.to_string()).collect();
		format!("defining sum type {} with types {}", self.symbol, types.join(", "))
	}
}

impl UseGenerics for DefineSum {
	fn get_generics(&self) -> &[Symbol] {
    	&self.generics
	}
}