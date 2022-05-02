use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::{Tree, Symbol, LexicalScope, Value, Declare, Expression, ErrorHandler, Token, TypeKind, SymbolFinder, UseGenerics, Type};

#[derive(Debug, Hash, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DefineTrait {
	pub symbol: Symbol,
	pub functions: Vec<Declare>,
	pub generics: Vec<Symbol>,
	pub token: Token
}

impl Tree for DefineTrait {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
    	
		scope.add(crate::ScopeContext::Map);
		self.define_generics(scope);
		
		let mut map: HashMap<Value, Value> = HashMap::new();
		for function in &self.functions {
			let symbol = &function.symbol;
			
			let mut type_value = crate::resolve_or_abort!(&function.value_type, scope, { scope.pop(); });
			if let Err(()) = self.use_generics_in_function(&mut type_value) {
				scope.pop();
				return self.not_function_error(&type_value);
			}
			
			let value = crate::resolve_or_abort!(&function.value, scope, { scope.pop(); });
			
			if let Value::FunctionBody(..) = value {
				map.insert(symbol.identifier.clone(), value);
			} else if let Value::None = value {
				map.insert(symbol.identifier.clone(), value);
			} else {
				scope.pop();
				return self.not_function_error(&type_value);
			}
		}
		
		scope.pop();
		if let Some(new_trait) = scope.define_trait(&self.symbol, map) {
			new_trait
		} else {
			self.error("unable to define trait")
		}
	}
}

impl DefineTrait {
	pub fn new(token: Token, symbol: Symbol, declarations: Vec<Expression>, generics: Vec<Symbol>) -> Self {
		let functions: Vec<Declare> = declarations
			.iter()
			.map(|declare|
				if let Expression::Declare(declare) = declare {
					declare.clone()
				} else {
					panic!("only declarations are allowed in implementations")
				}
			)
			.collect();
		Self {
			token,
			symbol,
			functions,
			generics
		}
	}
	fn use_generics_in_function(&self, type_value: &mut Value) -> Result<(), ()> {
		match *type_value {
			Value::Type(ref mut type_kind) => match type_kind {
				TypeKind::Function(ref mut t) => {
					let generics = self.resolve_generics();
					t.convert_primitives_to_generics(&generics);
					return Ok(())
				},
				_ => return Err(())
			},
			_ => return Err(())
		};
	}
	fn not_function_error(&self, value: &Value) -> Value {
		self.error(format!(
			"trait blocks can only contain functions or function signatures, not {}",
			TypeKind::infer(value)
		))
	}
}

impl UseGenerics for DefineTrait {
	fn get_generics(&self) -> &[Symbol] {
    	&self.generics
	}
}

impl ErrorHandler for DefineTrait {
	fn get_token(&self) -> &Token {
    	&self.token
	}
	fn get_context(&self) -> String {
		let functions: Vec<String> = self.functions
			.iter()
			.map(|f| format!("{:?}", f))
			.collect();
    	format!("defining trait {} with functions: {}", self.symbol, functions.join(", "))
	}
}