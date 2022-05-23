use crate::*;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Declaration {
	pub type_hint: Typed<Ident>,
	pub name: Ident,
	pub value: Typed<Box<Exp>>,
	pub mutable: bool,
}

impl Analyze for Declaration {
	fn resolve_names(&mut self, scope: &mut Env) -> Result<(), Error> {
		match scope.get_symbol(&self.name) {
			Some(_) => panic!("symbol is already declared"),
			None => {
				let value = SymbolValue {
					value: None,
					type_hint: (*self.type_hint).to_owned(),
					mutable: self.mutable
				};
				scope.set_symbol(&self.name, value)
			}
		};
		(*self.value).resolve_names(scope)
	}
	fn resolve_types(&mut self, scope: &mut Env) -> Result<(), Error> {
		self.value.resolve_types(scope);
		
		// Check that type_hint exists
		let type_hint = *self.type_hint;
		let type_hint_value = get_type_or_panic(&type_hint, scope);
		self.type_hint = Typed::Typed(type_hint, type_hint_value);
		
		// Get type of value
		let value = *self.value;
		let value_type = value.type_of(scope);
		self.value = Typed::Typed(value, value_type);
		
		Ok(())
	}
	fn check_types(&mut self, scope: &mut Env) -> Result<(), Error> {
		if self.type_hint.type_value() == self.value.type_value() { 
			return Ok(()) 
		}
		panic!("type mismatch: cannot declare")
	}
}

impl TypeOf for Declaration {}