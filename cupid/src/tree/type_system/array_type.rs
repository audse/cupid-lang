use std::hash::{Hash, Hasher};
use crate::{TypeKind, Type, GenericType, Tree, Value, ErrorHandler, Expression, Token};

#[derive(Debug, Clone)]
pub struct ArrayType {
	pub element_type: Box<TypeKind>,
}

impl Type for ArrayType {
	fn apply_arguments(&mut self, arguments: &[GenericType]) -> Result<(), ()> {
		if arguments.len() > 0 {
			if let Some(element_type) = TypeKind::replace_generic(&mut  *self.element_type, &arguments[0]) {
				self.element_type = element_type;
			} else {
				return Err(());
			}
		}
		Ok(())
	}
	fn convert_primitives_to_generics(&mut self, generics: &[GenericType]) {
		match &*self.element_type {
			TypeKind::Primitive(primitive) => {
				let generic_identifiers: Vec<String> = generics.iter().map(|g| g.identifier.to_string()).collect();
				if generic_identifiers.contains(&primitive.identifier.to_string()) {
					self.element_type = Box::new(TypeKind::Generic(GenericType::new(&primitive.identifier, None)));
				}
			},
			_ => ()
		}
	}
}

impl PartialEq for ArrayType {
	fn eq(&self, other: &Self) -> bool {
		match &*self.element_type {
			TypeKind::Generic(GenericType	{ identifier: _, type_value: _ }) => true,
			_ => self.element_type == other.element_type
		}
	}
}

impl Eq for ArrayType {}

impl Hash for ArrayType {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.element_type.hash(state);
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayTypeHint {
	pub token: Token,
	pub element_type: Box<Expression>,
}

impl Tree for ArrayTypeHint {
	fn resolve(&self, scope: &mut crate::LexicalScope) -> Value {
		let element_type = crate::resolve_or_abort!(self.element_type, scope);
		if let Value::Type(element_type) = element_type {
			let array_type = TypeKind::Array(ArrayType { element_type: Box::new(element_type) });
			Value::Type(array_type)
		} else {
			self.error(format!("expected a type hint for array elements, not {element_type}"))
		}
	}
}

impl ErrorHandler for ArrayTypeHint {
	fn get_token(&self) -> &Token {
    	&self.token
	}
	fn get_context(&self) -> String {
    	format!("array type with elements of type {}", self.element_type)
	}
}