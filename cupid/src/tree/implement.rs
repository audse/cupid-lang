use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::{Tree, Symbol, LexicalScope, Expression, Value, Declare, ErrorHandler, Token, TypeKind, SymbolFinder, UseGenerics, Type};

#[derive(Debug, Hash, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Implement {
	pub use_type: Symbol,
	pub functions: Vec<Declare>,
	pub generics: Vec<Symbol>,
	pub token: Token
}

fn make_function_map(parent: &impl UseGenerics, functions: &Vec<Declare>, scope: &mut LexicalScope) -> Result<HashMap<Value, Value>, Option<Value>> {
	let mut map: HashMap<Value, Value> = HashMap::with_capacity(functions.len());
	for function in functions {
		let symbol = &function.symbol;
		
		// get type of declaration
		let mut type_value = match function.value_type.resolve(scope) {
			Value::Error(e) => return Err(Some(Value::Error(e))),
			v => v
		};
		
		// use generics in function type signatures
		if let Err(()) = parent.convert_primitives_to_generics(&mut type_value) {
			return Err(None);
		};
		
		// get value (function body) of declaration
		let value = match function.value.resolve(scope) {
			Value::FunctionBody(a, b, c) => Value::FunctionBody(a, b, c),
			e => return Err(Some(e))
		};
		map.insert(symbol.identifier.clone(), value);
	}
	Ok(map)
}

impl Tree for Implement {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
		let use_type_value = crate::resolve_or_abort!(self.use_type, scope);
		let mut use_type = match &use_type_value {
			Value::Type(use_type) => use_type.clone(),
			_ => return self.not_type_error(&use_type_value),
		};
		
		scope.add(crate::ScopeContext::Map);
		self.define_generics(scope);
		
		let map = make_function_map(self, &self.functions, scope);
		let map = match map {
			Ok(m) => m,
			Err(Some(e)) => {
				scope.pop();
				return e;
			},
			Err(None) => {
				scope.pop();
				return self.cannot_implement_error(&use_type)
			}
		};
		
		if let Err(_) = use_type.implement(map) {
			scope.pop();
			return self.cannot_implement_error(&use_type);
		}
		
		scope.pop();
		if let Some(val) = scope.implement_type(&self.use_type, use_type) {
			val
		} else {
			self.unable_to_implement_error()
		}
	}
}

impl ErrorHandler for Implement {
	fn get_token(&self) -> &Token {
		&self.token
	}
	fn get_context(&self) -> String {
    	format!("implementing type {}", self.use_type.identifier)
	}
}

impl Implement {
	pub fn new(token: Token, use_type: Symbol, declarations: Vec<Expression>, generics: Vec<Symbol>) -> Self {
		let functions: Vec<Declare> = declarations
			.iter()
			.map(|declare| if let Expression::Declare(declare) = declare {
					declare.clone()
				} else {
					panic!("only declarations are allowed in implementations")
				})
			.collect();
		Self {
			token,
			use_type,
			functions,
			generics
		}
	}
}

impl UseGenerics for Implement {
	fn get_generics(&self) -> &[Symbol] {
    	&self.generics
	}
}

#[derive(Debug, Hash, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImplementTrait {
	pub trait_symbol: Symbol,
	pub type_symbol: Symbol,
	pub functions: Vec<Declare>,
	pub generics: Vec<Symbol>,
	pub token: Token,
}

impl Tree for ImplementTrait {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
		
		// make sure trait exists
		match crate::resolve_or_abort!(self.trait_symbol, scope) {
			Value::Trait(trait_value) => trait_value,
			x => return self.not_trait_error(&x)
		};
		
		let type_value = crate::resolve_or_abort!(self.type_symbol, scope);
		let type_value = if let Value::Type(type_value) = &type_value {
			type_value.clone()
		} else {
			return self.not_type_error(&type_value);
		};
		
		scope.add(crate::ScopeContext::Map);
		self.define_generics(scope);
		
		let functions = make_function_map(self, &self.functions, scope);
		let functions = match functions {
			Ok(m) => m,
			Err(Some(e)) => {
				scope.pop();
				return e;
			},
			Err(None) => {
				scope.pop();
				return self.cannot_implement_error(&type_value)
			}
		};
		
		scope.pop();
		if let Some(val) = scope.implement_trait(&self.type_symbol, &self.trait_symbol, functions) {
			val
		} else {
			self.unable_to_implement_error()
		}
	}
}

impl ErrorHandler for ImplementTrait {
	fn get_token(&self) -> &Token {
		&self.token
	}
	fn get_context(&self) -> String {
		format!("implementing trait {} for type {}", self.trait_symbol.identifier, self.type_symbol.identifier)
	}
}

impl ImplementTrait {
	pub fn new(token: Token, trait_symbol: Symbol, type_symbol: Symbol, declarations: Vec<Expression>, generics: Vec<Symbol>) -> Self {
		let functions: Vec<Declare> = declarations
			.iter()
			.map(|declare|
				if let Expression::Declare(declare) = declare {
					declare.clone()
				} else {
					panic!("only declarations are allowed in trait implementations")
				}
			)
			.collect();
		Self {
			token,
			trait_symbol,
			type_symbol,
			functions,
			generics
		}
	}
}

impl UseGenerics for ImplementTrait {
	fn get_generics(&self) -> &[Symbol] {
		&self.generics
	}
}

pub trait ImplementError: ErrorHandler {
	fn get_symbol(&self) -> &Symbol;
	fn not_trait_error(&self, value: &Value) -> Value {
		self.error_context(
			"expecting a trait, found a value", 
			format!("implementating {} with value {value}", self.get_symbol()).as_str()
		)
	}
	fn not_type_error(&self, value: &Value) -> Value {
		self.error_context(
			"expecting a type, found a value", 
			format!("accessing variable {} with value {value}", self.get_symbol()).as_str()
		)
	}
	fn not_function_error(&self, value: &Value) -> Value {
		self.error(format!(
			"`use` blocks can only contain functions, not {}",
			TypeKind::infer(value)
		))
	}
	fn cannot_implement_error(&self, value_type: &TypeKind) -> Value {
		return self.error_context(
			"only primitives, type aliases, and custom types can be implemented",
			format!("attempting to implement type {value_type}").as_str()
		)
	}
	fn unable_to_implement_error(&self) -> Value {
		return self.error(format!("unable to implement {}", self.get_symbol()))
	}
}

impl ImplementError for ImplementTrait {
	fn get_symbol(&self) -> &Symbol {
    	&self.trait_symbol
	}
}

impl ImplementError for Implement {
	fn get_symbol(&self) -> &Symbol {
    	&self.use_type
	}
}