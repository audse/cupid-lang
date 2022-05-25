use crate::*;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct SymbolValue {
	pub value: Option<Value>,
	pub type_hint: Ident,
	pub mutable: bool,
}

impl SymbolValue {
	pub fn as_type(&self) -> Result<Type, (Source, ErrCode)> {
		if let Some(value) = &self.value {
			if let Val::Type(type_hint) = &*value.0 {
				return Ok(type_hint.to_owned());
			}
			Err((self.value.as_ref().unwrap().1.source.unwrap(), 417))
		} else {
			Err((0, 404))
		}
	}
	pub fn as_function(&self) -> Result<Function, (Source, ErrCode)> {
		if let Some(value) = &self.value {
			if let Val::Function(function) = &*value.0 {
				return Ok(function.to_owned());
			}
		}
		Err((self.value.as_ref().unwrap().1.source.unwrap(), 418))
	}
}