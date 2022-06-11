use crate::*;

pub trait AsValue {
	fn as_type(&self) -> ASTResult<Type>;
	fn as_function(&self) -> ASTResult<Function>;
	fn as_function_mut(&mut self) -> ASTResult<&mut Function>;
	fn as_trait(&self) -> ASTResult<Trait>;
	fn as_trait_mut(&mut self) -> ASTResult<&mut Trait>;
}

impl AsValue for SymbolValue {
	fn as_type(&self) -> ASTResult<Type> {
		let value = self.value.as_ref().ok_or_else(|| self.as_err(ERR_EXPECTED_TYPE))?;
		match value {
			VType(type_val) => Ok(type_val.to_owned()),
			x => x.to_err(ERR_EXPECTED_TYPE)
		}
	}
	fn as_function(&self) -> ASTResult<Function> {
		let value = self.value.as_ref().ok_or_else(|| self.as_err(ERR_EXPECTED_FUNCTION))?;
		match value {
			VFunction(function) => Ok(*function.to_owned()),
			x => x.to_err(ERR_EXPECTED_FUNCTION)
		}
	}
	fn as_function_mut(&mut self) -> ASTResult<&mut Function> {
		let value = self.value.as_mut().unwrap();
		match value {
			VFunction(function) => Ok(&mut *function),
			x => x.to_err(ERR_EXPECTED_FUNCTION)
		}
	}
	fn as_trait(&self) -> ASTResult<Trait> {
		let value = self.value.as_ref().ok_or_else(|| self.as_err(ERR_EXPECTED_TRAIT))?;
		match value {
			VTrait(trait_val) => Ok(trait_val.to_owned()),
			x => x.to_err(ERR_EXPECTED_TRAIT)
		}
	}
	fn as_trait_mut(&mut self) -> ASTResult<&mut Trait> {
		let value = self.value.as_mut().unwrap();
		match value {
			VTrait(trait_val) => Ok(trait_val),
			x => x.to_err(ERR_EXPECTED_TRAIT)
		}
	}
}