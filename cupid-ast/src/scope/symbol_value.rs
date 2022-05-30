use crate::*;

build_struct! {
	#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Tabled)]
	pub SymbolValueBuilder => pub SymbolValue {
		
		#[tabled(display_with = "fmt_option")]
		pub value: Option<Value>,
		pub type_hint: Ident,
		pub mutable: bool,
	}
}

impl SymbolValueBuilder {
	pub fn from_type<T: ToOwned<Owned = T> + UseAttributes + ToIdent + Into<Val> + Into<Value>>(mut self, type_val: T) -> Self {
		self.type_hint = type_val.to_ident();
		self.value = Some(type_val.into());
		self
	}
}

impl SymbolValue {
	pub fn as_type(&self) -> Result<Type, ASTErr> {
		let value = self.value.as_ref().unwrap();
		if let Val::Type(type_hint) = &*value.val {
			return Ok(type_hint.to_owned());
		}
		Err((value.attributes.source.unwrap(), 417))
	}
	pub fn as_function(&self) -> Result<Typed<Function>, ASTErr> {
		let value = self.value.as_ref().unwrap();
		if let Val::Function(function) = &*value.val {
			return Ok(*function.to_owned());
		}
		Err((value.attributes.source.unwrap(), 418))
	}
	pub fn as_trait(&self) -> Result<Trait, ASTErr> {
		let value = self.value.as_ref().unwrap();
		if let Val::Trait(trait_val) = &*value.val {
			return Ok(trait_val.to_owned());
		}
		Err((value.attributes.source.unwrap(), 418))
	}
	pub fn as_trait_mut(&mut self) -> Result<&mut Trait, ASTErr> {
		let value = self.value.as_mut().unwrap();
		if let Val::Trait(trait_val) = &mut *value.val {
			return Ok(trait_val);
		}
		Err((value.attributes.source.unwrap(), 418))
	}
	pub fn as_function_mut(&mut self) -> Result<&mut Typed<Function>, ASTErr> {
		let value = self.value.as_mut().unwrap();
		if let Val::Function(function_val) = &mut *value.val {
			return Ok(function_val);
		}
		Err((value.attributes.source.unwrap(), 418))
	}
}

impl UseAttributes for SymbolValue {
    fn attributes(&self) -> &Attributes {
		if let Some(value) = &self.value {
			&value.attributes
		} else {
			&self.type_hint.attributes
		}
    }
	fn attributes_mut(&mut self) -> &mut Attributes {
		if let Some(value) = &mut self.value {
			&mut value.attributes
		} else {
			&mut self.type_hint.attributes
		}
	}
}