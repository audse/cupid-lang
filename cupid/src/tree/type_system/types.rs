use crate::{Token, Value, Tree, Symbol, LexicalScope, SymbolFinder, ErrorHandler, TypeKind};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DefineType {
	pub token: Token,
	pub type_symbol: Symbol,
	pub type_value: TypeKind,
}

impl Tree for DefineType {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
		if let Some(new_type) = scope.define_type(&self.type_symbol, self.type_value.clone()) {
			new_type
		} else {
			self.error("unable to define")
		}
	}
}

impl ErrorHandler for DefineType {
	fn get_token(&self) -> &Token {
    	&self.token
	}
	fn get_context(&self) -> String {
    	format!("defining type {} with value {}", self.type_symbol, self.type_value)
	}
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DefineTypeAlias {
	pub token: Token,
	pub type_symbol: Symbol,
	pub arguments: Vec<TypeKind>,
}

impl Tree for DefineTypeAlias {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
		// if let Some(mut stored_type) = scope.get_definition(&self.arguments[0]) {
		// 	stored_type.apply_arguments(&self.arguments[0].arguments);
		// 	match scope.define_type(&self.type_symbol, stored_type) {
		// 		Ok(new_type) => Value::Type(new_type),
		// 		Err(error) => error
		// 	}
		// } else {
		// 	self.error(format!("cannot create a type alias for a non-existent type ({})", self.arguments[0]))
		// }
		Value::None
	}
}

impl ErrorHandler for DefineTypeAlias {
	fn get_token(&self) -> &Token {
		&self.token
	}
	fn get_context(&self) -> String {
		// let args: Vec<String> = self.arguments.iter().map(|a| a.to_string()).collect();
		// format!(
		// 	"defining a type alias for {} with arguments {:?}", 
		// 	self.type_symbol, 
		// 	args.join(", ")
		// )
		String::new()
	}
}