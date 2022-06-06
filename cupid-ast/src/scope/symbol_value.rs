use crate::*;

build_struct! {
	#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Tabled)]
	pub SymbolValueBuilder => pub SymbolValue<'val> {
		
		#[tabled(display_with = "fmt_option")]
		pub value: Option<BoxValue<'val>>,
		pub type_hint: Ident,
		pub mutable: bool,
	}
}

impl<'val> SymbolValueBuilder<'val> {
	pub fn from_type<T: ToOwned<Owned = T> + UseAttributes + ToIdent + Into<BoxValue<'val>>>(mut self, type_val: T) -> Self {
		self.type_hint = type_val.to_ident();
		self.value = Some(type_val.into());
		self
	}
}

impl SymbolValue<'_> {
	pub fn as_type(&self) -> ASTResult<Type> {
		let value = self.value.as_ref().unwrap();
		if let Some(type_hint) = &**value.value.as_type() {
			return Ok(type_hint.to_owned());
		}
		value.to_err(ERR_EXPECTED_TYPE)
	}
	pub fn as_function(&self) -> ASTResult<Function> {
		let value = self.value.as_ref().unwrap();
		if let Some(function) = &**value.value.as_function() {
			return Ok(function.to_owned());
		}
		value.to_err(ERR_EXPECTED_FUNCTION)
	}
	pub fn as_function_mut(&mut self) -> ASTResult<&mut Function> {
		let value = self.value.as_mut().unwrap();
		if let Some(function) = &**value.value.as_function_mut() {
			return Ok(function.to_owned());
		} else {
			value.to_err(ERR_EXPECTED_FUNCTION)
		}
	}
	pub fn as_trait(&self) -> ASTResult<Trait> {
		let value = self.value.as_ref().unwrap();
		if let Some(trait_val) = &**value.value.as_trait() {
			return Ok(trait_val.to_owned());
		}
		value.to_err(ERR_EXPECTED_TRAIT)
	}
	pub fn as_trait_mut(&mut self) -> ASTResult<&mut Trait> {
		let value = self.value.as_mut().unwrap();
		if let Some(trait_val) = &**value.value.as_trait_mut() {
			return Ok(trait_val.to_owned());
		} else {
			value.to_err(ERR_EXPECTED_TRAIT)
		}
	}
}

impl UseAttributes for SymbolValue<'_> {
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