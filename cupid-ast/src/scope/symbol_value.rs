use crate::*;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct SymbolValue {
	pub value: Option<Value>,
	pub type_hint: Ident,
	pub mutable: bool,
}

impl SymbolValue {
	pub fn as_type(&self) -> Option<Type> {
		if let Some(Value::Type(type_hint)) = &self.value {
			Some(type_hint.to_owned())
		} else {
			None
		}
	}
	pub fn as_function(&self) -> Option<Function> {
		if let Some(Value::Function(function)) = &self.value {
			Some(function.to_owned())
		} else {
			None
		}
	}
}