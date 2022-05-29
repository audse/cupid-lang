use crate::*;

build_struct! {
	#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Tabled)]
	pub DeclarationBuilder => pub Declaration {
		pub type_hint: Typed<Ident>,
		pub name: Ident,
		pub value: Typed<Box<Exp>>,
		
        #[tabled(skip)]
		pub mutable: bool,

        #[tabled(skip)]
		pub attributes: Attributes,
	}
}

impl Analyze for Declaration {
	fn analyze_scope(&mut self, scope: &mut Env) -> Result<(), (Source, ErrCode)> {
		self.type_hint.analyze_scope(scope)?;
		self.value.analyze_scope(scope)?;
		Ok(())
	}
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
		let (expected, found) = (self.type_hint.get_type(), self.value.get_type());
		if expected != found {
			scope.traceback.push(format!("Expected type\n{expected}, found type\n{found}"));
			return Err((self.attributes.source.unwrap(), ERR_TYPE_MISMATCH));
		}
		Ok(())
	}
}

impl UseAttributes for Declaration {
	fn attributes(&mut self) -> &mut Attributes { &mut self.attributes }
}

impl TypeOf for Declaration {}