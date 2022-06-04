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
	pub fn as_type(&self) -> ASTResult<Type> {
		let value = self.value.as_ref().unwrap();
		if let Val::Type(type_hint) = &*value.val {
			return Ok(type_hint.to_owned());
		}
		value.to_err(ERR_EXPECTED_TYPE)
	}
	pub fn as_function(&self) -> ASTResult<Function> {
		let value = self.value.as_ref().unwrap();
		if let Val::Function(function) = &*value.val {
			return Ok(*function.to_owned());
		}
		value.to_err(ERR_EXPECTED_FUNCTION)
	}
	pub fn as_function_mut(&mut self) -> ASTResult<&mut Function> {
		let value = self.value.as_mut().unwrap();
		if !matches!(&*value.val, Val::Function(_)) {
			value.to_err(ERR_EXPECTED_FUNCTION)
		} else if let Val::Function(function_val) = &mut *value.val {
			Ok(function_val)
		} else {
			unreachable!()
		}
	}
	pub fn as_trait(&self) -> ASTResult<Trait> {
		let value = self.value.as_ref().unwrap();
		if let Val::Trait(trait_val) = &*value.val {
			return Ok(trait_val.to_owned());
		}
		value.to_err(ERR_EXPECTED_TRAIT)
	}
	pub fn as_trait_mut(&mut self) -> ASTResult<&mut Trait> {
		let value = self.value.as_mut().unwrap();
		if !matches!(&*value.val, Val::Trait(_)) {
			value.to_err(ERR_EXPECTED_TRAIT)
		} else if let Val::Trait(trait_val) = &mut *value.val {
			Ok(trait_val)
		} else {
			unreachable!()
		}
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