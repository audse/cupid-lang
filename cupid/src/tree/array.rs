use crate::{Expression, Value, Tree, LexicalScope, ErrorHandler, Token, Type};

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Array {
	pub items: Vec<Expression>
}

impl Tree for Array {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
		let items: Vec<Box<Value>> = self.items
			.iter()
			.map(|i| Box::new(i.resolve(scope)))
			.collect();
		Value::Array(items)
	}
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct ArrayAccess {
	pub array: Box<Expression>,
	pub term: Box<Expression>,
	pub token: Token,
}

impl Tree for ArrayAccess {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
		let array = crate::resolve_or_abort!(self.array, scope);
		let term = crate::resolve_or_abort!(self.term, scope);
		match (term, array) {
			(Value::Integer(i), Value::Array(a)) => *(a[i as usize]).clone(),
			(i, Value::Array(_)) => self.bad_access_error(&i),
			(i, a) => self.not_array_error(&a, &i),
		}
	}
}

impl ErrorHandler for ArrayAccess {
	fn get_token(&self) -> &crate::Token {
    	&self.token
	}
	fn get_context(&self) -> String {
    	format!("Accessing property {} of array {}", self.term, self.array)
	}
}

impl ArrayAccess {
	fn bad_access_error(&self, accessor: &Value) -> Value {
		self.error(format!(
			"type mismatch: array items can only be accessed with integers, not {} ({})", 
			Type::from(accessor), 
			accessor
		))
	}
	fn not_array_error(&self, array: &Value, accessor: &Value) -> Value {
		self.error(format!(
			"type mismatch: {} ({}) is not an array, and cannot be accessed by {} ({})",
			array,
			Type::from(array),
			accessor,
			Type::from(accessor)
		))
	}
}