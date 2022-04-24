use std::hash::{Hash, Hasher};
use crate::{TypeKind, Type, GenericType, Tree, Value, Expression, Token, ErrorHandler};

#[derive(Debug, Clone)]
pub struct FunctionType {
	pub return_type: Box<TypeKind>,
}

impl Type for FunctionType {
	fn apply_arguments(&mut self, arguments: &[GenericType]) -> Result<(), ()> {
		if arguments.len() > 0 {
			if let Some(return_type) = TypeKind::replace_generic(&mut *self.return_type, &arguments[0]) {
				self.return_type = return_type;
			} else {
				return Err(());
			}
		}
		Ok(())
	}
	fn convert_primitives_to_generics(&mut self, generics: &[GenericType]) {
		match &*self.return_type {
			TypeKind::Primitive(primitive) => {
				let generic_identifiers: Vec<String> = generics.iter().map(|g| g.identifier.to_string()).collect();
				if generic_identifiers.contains(&primitive.identifier.to_string()) {
					self.return_type = Box::new(TypeKind::Generic(GenericType::new(&primitive.identifier, None)));
				}
			},
			_ => ()
		}
	}
}

impl PartialEq for FunctionType {
	fn eq(&self, other: &Self) -> bool {
		match &*self.return_type {
			TypeKind::Generic(GenericType	{ identifier: _, type_value: _ }) => true,
			_ => self.return_type == other.return_type
		}
	}
}

impl Eq for FunctionType {}

impl Hash for FunctionType {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.return_type.hash(state);
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionTypeHint {
	pub token: Token,
	pub return_type: Box<Expression>,
}

impl Tree for FunctionTypeHint {
	fn resolve(&self, scope: &mut crate::LexicalScope) -> Value {
		let return_type = crate::resolve_or_abort!(self.return_type, scope);
		if let Value::Type(return_type) = return_type {
			let function_type = TypeKind::Function(FunctionType { return_type: Box::new(return_type) });
			Value::Type(function_type)
		} else {
			self.error(format!("expected a type hint for function return value, not {return_type}"))
		}
	}
}

impl ErrorHandler for FunctionTypeHint {
	fn get_token(&self) -> &Token {
		&self.token
	}
	fn get_context(&self) -> String {
		format!("function type with return value of type {}", self.return_type)
	}
}