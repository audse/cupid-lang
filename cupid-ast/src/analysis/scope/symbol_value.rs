use crate::*;

build_struct! {
	#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
	pub SymbolValueBuilder => pub SymbolValue {
		pub value: Option<Value>,
		pub type_hint: Ident,
		pub mutable: bool,
	}
}

impl SymbolValue {
	pub fn as_type(&self) -> Result<Type, (Source, ErrCode)> {
		if let Some(value) = &self.value {
			if let Val::Type(type_hint) = &*value.val {
				return Ok(type_hint.to_owned());
			}
			Err((self.value.as_ref().unwrap().attributes.source.unwrap(), 417))
		} else {
			Err((0, 404))
		}
	}
	pub fn as_function(&self) -> Result<Function, (Source, ErrCode)> {
		if let Some(value) = &self.value {
			if let Val::Function(function) = &*value.val {
				return Ok(function.to_owned());
			}
		}
		Err((self.value.as_ref().unwrap().attributes.source.unwrap(), 418))
	}
}