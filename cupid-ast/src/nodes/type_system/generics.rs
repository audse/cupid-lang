use crate::*;

build_struct! {
	#[derive(Debug, Clone, Default, Tabled)]
	pub GenericBuilder => pub Generic {
		
		#[tabled(display_with = "fmt_option")]
		pub ident: Option<Str>, 

		#[tabled(display_with = "fmt_option")]
		pub arg: Option<Ident>
	}
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Tabled)]
pub struct GenericList(
	#[tabled(display_with = "fmt_vec")]
	pub Vec<Generic>
);

impl PartialEq for Generic {
	fn eq(&self, other: &Self) -> bool {
		if let (Some(name), Some(other_name)) = (&self.ident, &other.ident) {
			name == other_name
		} else if let (None, None) = (&self.ident, &other.ident) {
			self.arg == other.arg
		} else {
			true
		}
	}
}

impl Eq for Generic {}

impl Generic {
	pub fn apply_named(&mut self, arg: &mut TypedIdent) {
		// If the generic name matches the arg's name, sets the generic's value to the arg's value
		if let Some(param_name) = &mut self.ident {
			if param_name == &mut arg.0 { 
				self.arg = Some(arg.1.to_owned());
			}
		}
	}
	pub fn apply_unnamed(&mut self, arg: Ident) {
		// Sets the generic's value to the arg's value
		self.arg = Some(arg);
	}
	pub fn apply(&mut self, arg: &mut Generic) {
		// If the generic name matches the arg's name, or either is unnamed,
		// sets the generic's value to the arg's value
		if let (Some(param_name), Some(arg_name)) = (&mut self.ident, &mut arg.ident) {
			if param_name == arg_name {
				self.arg = arg.arg.to_owned();
			}
		} else {
			self.arg = arg.arg.to_owned();
		}
	}
	pub fn set_symbol(&self, scope: &mut Env) -> Result<(), (Source, ErrCode)> {
		if let Some(name) = &self.ident {
			let type_value = invert(self.arg.to_owned().map(|t| t.type_of(scope)))?
				.map(|t| Value { 
					val: Untyped(Val::Type(t)), 
					..Default::default() 
				});
			let value = SymbolValue::build()
				.value(type_value)
				.build();
			scope.set_symbol(&Ident { name: name.to_owned(), ..Default::default() }, value);
		}
		Ok(())
	}
}

impl Hash for Generic {
	fn hash<H: Hasher>(&self, _state: &mut H) {}
}

impl GenericList {
	pub fn apply(&mut self, args: GenericList) {
		// Matches and applies generic params to arguments
		for (i, mut arg) in args.0.into_iter().enumerate() {
			if arg.ident.is_some() {
				self.iter_mut().for_each(|param| param.apply(&mut arg));
			} else {
				self.0[i].apply(&mut arg);
			}
		}
	}
	pub fn apply_named(&mut self, args: Vec<TypedIdent>) {
		// Matches and applies type idents to generic params based on name
		for mut arg in args.into_iter() {
			self.iter_mut().for_each(|param| param.apply_named(&mut arg));
		}
	}
	pub fn apply_unnamed(&mut self, args: Vec<Ident>) {
		// Matches and applies type idents to generic params based on position
		for (i, arg) in args.into_iter().enumerate() {
			self.0[i].apply_unnamed(arg);
		}
	}
	pub fn set_symbols(&self, scope: &mut Env) -> Result<(), (Source, ErrCode)> {
		for symbol in self.iter() {
			symbol.set_symbol(scope)?;
		}
		Ok(())
	}
}

impl From<Vec<&'static str>> for GenericList {
	fn from(names: Vec<&'static str>) -> Self {
    	Self(names.into_iter().map(|n| Generic::build().new_str(n).build()).collect::<Vec<Generic>>())
	}
}

impl std::ops::Deref for GenericList {
	type Target = Vec<Generic>;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl std::ops::DerefMut for GenericList {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}