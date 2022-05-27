use crate::*;

#[derive(Debug, Clone, Default, Tabled)]
pub struct GenericParam {
	
	#[tabled(display_with = "fmt_option")]
	pub name: Option<Str>, 

	#[tabled(display_with = "fmt_option")]
	pub value: Option<Ident>
}

impl PartialEq for GenericParam {
	fn eq(&self, other: &Self) -> bool {
		if let (Some(name), Some(other_name)) = (&self.name, &other.name) {
			name == other_name
		} else if let (None, None) = (&self.name, &other.name) {
			self.value == other.value
		} else {
			true
		}
	}
}

impl Eq for GenericParam {}

impl GenericParam {
	pub const fn new(name: &'static str) -> Self { 
		Self { name: Some(Cow::Borrowed(name)), value: None }
	}
	pub fn apply_named(&mut self, arg: &mut TypedIdent) {
		// If the generic name matches the arg's name, sets the generic's value to the arg's value
		if let Some(param_name) = &mut self.name {
			if param_name == &mut arg.0 { 
				self.value = Some(arg.1.to_owned());
			}
		}
	}
	pub fn apply_unnamed(&mut self, arg: Ident) {
		// Sets the generic's value to the arg's value
		self.value = Some(arg);
	}
	pub fn apply(&mut self, arg: &mut GenericParam) {
		// If the generic name matches the arg's name, or either is unnamed,
		// sets the generic's value to the arg's value
		if let (Some(param_name), Some(arg_name)) = (&mut self.name, &mut arg.name) {
			if param_name == arg_name {
				self.value = arg.value.to_owned();
			}
		} else {
			self.value = arg.value.to_owned();
		}
	}
}

impl Hash for GenericParam {
	fn hash<H: Hasher>(&self, _state: &mut H) {}
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Tabled)]
pub struct GenericParams(
	#[tabled(display_with = "fmt_vec")]
	pub Vec<GenericParam>
);

impl GenericParams {
	pub fn apply(&mut self, args: GenericParams) {
		// Matches and applies generic params to arguments
		for (i, mut arg) in args.0.into_iter().enumerate() {
			if arg.name.is_some() {
				self.0.iter_mut().for_each(|param| param.apply(&mut arg));
			} else {
				self.0[i].apply(&mut arg);
			}
		}
	}
	pub fn apply_named(&mut self, args: Vec<TypedIdent>) {
		// Matches and applies type identifiers to generic params based on name
		for mut arg in args.into_iter() {
			self.0.iter_mut().for_each(|param| param.apply_named(&mut arg));
		}
	}
	pub fn apply_unnamed(&mut self, args: Vec<Ident>) {
		// Matches and applies type identifiers to generic params based on position
		for (i, arg) in args.into_iter().enumerate() {
			self.0[i].apply_unnamed(arg);
		}
	}
}

impl From<Vec<&'static str>> for GenericParams {
	fn from(names: Vec<&'static str>) -> Self {
    	Self(names.into_iter().map(GenericParam::new).collect::<Vec<GenericParam>>())
	}
}