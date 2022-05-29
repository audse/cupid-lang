use crate::*;

build_struct! {
	#[derive(Debug, Clone, Default, Tabled)]
	pub TraitBuilder => pub Trait {
		pub name: Ident,
		
		#[tabled(display_with = "fmt_vec")]
		pub methods: Vec<Method>,

		#[tabled(display_with = "fmt_vec")]
		pub bounds: Vec<Ident>,
	}
}

impl Trait {
	pub fn into_ident(&self) -> Ident {
		self.name.to_owned()
	}
}

impl UseAttributes for Trait {
	fn attributes(&mut self) -> &mut Attributes { self.name.attributes() }
}

impl Analyze for Trait {
	fn analyze_scope(&mut self, scope: &mut Env) -> Result<(), (Source, ErrCode)> {
		let closure = scope.add_isolated_closure(Some(self.name.to_owned()), Context::Trait);
		scope.update_closure(&self.name, closure)?;
		scope.use_closure(closure);
		self.attributes().closure = closure;

		self.name.analyze_scope(scope)?;
		for method in self.methods.iter_mut() {
			method.attributes().closure = closure;

			method.analyze_scope(scope)?;
		}
		scope.reset_closure();
		Ok(())
	}
	fn analyze_names(&mut self, scope: &mut Env) -> Result<(), (Source, ErrCode)> {
		scope.use_closure(self.attributes().closure);

		self.name.analyze_names(scope)?;
		for method in self.methods.iter_mut() {
			method.analyze_names(scope)?;
		}

		scope.reset_closure();
		Ok(())
	}
	fn analyze_types(&mut self, scope: &mut Env) -> Result<(), (Source, ErrCode)> {
		scope.use_closure(self.attributes().closure);

		for method in self.methods.iter_mut() {
			method.analyze_types(scope)?;
		}

		scope.reset_closure();
		Ok(())
	}
	fn check_types(&mut self, scope: &mut Env) -> Result<(), (Source, ErrCode)> {
		scope.use_closure(self.attributes().closure);

		for method in self.methods.iter_mut() {
			method.check_types(scope)?;
		}

		scope.reset_closure();
		Ok(())
	}
}

impl ToIdent for Trait { 
	fn to_ident(&self) -> Ident { self.name.to_owned() } 
}

impl From<Trait> for Val { 
	fn from(t: Trait) -> Val { Val::Trait(t) } 
}

impl PartialEq for Trait { 
	fn eq(&self, other: &Self) -> bool { self.name == other.name } 
}

impl Eq for Trait {}

impl Hash for Trait {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.name.hash(state);
	}
}

impl From<Trait> for Value {
	fn from(mut t: Trait) -> Self {
		Value::build()
			.attributes(t.attributes().to_owned())
			.val(IsTyped(t.into(), TRAIT.to_owned()))
			.build()
	}
}