use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Declaration {
	pub type_hint: Typed<Ident>,
	pub name: Ident,
	pub value: Typed<Box<Exp>>,
	pub mutable: bool,
	pub attributes: Attributes,
}

impl Analyze for Declaration {
	fn analyze_names(&mut self, scope: &mut Env) -> Result<(), (Source, ErrCode)> {
		scope.no_symbol(&self.name)?;
		
		let value = SymbolValue {
			value: None,
			type_hint: (*self.type_hint).to_owned(),
			mutable: self.mutable
		};
		
		scope.set_symbol(&self.name, value);
		self.value.analyze_names(scope)
	}
	fn analyze_types(&mut self, scope: &mut Env) -> Result<(), (Source, ErrCode)> {
		self.value.analyze_types(scope)?;
		
		self.type_hint.to_typed(self.type_hint.type_of(scope)?);
		self.value.to_typed(self.value.type_of(scope)?);
		
		Ok(())
	}
	fn check_types(&mut self, scope: &mut Env) -> Result<(), (Source, ErrCode)> {
		self.value.check_types(scope)?;
		if self.type_hint.get_type() != self.value.get_type() {
			return Err((self.attributes.source.unwrap(), ERR_TYPE_MISMATCH));
		}
		Ok(())
	}
}

impl UseAttributes for Declaration {
	fn attributes(&mut self) -> &mut Attributes { &mut self.attributes }
}

impl TypeOf for Declaration {}