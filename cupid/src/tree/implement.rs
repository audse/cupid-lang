use std::collections::HashMap;
use crate::{Tree, Symbol, LexicalScope, Expression, Value, Declare, ErrorHandler, Token, TypeKind, SymbolFinder};

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Implement {
	pub use_type: Symbol,
	pub functions: Vec<Declare>,
	pub token: Token
}

impl Tree for Implement {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
		let use_type_value = crate::resolve_or_abort!(self.use_type, scope);
		let mut use_type = if let Value::Type(use_type) = &use_type_value {
			use_type.clone()
		} else {
			return self.not_type_error(&use_type_value);
		};
		let mut map: HashMap<Value, Value> = HashMap::new();
		for function in &self.functions {
			let symbol = &function.symbol.identifier;
			let type_value = crate::resolve_or_abort!(&function.value_type, scope);
			let value = crate::resolve_or_abort!(&function.value, scope);
			if Implement::is_function(&type_value, &value) {
				map.insert(symbol.clone(), value);
			} else {
				return self.not_function_error(&type_value);
			}
		}
		match use_type {
			TypeKind::Primitive(ref mut p) => {
				p.implement = map;
			},
			TypeKind::Alias(ref mut a) => {
				a.implement = map;
			},
			TypeKind::Struct(ref mut s) => {
				s.implement = map;
			},
			x => return self.error_context(
				"only primitives, type aliases, and custom types can be implemented",
				format!("attempting to implement type {x}").as_str()
			)
		};
		if let Some(val) = scope.implement_type(&self.use_type, use_type) {
			val
		} else {
			self.error(format!("problem implementing type {use_type_value}"))
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
	pub fn new(token: Token, use_type: Symbol, declarations: Vec<Expression>) -> Self {
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
			use_type,
			functions
		}
	}
	fn not_type_error(&self, value: &Value) -> Value {
		self.error_context(
			"expecting a type, found a value", 
			format!("accessing variable {} with value {value}", self.use_type).as_str()
		)
	}
	fn not_function_error(&self, value: &Value) -> Value {
		self.error(format!(
			"type `use` blocks can only contain functions, not {}",
			TypeKind::infer(value)
		))
	}
	fn is_function(type_value: &Value, value: &Value) -> bool {
		match &type_value {
			Value::Type(type_kind) => match &type_kind {
				TypeKind::Function(_) => (),
				_ => return false
			},
			_ => ()
		};
		match TypeKind::infer(value) {
			TypeKind::Function(_) => (),
			_ => return false
		};
		true
	}
}