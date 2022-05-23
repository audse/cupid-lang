use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SelfSignature {
	pub self_symbol: Typed<Ident>,
	pub mutable: bool,
}

impl Analyze for SelfSignature {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Signature {
	pub self_signature: Option<SelfSignature>,
	pub params: Vec<(Str, Typed<Ident>)>,
	pub generics: Vec<GenericParam>,
	pub return_type: Option<Typed<Ident>>,
}

impl Analyze for Signature {
	
	fn resolve_types(&mut self, scope: &mut Env) -> Result<(), Error> {
		// Check that all param types are valid, existing types
		for (_, param_type) in self.params.iter_mut() {
			let param_type_value = get_type_or_panic(&param_type, scope);
			param_type.to_typed(param_type_value);
		}
		// Check that return type is a valid, existing type, or set to nothing
		if let Some(return_type) = &mut self.return_type {
			let return_type_value = get_type_or_panic(&return_type, scope);
			return_type.to_typed(return_type_value);
		} else {
			self.return_type = Some(Typed::nothing());
		}
		Ok(())
	}
}