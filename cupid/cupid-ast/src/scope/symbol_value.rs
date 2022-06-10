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

impl SymbolValue {
	pub fn as_type(&self) -> ASTResult<Type> {
		let value = self.value.as_ref().ok_or_else(|| self.as_err(ERR_EXPECTED_TYPE))?;
		match value {
			VType(type_val) => Ok(type_val.to_owned()),
			x => x.to_err(ERR_EXPECTED_TYPE)
		}
	}
	pub fn as_function(&self) -> ASTResult<Function> {
		let value = self.value.as_ref().ok_or_else(|| self.as_err(ERR_EXPECTED_FUNCTION))?;
		match value {
			VFunction(function) => Ok(*function.to_owned()),
			x => x.to_err(ERR_EXPECTED_FUNCTION)
		}
	}
	pub fn as_function_mut(&mut self) -> ASTResult<&mut Function> {
		let value = self.value.as_mut().unwrap();
		match value {
			VFunction(function) => Ok(&mut *function),
			x => x.to_err(ERR_EXPECTED_FUNCTION)
		}
	}
	pub fn as_trait(&self) -> ASTResult<Trait> {
		let value = self.value.as_ref().ok_or_else(|| self.as_err(ERR_EXPECTED_TRAIT))?;
		match value {
			VTrait(trait_val) => Ok(trait_val.to_owned()),
			x => x.to_err(ERR_EXPECTED_TRAIT)
		}
	}
	pub fn as_trait_mut(&mut self) -> ASTResult<&mut Trait> {
		let value = self.value.as_mut().unwrap();
		match value {
			VTrait(trait_val) => Ok(trait_val),
			x => x.to_err(ERR_EXPECTED_TRAIT)
		}
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