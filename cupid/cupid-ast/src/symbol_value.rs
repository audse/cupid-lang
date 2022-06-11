use crate::*;

build_struct! {
	#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
	pub SymbolValueBuilder => pub SymbolValue {
		pub value: Option<Value>,
		pub type_hint: Ident,
		pub mutable: bool,
	}
}

impl UseAttributes for SymbolValue {
    fn attributes(&self) -> &Attributes {
		if let Some(value) = &self.value {
			value.attributes()
		} else {
			&self.type_hint.attributes
		}
    }
	fn attributes_mut(&mut self) -> &mut Attributes {
		if let Some(value) = &mut self.value {
			value.attributes_mut()
		} else {
			&mut self.type_hint.attributes
		}
	}
}