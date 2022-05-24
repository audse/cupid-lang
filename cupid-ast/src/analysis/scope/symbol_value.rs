use crate::*;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct SymbolValue {
	pub value: Option<Value>,
	pub type_hint: Ident,
	pub mutable: bool,
}

impl SymbolValue {
	pub fn as_type(&self) -> Result<Type, ErrCode> {
		if let Some(value) = &self.value {
			if let Val::Type(type_hint) = &*value.0 {
				return Ok(type_hint.to_owned());
			}
		}
		Err(417)
	}
	pub fn as_function(&self) -> Result<Function, ErrCode> {
		if let Some(value) = &self.value {
			if let Val::Function(function) = &*value.0 {
				return Ok(function.to_owned());
			}
		}
		Err(417)
	}
}